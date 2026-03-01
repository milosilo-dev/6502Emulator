        LDX #$05
loop:   DEX         ; decrement X
        BNE loop    ; loop until X = 0
        NOP

        LDA #$00
        CMP #$00   ; Z flag should be set
        BNE fail
        CLC
        ADC #$01   ; C flag should be set after addition
        BCC fail
success:
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