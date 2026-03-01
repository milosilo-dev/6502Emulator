        SUCCESS_ADDR = $6666

        LDX #$05
loop:   DEX         ; decrement X
        BNE loop    ; loop until X = 0
        NOP

        LDA #$00
        CMP #$00   ; Z flag should be set
        BNE fail
        CLC
        ADC #$01   ; C flag should be set after addition
        BCS fail
success:
        LDA #$FF
        STA SUCCESS_ADDR ; load FF at 6666 for a sucsess
successloop:
        JMP successloop ; loop if success
        NOP
fail:
        JMP fail   ; trap if wrong

        JSR sub
        NOP
        NOP
        JMP end

sub:    LDA #$42
        RTS
end:    NOP