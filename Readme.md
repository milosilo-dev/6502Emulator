# 6502 Emulator (Rust)

A full **MOS 6502** emulator written in **pure Rust** — with **zero external dependencies** (besides `std`).

Designed with flexibility in mind, this emulator features a **dynamic address bus**, allowing you to plug in custom devices directly into the CPU. This makes it suitable as the core for larger systems such as a BBC Micro–style emulator, game consoles, or custom 6502-based projects.

---

## Features

- Pure Rust (`std` only)
- Dynamic, pluggable address bus
- Instruction-by-instruction test coverage
- Designed for extensibility
- Still under active development

---

## Architecture

The emulator exposes a flexible address bus interface so that memory and hardware devices can be attached dynamically. This allows you to:

- Map RAM/ROM however you like  
- Implement memory-mapped I/O  
- Attach custom hardware devices  
- Build a complete retro system around the CPU core  

The goal is to make this usable as the heart of a full 8-bit system emulator.

---

## Tests

Every implemented instruction has a dedicated test.

All tests were generated using AI to ensure they match the *real* 6502 behaviour — not just assumptions about how the CPU should work.  

> The rest of the emulator code is entirely AI-free

---

## Known Issues & Limitations

- The original 6502 page-boundary indirect JMP bug is not currently emulated:  
  > The real 6502 incorrectly fetches the high byte of the target address if the indirect vector falls on a page boundary.
- Not all instructions are implemented yet (see below).
- Not every physical pin of the real 6502 is exposed as a public function on the CPU struct.
- The current `execute()` function will likely be replaced with a `step()` function that:
  - Executes exactly one instruction at the current PC
  - Returns the number of clock cycles used
- The CPU implementation currently lives in a single file and could benefit from refactoring.

---

## Implemented Instructions (39 / 56)

| ADC | AND | ASL | BCC |
|-----|-----|-----|-----|
| BCS | BEQ | BIT | BMI |
| BNE | BPL | BRK | BVC |
| BVS | CLC | CLD | CLI |
| CLV | CMP | CPX | CPY |
| DEC | DEX | DEY | EOR |
| INC | INX | INY | JMP |
| JSR | LDA | LDX | LDY |
| NOP | ORA | PHA | PHP |
| PLA | PLP | ROL |     |

---

## Roadmap

- [ ] Implement remaining instructions  
- [ ] Add optional emulation of original hardware quirks  
- [ ] Convert `execute()` → `step()` API  
- [ ] Improve code structure & formatting  
- [ ] Expand documentation and usage examples  

---

## Project Goals

This emulator aims to be:

- Accurate  
- Clean and dependency-free  
- Flexible enough to power full-system emulation  
- Educational and easy to extend  
