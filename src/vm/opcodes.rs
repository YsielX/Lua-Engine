#[derive(Copy, Clone)]
pub enum OpMode {
    IABC,
    IABx,
    IAsBx,
    IAx
}

#[derive(Copy, Clone)]
pub enum OpArgMode {
    OpArgN,
    OpArgU,
    OpArgR,
    OpArgK
}

pub struct Opcode {
    pub test_flag: bool,
    pub set_a_flag: bool,
    pub arg_b_mode: OpArgMode,
    pub arg_c_mode: OpArgMode,
    pub op_mode: OpMode,
    pub name: &'static str
}

pub const OPCODES: &[Opcode] = &[
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IABC, "MOVE    "), // R(A) := R(B)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgN, OpMode::IABx, "LOADK   "), // R(A) := Kst(Bx)
    opcode( false, true, OpArgMode::OpArgN, OpArgMode::OpArgN, OpMode::IABx, "LOADKX  "), // R(A) := Kst(extra arg)
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgU, OpMode::IABC, "LOADBOOL"), // R(A) := (bool)B; if (C) pc++
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgN, OpMode::IABC, "LOADNIL "), // R(A), R(A+1), ..., R(A+B) := nil
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgN, OpMode::IABC, "GETUPVAL"), // R(A) := UpValue[B]
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgK, OpMode::IABC, "GETTABUP"), // R(A) := UpValue[B][RK(C)]
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgK, OpMode::IABC, "GETTABLE"), // R(A) := R(B)[RK(C)]
    opcode( false, false, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "SETTABUP"), // UpValue[A][RK(B)] := RK(C)
    opcode( false, false, OpArgMode::OpArgU, OpArgMode::OpArgN, OpMode::IABC, "SETUPVAL"), // UpValue[B] := R(A)
    opcode( false, false, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "SETTABLE"), // R(A)[RK(B)] := RK(C)
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgU, OpMode::IABC, "NEWTABLE"), // R(A) := {} (size = B,C)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgK, OpMode::IABC, "SELF    "), // R(A+1) := R(B); R(A) := R(B)[RK(C)]
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "ADD     "), // R(A) := RK(B) + RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "SUB     "), // R(A) := RK(B) - RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "MUL     "), // R(A) := RK(B) * RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "MOD     "), // R(A) := RK(B) % RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "POW     "), // R(A) := RK(B) ^ RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "DIV     "), // R(A) := RK(B) / RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "IDIV    "), // R(A) := RK(B) // RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "BAND    "), // R(A) := RK(B) & RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "BOR     "), // R(A) := RK(B) | RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "BXOR    "), // R(A) := RK(B) ~ RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "SHL     "), // R(A) := RK(B) << RK(C)
    opcode( false, true, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "SHR     "), // R(A) := RK(B) >> RK(C)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IABC, "UNM     "), // R(A) := -R(B)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IABC, "BNOT    "), // R(A) := ~R(B)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IABC, "NOT     "), // R(A) := not R(B)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IABC, "LEN     "), // R(A) := length of R(B)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgR, OpMode::IABC, "CONCAT  "), // R(A) := R(B).. ... ..R(C)
    opcode( false, false, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IAsBx, "JMP     "), // pc+=sBx; if (A) close all upvalues >= R(A - 1)
    opcode( true, false, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "EQ      "), // if ((RK(B) == RK(C)) ~= A) then pc++
    opcode( true, false, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "LT      "), // if ((RK(B) <  RK(C)) ~= A) then pc++
    opcode( true, false, OpArgMode::OpArgK, OpArgMode::OpArgK, OpMode::IABC, "LE      "), // if ((RK(B) <= RK(C)) ~= A) then pc++
    opcode( true, false, OpArgMode::OpArgN, OpArgMode::OpArgU, OpMode::IABC, "TEST    "), // if not (R(A) <=> C) then pc++
    opcode( true, true, OpArgMode::OpArgR, OpArgMode::OpArgU, OpMode::IABC, "TESTSET "), // if (R(B) <=> C) then R(A) := R(B) else pc++
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgU, OpMode::IABC, "CALL    "), // R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgU, OpMode::IABC, "TAILCALL"), // return R(A)(R(A+1), ... ,R(A+B-1))
    opcode( false, false, OpArgMode::OpArgU, OpArgMode::OpArgN, OpMode::IABC, "RETURN  "), // return R(A), ... ,R(A+B-2)
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IAsBx, "FORLOOP "), // R(A)+=R(A+2); if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IAsBx, "FORPREP "), // R(A)-=R(A+2); pc+=sBx
    opcode( false, false, OpArgMode::OpArgN, OpArgMode::OpArgU, OpMode::IABC, "TFORCALL"),  // R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));
    opcode( false, true, OpArgMode::OpArgR, OpArgMode::OpArgN, OpMode::IAsBx, "TFORLOOP"), // if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }
    opcode( false, false, OpArgMode::OpArgU, OpArgMode::OpArgU, OpMode::IABC, "SETLIST "),  // R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgN, OpMode::IABx, "CLOSURE "),  // R(A) := closure(KPROTO[Bx])
    opcode( false, true, OpArgMode::OpArgU, OpArgMode::OpArgN, OpMode::IABC, "VARARG  "),  // R(A), R(A+1), ..., R(A+B-2) = vararg
    opcode( false, false, OpArgMode::OpArgU, OpArgMode::OpArgU, OpMode::IAx, "EXTRAARG"),   // extra (larger) argument for previous opcode
];

const fn opcode(test_flag: bool, set_a_flag: bool, arg_b_mode: OpArgMode, arg_c_mode: OpArgMode, op_mode: OpMode, name: &'static str) -> Opcode {
    Opcode {
        test_flag,
        set_a_flag,
        arg_b_mode,
        arg_c_mode,
        op_mode,
        name
    }
}