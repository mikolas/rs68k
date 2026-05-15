# rs68k

A cycle-exact Motorola 68000 CPU emulator in Rust, derived from MAME's Olivier Galibert
microcode-based core. Verified against 317,500 test vectors with 100% pass rate.

## Features

- Cycle-exact execution matching real 68000 silicon timing
- Two execution modes:
  - **Full handlers**: one instruction per call (fast path)
  - **Pin-level partial handlers**: suspend/resume at bus access boundaries (for parallel hardware emulation)
- Drop-in `clock()` interface with real bus signals (AS, UDS, LDS, DTACK, VPA, E clock, bus arbitration)
- Generated from the same microcode intermediate representation as MAME's C++ output
- 1535 instruction handlers covering all 68000 opcodes and exception states
- No runtime dependencies

## Building

```
cargo build --release
```

## Testing

```
# Full instruction test suite (127 instructions, 317,500 vectors)
cargo test --release --test sst

# Pin-level partial handler verification (127 instructions, 63,500 vectors)
cargo test --release --test partial partial_sst_all

# All tests
cargo test --release
```

Test data is in `external/SingleStepTests` (git submodule). Initialize with:
```
git submodule update --init
```

## Benchmarking

```
cargo run --release --example bench
cargo run --release --example bench_partial
```

## Usage

See `MANUAL.md` for integration instructions and `BUS_INTERFACE.md` for the
complete bus interface specification.

## Documentation

| File | Contents |
|------|----------|
| `MANUAL.md` | Integration guide, API reference, testing instructions |
| `BUS_INTERFACE.md` | Bus interface specification (both modes) |
| `STATUS.md` | Development history and current state |
| `DEBUG.md` | Debugging facilities specification |

## Origin

The code generator (`gen_rust.py`) is a fork of MAME's `m68000gen.py` by Olivier Galibert.
The microcode and nanocode tables are unchanged from the original. Only the output layer
was replaced to emit Rust instead of C++.

## License

BSD-3-Clause

This project is derived from MAME's m68000 CPU core, which is licensed under the
BSD 3-Clause License. See below.

```
Copyright (c) Olivier Galibert and the MAME team
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors
   may be used to endorse or promote products derived from this software without
   specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
THE POSSIBILITY OF SUCH DAMAGE.
```
