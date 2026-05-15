#!/usr/bin/env python3
"""
Translate m68000-sdf.cpp handlers to Rust.
Strategy: preprocess C++ to remove MAME cruft, then do line-level translation
with proper brace tracking for goto→loop/match conversion.
"""

import re, os

INPUT = "m68000/m68000-sdf.cpp"
OUTPUT_DIR = "src/handlers"


def parse_handlers(path):
    with open(path) as f:
        content = f.read()
    handlers = []
    for m in re.finditer(r'void m68000_device::(\w+_df)\(\)[^\n]*\n\{', content):
        name, body_start = m.group(1), m.end()
        depth, pos = 1, body_start
        while depth > 0 and pos < len(content):
            if content[pos] == '{': depth += 1
            elif content[pos] == '}': depth -= 1
            pos += 1
        handlers.append((name[:-3], content[body_start:pos-1]))
    return handlers


def preprocess(body):
    """Strip MAME scheduler blocks and callbacks from body text."""
    lines = body.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        # Strip icount check blocks
        if 'if(m_icount <= 0)' in line or 'if(m_icount <= 0 && access_to_be_redone()' in line:
            depth = line.count('{') - line.count('}')
            i += 1
            while i < len(lines) and depth > 0:
                depth += lines[i].count('{') - lines[i].count('}')
                i += 1
            continue
        # Strip callbacks
        if any(x in line for x in ['debugger_exception_hook', 'debugger_wait_hook',
                                     'm_cmpild_instr_callback', 'm_rte_instr_callback',
                                     'm_reset_cb']):
            i += 1
            continue
        # Strip TAS callback block (if + body)
        if 'm_tas_write_callback' in line:
            # skip this line and possibly next (the callback call)
            i += 1
            while i < len(lines) and lines[i].strip().startswith('m_tas_write_callback'):
                i += 1
            continue
        # Strip icount += (leftover from stripped blocks)
        if re.match(r'\s*m_icount \+= \d+;', line):
            i += 1
            continue
        if 'm_inst_substate' in line:
            i += 1
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)


def translate_handler(name, body):
    """Translate a preprocessed handler body to Rust."""
    body = preprocess(body)
    lines = body.split('\n')

    # Extract rx/ry declarations
    preamble = []
    for line in lines:
        if 'int rx = map_sp(((m_irdi >> 9) & 7) | 8)' in line:
            preamble.append('let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);')
        elif 'int rx = (m_irdi >> 9) & 7' in line:
            preamble.append('let rx = ((self.irdi >> 9) & 7) as usize;')
        elif 'int ry = map_sp((m_irdi & 7) | 8)' in line:
            preamble.append('let ry = self.map_sp(((self.irdi & 7) | 8) as usize);')
        elif 'int ry = m_irdi & 7' in line:
            preamble.append('let ry = (self.irdi & 7) as usize;')

    if 'goto ' in body:
        return emit_goto_handler(lines, preamble)
    else:
        return emit_linear_handler(lines, preamble)


def emit_linear_handler(lines, preamble):
    """Emit a handler without gotos."""
    out = ['        ' + p for p in preamble]
    i = 0
    while i < len(lines):
        i, stmts = translate_lines(lines, i)
        for s in stmts:
            out.append('        ' + s)
    return '\n'.join(out)


def emit_goto_handler(lines, preamble):
    """Emit a handler with gotos using loop+match."""
    # Find labels
    labels = {}
    state_num = 1
    for line in lines:
        m = re.match(r'^(\w+):\s*$', line.strip())
        if m and m.group(1) not in labels:
            labels[m.group(1)] = state_num
            state_num += 1

    # Split into blocks
    blocks = []  # (state_id, [lines])
    current_lines = []
    current_state = 0
    for line in lines:
        m = re.match(r'^(\w+):\s*$', line.strip())
        if m:
            blocks.append((current_state, current_lines))
            current_state = labels[m.group(1)]
            current_lines = []
        else:
            current_lines.append(line)
    blocks.append((current_state, current_lines))

    out = ['        ' + p for p in preamble]
    out.append('        let mut __state = 0u32;')
    out.append('        loop { match __state {')

    for block_idx, (state_id, block_lines) in enumerate(blocks):
        out.append(f'        {state_id} => {{')
        i = 0
        has_exit = False
        while i < len(block_lines):
            i, stmts = translate_lines_goto(block_lines, i, labels)
            for s in stmts:
                out.append('            ' + s)
                if 'return;' in s or 'continue;' in s:
                    has_exit = True
        # Fallthrough to next block
        if not has_exit:
            if block_idx + 1 < len(blocks):
                next_state = blocks[block_idx + 1][0]
                out.append(f'            __state = {next_state}; continue;')
            else:
                out.append('            return;')
        out.append('        }')

    out.append('        _ => return,')
    out.append('        }} // end match+loop')
    return '\n'.join(out)


def translate_lines(lines, i):
    """Translate lines starting at i, return (new_i, [rust_statements])."""
    line = lines[i]
    s = line.strip()
    i += 1

    if not s or s.startswith('//'):
        return i, []
    if s.startswith('int rx') or s.startswith('int ry'):
        return i, []

    # Two-line patterns: if(cond)\n\t\tstmt;
    if s == 'if(m_sr & SR_T)' and i < len(lines) and 'm_next_state = S_TRACE' in lines[i]:
        i += 1
        return i, ['if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }']
    if s == 'if(m_next_state == S_TRACE)' and i < len(lines) and 'm_next_state = 0' in lines[i]:
        i += 1
        return i, ['if self.next_state == S_TRACE as u32 { self.next_state = 0; }']
    if s == 'if(m_next_state != S_TRACE) m_next_state = m_int_next_state;':
        return i, ['if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }']
    if s == 'if(!(m_aob & 1))' and i < len(lines) and 'm_edb >>= 8' in lines[i]:
        i += 1
        return i, ['if self.aob & 1 == 0 { self.edb >>= 8; }']

    # if block with braces
    if s == 'if(m_aob & 1) {':
        stmts = ['if self.aob & 1 != 0 {']
        while i < len(lines):
            ls = lines[i].strip()
            i += 1
            if ls == '}':
                stmts.append('}')
                break
            t = translate_stmt(ls)
            if t: stmts.append('    ' + t)
        return i, stmts

    if s == 'if(!(m_sr & SR_S)) {':
        stmts = ['if self.sr & SR_S == 0 {']
        while i < len(lines):
            ls = lines[i].strip()
            i += 1
            if ls == '}':
                stmts.append('}')
                break
            t = translate_stmt(ls)
            if t: stmts.append('    ' + t)
        return i, stmts

    # Single statement
    t = translate_stmt(s)
    if t:
        return i, [t]
    return i, []


def translate_lines_goto(lines, i, labels):
    """Like translate_lines but handles goto and if/goto patterns."""
    line = lines[i]
    s = line.strip()
    i += 1

    if not s or s.startswith('//'):
        return i, []
    if s.startswith('int rx') or s.startswith('int ry'):
        return i, []

    # goto
    gm = re.match(r'goto (\w+);', s)
    if gm:
        target = labels.get(gm.group(1), 0)
        return i, [f'__state = {target}; continue;']

    # if(cond) goto X; (single line)
    gm = re.match(r'if\((.+)\)\s+goto (\w+);', s)
    if gm:
        cond = translate_cond(gm.group(1))
        target = labels.get(gm.group(2), 0)
        return i, [f'if {cond} {{ __state = {target}; continue; }}']

    # if(cond)\n\tgoto X;\nelse\n\tgoto Y;
    if re.match(r'if\(.+\)\s*$', s):
        cond_match = re.match(r'if\((.+)\)\s*$', s)
        cond = translate_cond(cond_match.group(1))
        # Next line should be goto
        if i < len(lines) and 'goto ' in lines[i]:
            gm = re.match(r'\s*goto (\w+);', lines[i])
            if gm:
                target_t = labels.get(gm.group(1), 0)
                i += 1
                # Check for else + goto
                if i < len(lines) and lines[i].strip() == 'else':
                    i += 1
                    if i < len(lines):
                        gm2 = re.match(r'\s*goto (\w+);', lines[i])
                        if gm2:
                            target_f = labels.get(gm2.group(1), 0)
                            i += 1
                            return i, [f'if {cond} {{ __state = {target_t}; continue; }} else {{ __state = {target_f}; continue; }}']
                return i, [f'if {cond} {{ __state = {target_t}; continue; }}']

    # Two-line patterns
    if s == 'if(m_sr & SR_T)' and i < len(lines) and 'm_next_state = S_TRACE' in lines[i]:
        i += 1
        return i, ['if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }']
    if s == 'if(m_next_state == S_TRACE)' and i < len(lines) and 'm_next_state = 0' in lines[i]:
        i += 1
        return i, ['if self.next_state == S_TRACE as u32 { self.next_state = 0; }']
    if s == 'if(m_next_state != S_TRACE) m_next_state = m_int_next_state;':
        return i, ['if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }']
    if s == 'if(!(m_aob & 1))' and i < len(lines) and 'm_edb >>= 8' in lines[i]:
        i += 1
        return i, ['if self.aob & 1 == 0 { self.edb >>= 8; }']

    # if block with braces
    if s == 'if(m_aob & 1) {':
        stmts = ['if self.aob & 1 != 0 {']
        while i < len(lines):
            ls = lines[i].strip()
            i += 1
            if ls == '}':
                stmts.append('}')
                break
            t = translate_stmt(ls)
            if t: stmts.append('    ' + t)
        return i, stmts

    if s == 'if(!(m_sr & SR_S)) {':
        stmts = ['if self.sr & SR_S == 0 {']
        while i < len(lines):
            ls = lines[i].strip()
            i += 1
            if ls == '}':
                stmts.append('}')
                break
            t = translate_stmt(ls)
            if t: stmts.append('    ' + t)
        return i, stmts

    # Single statement
    t = translate_stmt(s)
    if t:
        return i, [t]
    return i, []


def translate_cond(cond):
    """Translate a C++ condition expression to Rust."""
    cond = cond.strip()
    if cond == 'm_t':
        return 'self.t != 0'
    if cond == '!m_t':
        return 'self.t == 0'
    if cond == 'm_t == 0':
        return 'self.t == 0'
    if cond == 'm_t == 1':
        return 'self.t == 1'
    if cond == 'm_t == 2':
        return 'self.t == 2'
    return subst_all(cond) + ' != 0'


def translate_stmt(s):
    """Translate a single C++ statement to Rust."""
    if not s or s == '}' or s == '{':
        return None

    # icount
    m = re.match(r'm_icount -= (\d+);', s)
    if m:
        return f'self.icount -= {m.group(1)};'

    if s == 'return;':
        return 'return;'

    # Bus reads
    if s == 'm_edb = m_opcodes.read_interruptible(m_aob & ~1);':
        return 'self.edb = bus.read_program(self.aob & !1);'
    if 'm_edb = m_opcodes.read_interruptible(m_aob & ~1,' in s:
        return 'self.edb = bus.read_program(self.aob & !1);'
    if s == 'm_edb = m_program.read_interruptible(m_aob & ~1);':
        return 'self.edb = bus.read_data(self.aob & !1);'
    if 'm_edb = m_program.read_interruptible(m_aob & ~1,' in s:
        return 'self.edb = bus.read_data(self.aob & !1);'
    if s == 'm_edb = m_cpu_space.read_interruptible(m_aob);':
        return 'self.edb = bus.read_cpu(self.aob);'

    # Bus writes
    if 'm_program.write_interruptible(m_aob & ~1, m_dbout' in s:
        return 'bus.write_data(self.aob & !1, self.dbout);'

    # Function calls
    for fn in ['set_ftu_const', 'step_movem', 'step_movem_predec',
               'update_user_super', 'update_interrupt',
               'start_interrupt_vector_lookup', 'end_interrupt_vector_lookup',
               'sr_z', 'sr_nz_u', 'sr_nzvc', 'sr_nzvc_u', 'sr_xnzvc', 'sr_xnzvc_u']:
        if s == f'{fn}();':
            return f'self.{fn}();'

    # ALU calls
    m = re.match(r'(alu_\w+)\((.+)\);', s)
    if m:
        fn, args_str = m.group(1), m.group(2)
        args = split_args(args_str)
        # ALU functions take u16 (or u8 for 8-bit variants)
        is_8bit = fn.endswith('8') or fn in ['alu_abcd8', 'alu_sbcd8']
        is_32bit = '32' in fn
        rargs = ', '.join(cast_alu_arg(subst_all(a.strip()), fn) for a in args)
        return f'self.{fn}({rargs});'

    # set_16h/set_16l
    m = re.match(r'(set_16[hl])\((.+)\);', s)
    if m:
        fn, args_str = m.group(1), m.group(2)
        args = split_args(args_str)
        target = subst_all(args[0].strip())
        value = subst_all(args[1].strip())
        rust_fn = 'set_16h' if fn == 'set_16h' else 'set_16l_32'
        return f'M68000::{rust_fn}(&mut {target}, {value});'

    # set_8*
    m = re.match(r'(set_8\w*)\((.+)\);', s)
    if m:
        fn, args_str = m.group(1), m.group(2)
        args = split_args(args_str)
        target = subst_all(args[0].strip())
        value = subst_all(args[1].strip())
        rust_fn = fn if fn != 'set_8' else 'set_8_32'
        return f'M68000::{rust_fn}(&mut {target}, {value});'

    # Chained: m_sr = m_isr = expr;
    m = re.match(r'm_sr = m_isr = (.+);', s)
    if m:
        rhs = subst_all(m.group(1))
        return f'self.isr = {rhs}; self.sr = self.isr;'

    # --- Special ternary patterns (handle before generic assignment) ---
    if s == 'm_inst_state = m_next_state ? m_next_state : m_decode_table[m_ird];':
        return 'self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };'

    if s == 'm_ssw = m_base_ssw | (m_sr & SR_S ? SSW_S : 0);':
        return 'self.ssw = self.base_ssw | (if self.sr & SR_S != 0 { SSW_S } else { 0 });'

    m = re.match(r'm_au = m_da\[(rx|ry)\] \+ \(\1 < 15 \? 1 : 2\);', s)
    if m:
        r = m.group(1)
        return f'self.au = self.da[{r}].wrapping_add(if {r} < 15 {{ 1 }} else {{ 2 }});'

    m = re.match(r'm_au = m_da\[(rx|ry)\] - \(\1 < 15 \? 1 : 2\);', s)
    if m:
        r = m.group(1)
        return f'self.au = self.da[{r}].wrapping_sub(if {r} < 15 {{ 1 }} else {{ 2 }});'

    if 'm_t = !(m_au & 0x3f) ? 0 : m_alue & 2 ? 1 : 2' in s:
        return 'self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };'

    if 'm_t = !(m_au & 0x3f) ? 0 : (m_alue & 3) == 1 ? 1 : (m_alue & 3) == 2 ? 2 : 3' in s:
        return 'self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };'

    if 'm_t = (m_isr & SR_Z) ? 2 : (m_isr & SR_N) ? 1 : 0' in s:
        return 'self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };'

    # Compound assignment
    m = re.match(r'(.+?)\s*(\|=|&=|\+=|-=|>>=|<<=)\s*(.+);', s)
    if m:
        lhs = subst_all(m.group(1))
        op = m.group(2)
        rhs = subst_all(m.group(3))
        return f'{lhs} {op} {rhs};'

    # Simple assignment
    m = re.match(r'(.+?)\s*=\s*(.+);', s)
    if m:
        lhs = subst_all(m.group(1).strip())
        rhs = subst_all(m.group(2).strip())
        return f'{lhs} = {rhs};'

    # Fallback
    return subst_all(s)


U32_FIELDS = {'self.au', 'self.at', 'self.pc', 'self.aob', 'self.dt',
              'self.movems', 'self.int_vector', 'self.ipc'}

def cast_alu_arg(expr, fn_name):
    """Add 'as u16' or 'as u8' cast if needed for ALU argument."""
    is_8bit = fn_name.endswith('8') and fn_name not in ['alu_asl32', 'alu_asr32', 'alu_lsl32', 'alu_lsr32',
                                                         'alu_rol32', 'alu_ror32', 'alu_roxl32', 'alu_roxr32']
    is_32bit = '32' in fn_name

    if is_32bit:
        # 32-bit ALU ops take u16 (they use alue internally for the other half)
        pass  # still u16 args
    if is_8bit:
        # 8-bit ops take u8 — but actually they take u16/u8 depending on the specific fn
        # alu_add8(u8, u8), alu_and8(u16, u16), etc. — check signatures
        # Actually in our Rust code: alu_add8(a: u8, b: u8), alu_and8(a: u16, b: u16)
        # Let's just cast u32 fields to the appropriate type
        for field in U32_FIELDS:
            if field in expr:
                return f'({expr}) as u8'
        if 'self.da[' in expr:
            return f'({expr}) as u8'
        return expr

    # 16-bit ALU: need u16
    for field in U32_FIELDS:
        if field in expr:
            return f'({expr}) as u16'
    if 'self.da[' in expr:
        return f'({expr}) as u16'
    return expr


def subst_all(s):
    """Apply all C++ → Rust substitutions."""
    # m_da[map_sp(...)]
    s = re.sub(r'm_da\[map_sp\(([^)]+)\)\]',
               lambda m: f'self.da[self.map_sp(({subst_inner(m.group(1))}) as usize)]', s)
    s = re.sub(r'm_da\[m_movems\]', 'self.da[self.movems as usize]', s)
    s = re.sub(r'm_da\[m_sp\]', 'self.da[self.sp as usize]', s)
    s = re.sub(r'm_da\[(\d+)\]', r'self.da[\1]', s)
    s = re.sub(r'm_da\[(rx|ry)\]', r'self.da[\1]', s)
    s = re.sub(r'm_decode_table\[m_ird\]', 'self.decode_table[self.ird as usize]', s)

    # Function calls
    s = re.sub(r'\b(high16|ext32|ext32h|merge_16_32)\(', r'M68000::\1(', s)

    # Member variables (must come after da[] patterns)
    s = re.sub(r'\bm_int_next_state\b', 'self.int_next_state', s)
    s = re.sub(r'\bm_(\w+)\b', r'self.\1', s)

    # Constants
    s = re.sub(r'\bS_PRIVILEDGE\b', 'S_PRIVILEGE', s)

    # Bitwise not
    s = re.sub(r'~(\d+)', r'!\1u32', s)
    s = s.replace('~', '!')

    # Ternary: simple cases
    m = re.search(r'\(([^?]+)\)\s*\?\s*([^:]+)\s*:\s*([^;,)]+)', s)
    if m:
        cond = m.group(1).strip()
        then = m.group(2).strip()
        else_ = m.group(3).strip()
        replacement = f'if ({cond}) != 0 {{ {then} }} else {{ {else_} }}'
        s = s[:m.start()] + replacement + s[m.end():]

    return s


def subst_inner(s):
    """Substitute inside map_sp() argument."""
    s = re.sub(r'\bm_(\w+)\b', r'self.\1', s)
    return s


def split_args(s):
    args, depth, current = [], 0, ''
    for c in s:
        if c == ',' and depth == 0:
            args.append(current); current = ''
        else:
            if c == '(': depth += 1
            elif c == ')': depth -= 1
            current += c
    if current: args.append(current)
    return args


def main():
    handlers = parse_handlers(INPUT)
    print(f"Parsed {len(handlers)} handlers")
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    CHUNK = 150
    num_files = (len(handlers) + CHUNK - 1) // CHUNK
    mod_lines = []
    dispatch_arms = []

    for file_idx in range(num_files):
        start = file_idx * CHUNK
        end = min(start + CHUNK, len(handlers))
        chunk = handlers[start:end]
        filename = f"handlers_{file_idx}.rs"
        mod_lines.append(f'mod handlers_{file_idx};\n')

        with open(os.path.join(OUTPUT_DIR, filename), 'w') as f:
            f.write('use crate::cpu::M68000;\nuse crate::types::*;\n\n')
            f.write('#[allow(non_snake_case, unused_variables, unused_assignments, unused_mut, unreachable_code)]\n')
            f.write('impl M68000 {\n')
            for func_name, body in chunk:
                rust_body = translate_handler(func_name, body)
                f.write(f'    pub fn handler_{func_name}(&mut self, bus: &mut impl crate::types::Bus) {{\n')
                f.write(rust_body)
                f.write('\n    }\n\n')
            f.write('}\n')

        for idx, (func_name, _) in enumerate(chunk):
            dispatch_arms.append((start + idx, func_name))

    mod_lines.append('mod dispatch;\n')
    with open(os.path.join(OUTPUT_DIR, 'dispatch.rs'), 'w') as f:
        f.write('use crate::cpu::M68000;\nuse crate::types::*;\n\n')
        f.write('#[allow(unused_variables)]\nimpl M68000 {\n')
        f.write('    pub fn dispatch(&mut self, bus: &mut impl crate::types::Bus) {\n')
        f.write('        match self.inst_state {\n')
        for state_idx, func_name in dispatch_arms:
            f.write(f'            {state_idx} => self.handler_{func_name}(bus),\n')
        f.write('            _ => self.handler_state_illegal(bus),\n')
        f.write('        }\n    }\n}\n')

    with open(os.path.join(OUTPUT_DIR, 'mod.rs'), 'w') as f:
        f.writelines(mod_lines)

    print(f"Generated {num_files} handler files + dispatch.rs")


if __name__ == '__main__':
    main()
