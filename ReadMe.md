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

### Bus

The emulator exposes a flexible address bus interface so that memory and hardware devices can be attached dynamically. This allows you to:

- Map RAM/ROM however you like  
- Implement memory-mapped I/O  
- Attach custom hardware devices  
- Build a complete retro system around the CPU core  

The goal is to make this usable as the core of a 8-bit 6502 system emulator.

The bus exists as a struct that have a vector of `devices`. A `Device` is a trait that must be implemented by any emulated hardware that wants to tie into the address bus.

The role of the bus is to simply direct read/write calls to their respective `Device`, it does this by keeping a list of all the devices and using basic linear search over the list to find the first device that covers the address specified.

Any struct that implements `Device` then have to create a `read` and `write` method, which are the end points for the `bus.read` and `bus.write` methods used in the CPU. the Device can then do what it likes with the methods.

I have done it in this way to try and closely emulate how the circitry between a 6502 and its hardware works.

### CPU

The `CPU` struct is the main struct of the project as it holds all of the logic for the 6502, due to its size, it is split up into multiple files in the `cpu` directory.

The main file where the type is defined holds the cpu registers, some public getters for the registers and a reset function.

### Instructions

The cpu has an execute prosedure, when this is called, the next byte is read from the pc's current location, it is then put through a match statement of every instruction in the 6502 instruction set, it then calls its corresponding function in `instruction.rs`.

I could improve this method with a lookup table because this would give me the ability to directly call the instruction's function rather than going through the match statement.

### Status

The 6502 Implements status through a one byte register called `status` each bit in the status register corispondes to one of the CPU flags. In 6502 assembly these are set automatically as an output for commands but some can also be set manually using speshiel commands, in my emulator, i have functions to set the cpu status outputs for different commands in `status.rs`.

### Stack

On the 6502, the stack is a single page of memory ($0100 - $01FF) and is indexed by a register called the stack pointer. The stack grows down, which means that as new values are pushed onto the stack the sp = sp - 1. On my emulator, this is managed by `stack.rs` which lets you push and pull bytes from the stack pointer.

---

## Tests

Every implemented instruction has a dedicated test.

All tests were generated using AI to ensure they match the *real* 6502 behaviour — not just my assumptions about how the CPU should work. I have read through each test case to make sure that there is no hallucinations in the tests.

It should also be noted that the test framework and thus the first few tests build are built by me in order to give the AI some idea of what I wanted.

> The rest of the emulator code is entirely AI-free

The reason the tests are so thorough (3000 lines!) is because regressions are a big problem in CPU Emulators with so many instructions to think about, these are to hopefully avoid regresions in the future.

---

## Known Issues & Limitations

- The original 6502 page-boundary indirect JMP bug is not currently emulated:  
  > The real 6502 incorrectly fetches the high byte of the target address if the indirect vector falls on a page boundary.
- Not every physical pin of the real 6502 is exposed as a public function on the CPU struct.
- The current `execute()` function will likely be replaced with a `step()` function that:
  - Executes exactly one instruction at the current PC
  - Returns the number of clock cycles used
- The CPU implementation currently lives in a single file and could benefit from refactoring.

---

## Implemented Instructions (56 / 56)

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
| PLA | PLP | ROL | ROR |
| RTI | RTS | SEC | SBC |
| SED | SEI | STA | STX |
| STY | TAX | TAY | TSX |
| TXA | TXS | TYA |     |

---

## Roadmap
 
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
