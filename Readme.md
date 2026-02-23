# 6502 Emulator
This is a full 6502 emulator built in rust with no external dependecies (apart from std!). I has a dynamic adress bus which allows the user of the emulator to build their own devices to tie directly into the CPU. This means that this emulator could be used for the core of something like a BBC Micro Emulator or similar.
## Tests
I have made sure to include a test for every instruction so that I can check they work as I build the CPU. All of these tests are writen by AI because this makes sure i get tests accurate to the original CPU and not just tests that replicate how i belive the CPU should work. (The rest of the code is AI free lol)
## Known issues
- The emulator has no way of recreating the bug where "An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary"
- I am yet to fully implement every instruction on the chip (See bellow)
- Not every pin exposed on a 6502 is acsessible as a public function from the CPU struct
- I plan to switch the "execute" function to a "step" function where the function returns the number of ticks used and just executes the next byte on the PC as an instruction.
- The formating of the code could be better, all of the actual CPU is in one file!
## Impemented instructions (35 / 56):

| ADC | AND | ASL | BCC |
|-----|-----|-----|-----|
| BCS | BEQ | BIT | BMI |
| BNE | BPL | BRK | BVC |
| BVS | CLC | CLD | CLI |
| CLV | CMP | CPX | CPY |
| DEC | DEX | DEY | EOR |
| INC | INX | INY | JMP |
| JSR | LDA | LDX | LDY |
| NOP | ORA |     |     |
