pub(crate) trait ARM {
    fn decode(&mut self, opcode: u32);

    // ALU Logic
    //AND{cond}{S} Rd,Rn,Op2    ;AND logical       Rd = Rn AND Op2
    fn logical_and(&mut self, rd: usize, rn: usize, op2: u32);
    //EOR{cond}{S} Rd,Rn,Op2    ;XOR logical       Rd = Rn XOR Op2
    fn logical_xor(&mut self, rd: usize, rn: usize, op2: u32);
    ////SUB{cond}{S} Rd,Rn,Op2 ;* ;subtract          Rd = Rn-Op2
    fn sub(&mut self, rd: usize, rn: usize, op2: u32);
    ////RSB{cond}{S} Rd,Rn,Op2 ;* ;subtract reversed Rd = Op2-Rn
    fn rev_sub(&mut self, rd: usize, rn: usize, op2: u32);
    ////ADD{cond}{S} Rd,Rn,Op2 ;* ;add               Rd = Rn+Op2
    fn add(&mut self, rd: usize, rn: usize, op2: u32);
    ////ADC{cond}{S} Rd,Rn,Op2 ;* ;add with carry    Rd = Rn+Op2+Cy
    fn add_with_carry(&mut self, rd: usize, rn: usize, op2: u32);
    ////SBC{cond}{S} Rd,Rn,Op2 ;* ;sub with carry    Rd = Rn-Op2+Cy-1
    fn sub_with_carry(&mut self, rd: usize, rn: usize, op2: u32);
    ////RSC{cond}{S} Rd,Rn,Op2 ;* ;sub cy. reversed  Rd = Op2-Rn+Cy-1
    fn rev_sub_with_carry(&mut self, rn: usize, rd: usize, op2: u32);
    ////TST{cond}{P}    Rn,Op2    ;test            Void = Rn AND Op2
    //fn test_and(&mut self, rn: usize, op2: u32);
    ////TEQ{cond}{P}    Rn,Op2    ;test exclusive  Void = Rn XOR Op2
    //fn test_xor(&mut self, rn: usize, op2: u32);
    ////CMP{cond}{P}    Rn,Op2 ;* ;compare         Void = Rn-Op2
    //fn test_sub(&mut self, rn: usize, op2: u32);
    ////CMN{cond}{P}    Rn,Op2 ;* ;compare neg.    Void = Rn+Op2
    //fn comp_neg(&mut self, rn: usize, op2: u32);
    ////ORR{cond}{S} Rd,Rn,Op2    ;OR logical        Rd = Rn OR Op2
    //fn logical_or(&mut self, rn: usize, rd: usize, op2: u32);
    ////MOV{cond}{S} Rd,Op2       ;move              Rd = Op2
    //fn set_equal(&mut self, rn: usize, rd: usize, op2: u32);
    ////BIC{cond}{S} Rd,Rn,Op2    ;bit clear         Rd = Rn AND NOT Op2
    //fn bit_clear(&mut self, rn: usize, rd: usize, op2: u32);
    ////MVN{cond}{S} Rd,Op2       ;not               Rd = NOT Op2
    //fn local_not(&mut self, rn: usize, rd: usize, op2: u32);

    // Multiply
    // MUL{cond}{S} Rd,Rm,Rs            1S+mI     NZx-  Rd = Rm*Rs
    // MLA{cond}{S} Rd,Rm,Rs,Rn         1S+mI+1I  NZx-  Rd = Rm*Rs+Rn
    // UMULL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+1I  NZx-  RdHiLo = Rm*Rs
    // UMLAL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+2I  NZx-  RdHiLo = Rm*Rs+RdHiLo
    // SMULL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+1I  NZx-  RdHiLo = Rm*Rs
    // SMLAL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+2I  NZx-  RdHiLo = Rm*Rs+RdHiLo
    // SMLAxy{cond}  Rd,Rm,Rs,Rn     ARMv5TE(xP)  ----q Rd=HalfRm*HalfRs+Rn
    // SMLAWy{cond}  Rd,Rm,Rs,Rn     ARMv5TE(xP)  ----q Rd=(Rm*HalfRs)/10000h+Rn
    // SMULWy{cond}  Rd,Rm,Rs        ARMv5TE(xP)  ----  Rd=(Rm*HalfRs)/10000h
    // SMLALxy{cond} RdLo,RdHi,Rm,Rs ARMv5TE(xP)  ----  RdHiLo=RdHiLo+HalfRm*HalfRs
    // SMULxy{cond}  Rd,Rm,Rs        ARMv5TE(xP)  ----  Rd=HalfRm*HalfRs

    // Memory Load/Store
    // LDR{cond}{B}{T} Rd,<Address>     1S+1N+1I+y ----  Rd=[Rn+/-<offset>]
    // LDR{cond}H      Rd,<Address>     1S+1N+1I+y ----  Load Unsigned halfword
    // LDR{cond}D      Rd,<Address>                ----  Load Dword ARMv5TE
    // LDR{cond}SB     Rd,<Address>     1S+1N+1I+y ----  Load Signed byte
    // LDR{cond}SH     Rd,<Address>     1S+1N+1I+y ----  Load Signed halfword
    // LDM{cond}{amod} Rn{!},<Rlist>{^} nS+1N+1I+y ----  Load Multiple
    // STR{cond}{B}{T} Rd,<Address>     2N         ----  [Rn+/-<offset>]=Rd
    // STR{cond}H      Rd,<Address>     2N         ----  Store halfword
    // STR{cond}D      Rd,<Address>                ----  Store Dword ARMv5TE
    // STM{cond}{amod} Rn{!},<Rlist>{^} (n-1)S+2N  ----  Store Multiple
    // SWP{cond}{B}    Rd,Rm,[Rn]       1S+2N+1I   ----  Rd=[Rn], [Rn]=Rm
    // PLD             <Address>        1S         ----  Prepare Cache ARMv5TE

    // Jumps, Calls, CPSR Mode, and others
    // B{cond}   label              2S+1N    ----  PC=$+8+/-32M
    // BL{cond}  label              2S+1N    ----  PC=$+8+/-32M, LR=$+4
    // BX{cond}  Rn                 2S+1N    ----  PC=Rn, T=Rn.0 (THUMB/ARM)
    // BLX{cond} Rn                 2S+1N    ----  PC=Rn, T=Rn.0, LR=PC+4, ARM9
    // BLX       label              2S+1N    ----  PC=PC+$+/-32M, LR=$+4, T=1, ARM9
    // MRS{cond} Rd,Psr             1S       ----  Rd=Psr
    // MSR{cond} Psr{_field},Op     1S      (psr)  Psr[field]=Op
    // SWI{cond} Imm24bit           2S+1N    ----  PC=8, ARM Svc mode, LR=$+4
    // BKPT      Imm16bit           ???      ----  PC=C, ARM Abt mode, LR=$+4 ARM9
    // The Undefined Instruction    2S+1I+1N ----  PC=4, ARM Und mode, LR=$+4
    // cond=false                   1S       ----  Any opcode with condition=false
    // NOP                          1S       ----  R0=R0

    // ARM9 only?
    // CLZ{cond} Rd,Rm              ???      ----    Count Leading Zeros ARMv5
    // QADD{cond} Rd,Rm,Rn                   ----q   Rd=Rm+Rn       ARMv5TE(xP)
    // QSUB{cond} Rd,Rm,Rn                   ----q   Rd=Rm-Rn       ARMv5TE(xP)
    // QDADD{cond} Rd,Rm,Rn                  ----q   Rd=Rm+Rn*2     ARMv5TE(xP)
    // QDSUB{cond} Rd,Rm,Rn                  ----q   Rd=Rm-Rn*2     ARMv5TE(xP)
}

pub(crate) trait Thumb {
    // ALU Logic
    //MOV Rd,Imm8bit      1S     NZ--  3   Rd=nn
    //MOV Rd,Rs           1S     NZ00  2   Rd=Rs+0
    //MOV R0..14,R8..15   1S     ----  5   Rd=Rs
    //MOV R8..14,R0..15   1S     ----  5   Rd=Rs
    //MOV R15,R0..15      2S+1N  ----  5   PC=Rs
    //MVN Rd,Rs           1S     NZ--  4   Rd=NOT Rs
    //AND Rd,Rs           1S     NZ--  4   Rd=Rd AND Rs
    //TST Rd,Rs           1S     NZ--  4 Void=Rd AND Rs
    //BIC Rd,Rs           1S     NZ--  4   Rd=Rd AND NOT Rs
    //ORR Rd,Rs           1S     NZ--  4   Rd=Rd OR Rs
    //EOR Rd,Rs           1S     NZ--  4   Rd=Rd XOR Rs
    //LSL Rd,Rs,Imm5bit   1S     NZc-  1   Rd=Rs SHL nn
    //LSL Rd,Rs           1S+1I  NZc-  4   Rd=Rd SHL (Rs AND 0FFh)
    //LSR Rd,Rs,Imm5bit   1S     NZc-  1   Rd=Rs SHR nn
    //LSR Rd,Rs           1S+1I  NZc-  4   Rd=Rd SHR (Rs AND 0FFh)
    //ASR Rd,Rs,Imm5bit   1S     NZc-  1   Rd=Rs SAR nn
    //ASR Rd,Rs           1S+1I  NZc-  4   Rd=Rd SAR (Rs AND 0FFh)
    //ROR Rd,Rs           1S+1I  NZc-  4   Rd=Rd ROR (Rs AND 0FFh)
    //NOP                 1S     ----  5   R8=R8

    // ALU Ops and Multiply
    //ADD Rd,Rs,Imm3bit   1S     NZCV  2   Rd=Rs+nn
    //ADD Rd,Imm8bit      1S     NZCV  3   Rd=Rd+nn
    //ADD Rd,Rs,Rn        1S     NZCV  2   Rd=Rs+Rn
    //ADD R0..14,R8..15   1S     ----  5   Rd=Rd+Rs
    //ADD R8..14,R0..15   1S     ----  5   Rd=Rd+Rs
    //ADD R15,R0..15      2S+1N  ----  5   PC=Rd+Rs
    //ADD Rd,PC,Imm8bit*4 1S     ---- 12   Rd=(($+4) AND NOT 2)+nn
    //ADD Rd,SP,Imm8bit*4 1S     ---- 12   Rd=SP+nn
    //ADD SP,Imm7bit*4    1S     ---- 13   SP=SP+nn
    //ADD SP,-Imm7bit*4   1S     ---- 13   SP=SP-nn
    //ADC Rd,Rs           1S     NZCV  4   Rd=Rd+Rs+Cy
    //SUB Rd,Rs,Imm3Bit   1S     NZCV  2   Rd=Rs-nn
    //SUB Rd,Imm8bit      1S     NZCV  3   Rd=Rd-nn
    //SUB Rd,Rs,Rn        1S     NZCV  2   Rd=Rs-Rn
    //SBC Rd,Rs           1S     NZCV  4   Rd=Rd-Rs-NOT Cy
    //NEG Rd,Rs           1S     NZCV  4   Rd=0-Rs
    //CMP Rd,Imm8bit      1S     NZCV  3 Void=Rd-nn
    //CMP Rd,Rs           1S     NZCV  4 Void=Rd-Rs
    //CMP R0-15,R8-15     1S     NZCV  5 Void=Rd-Rs
    //CMP R8-15,R0-15     1S     NZCV  5 Void=Rd-Rs
    //CMN Rd,Rs           1S     NZCV  4 Void=Rd+Rs
    //MUL Rd,Rs           1S+mI  NZx-  4   Rd=Rd*Rs

    // Jumps and Calls
    //B disp              2S+1N     ---- 18  PC=$+/-2048
    //BL disp             3S+1N     ---- 19  PC=$+/-4M, LR=$+5
    //B{cond=true} disp   2S+1N     ---- 16  PC=$+/-0..256
    //B{cond=false} disp  1S        ---- 16  N/A
    //BX R0..15           2S+1N     ----  5  PC=Rs, ARM/THUMB (Rs bit0)
    //SWI Imm8bit         2S+1N     ---- 17  PC=8, ARM SVC mode, LR=$+2
    //BKPT Imm8bit        ???       ---- 17  ??? ARM9 Prefetch Abort
    //BLX disp            ???       ---- ??? ??? ARM9
    //BLX R0..R14         ???       ---- ??? ??? ARM9
    //POP {Rlist,}PC   (n+1)S+2N+1I ---- 14
    //MOV R15,R0..15      2S+1N     ----  5  PC=Rs
    //ADD R15,R0..15      2S+1N     ----  5  PC=Rd+Rs

    // Memory Load/Store
    //LDR  Rd,[Rb,5bit*4] 1S+1N+1I  ----  9  Rd = WORD[Rb+nn]
    //LDR  Rd,[PC,8bit*4] 1S+1N+1I  ----  6  Rd = WORD[PC+nn]
    //LDR  Rd,[SP,8bit*4] 1S+1N+1I  ---- 11  Rd = WORD[SP+nn]
    //LDR  Rd,[Rb,Ro]     1S+1N+1I  ----  7  Rd = WORD[Rb+Ro]
    //LDRB Rd,[Rb,5bit*1] 1S+1N+1I  ----  9  Rd = BYTE[Rb+nn]
    //LDRB Rd,[Rb,Ro]     1S+1N+1I  ----  7  Rd = BYTE[Rb+Ro]
    //LDRH Rd,[Rb,5bit*2] 1S+1N+1I  ---- 10  Rd = HALFWORD[Rb+nn]
    //LDRH Rd,[Rb,Ro]     1S+1N+1I  ----  8  Rd = HALFWORD[Rb+Ro]
    //LDSB Rd,[Rb,Ro]     1S+1N+1I  ----  8  Rd = SIGNED_BYTE[Rb+Ro]
    //LDSH Rd,[Rb,Ro]     1S+1N+1I  ----  8  Rd = SIGNED_HALFWORD[Rb+Ro]
    //STR  Rd,[Rb,5bit*4] 2N        ----  9  WORD[Rb+nn] = Rd
    //STR  Rd,[SP,8bit*4] 2N        ---- 11  WORD[SP+nn] = Rd
    //STR  Rd,[Rb,Ro]     2N        ----  7  WORD[Rb+Ro] = Rd
    //STRB Rd,[Rb,5bit*1] 2N        ----  9  BYTE[Rb+nn] = Rd
    //STRB Rd,[Rb,Ro]     2N        ----  7  BYTE[Rb+Ro] = Rd
    //STRH Rd,[Rb,5bit*2] 2N        ---- 10  HALFWORD[Rb+nn] = Rd
    //STRH Rd,[Rb,Ro]     2N        ----  8  HALFWORD[Rb+Ro]=Rd
    //PUSH {Rlist}{LR}    (n-1)S+2N ---- 14
    //POP  {Rlist}{PC}              ---- 14  (ARM9: with mode switch)
    //STMIA Rb!,{Rlist}   (n-1)S+2N ---- 15
    //LDMIA Rb!,{Rlist}   nS+1N+1I  ---- 15
}
