use crate::state::lua_state::LuaState;
use super::inst_for::*;
use super::inst_load::*;
use super::inst_misc::*;
use super::inst_operators::*;
use super::inst_table::*;

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
    pub name: &'static str,
    pub action: fn(i: u32, vm: &mut LuaState)
}

pub const OPCODES: &[Opcode] = &[
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "MOVE    ", action: move_}, // R(A) := R(B)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABx, name: "LOADK   ", action: load_k}, // R(A) := Kst(Bx)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgN, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABx, name: "LOADKX  ", action: load_kx}, // R(A) := Kst(extra arg)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "LOADBOOL", action: load_bool}, // R(A) := (bool)B; if (C) pc++
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "LOADNIL ", action: load_nil}, // R(A), R(A+1), ..., R(A+B) := nil
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "GETUPVAL", action: fail}, // R(A) := UpValue[B]
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "GETTABUP", action: fail}, // R(A) := UpValue[B][RK(C)]
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "GETTABLE", action: get_table}, // R(A) := R(B)[RK(C)]
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "SETTABUP", action: fail}, // UpValue[A][RK(B)] := RK(C)
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "SETUPVAL", action: fail}, // UpValue[B] := R(A)
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "SETTABLE", action: set_table}, // R(A)[RK(B)] := RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "NEWTABLE", action: new_table}, // R(A) := {} (size = B,C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "SELF    ", action: fail}, // R(A+1) := R(B); R(A) := R(B)[RK(C)]
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "ADD     ", action: add}, // R(A) := RK(B) + RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "SUB     ", action: sub}, // R(A) := RK(B) - RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "MUL     ", action: mul}, // R(A) := RK(B) * RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "MOD     ", action: mod_}, // R(A) := RK(B) % RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "POW     ", action: pow}, // R(A) := RK(B) ^ RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "DIV     ", action: div}, // R(A) := RK(B) / RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "IDIV    ", action: idiv}, // R(A) := RK(B) // RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "BAND    ", action: band}, // R(A) := RK(B) & RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "BOR     ", action: bor}, // R(A) := RK(B) | RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "BXOR    ", action: bxor}, // R(A) := RK(B) ~ RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "SHL     ", action: shl}, // R(A) := RK(B) << RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "SHR     ", action: shr}, // R(A) := RK(B) >> RK(C)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "UNM     ", action: unm}, // R(A) := -R(B)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "BNOT    ", action: bnot}, // R(A) := ~R(B)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "NOT     ", action: not}, // R(A) := not R(B)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "LEN     ", action: len}, // R(A) := length of R(B)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgR, op_mode: OpMode::IABC, name: "CONCAT  ", action: concat}, // R(A) := R(B).. ... ..R(C)
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IAsBx, name: "JMP     ", action: jmp}, // pc+=sBx; if (A) close all upvalues >= R(A - 1)
    Opcode{ test_flag: true, set_a_flag: false, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "EQ      ", action: eq}, // if ((RK(B) == RK(C)) ~= A) then pc++
    Opcode{ test_flag: true, set_a_flag: false, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "LT      ", action: lt}, // if ((RK(B) <  RK(C)) ~= A) then pc++
    Opcode{ test_flag: true, set_a_flag: false, arg_b_mode: OpArgMode::OpArgK, arg_c_mode: OpArgMode::OpArgK, op_mode: OpMode::IABC, name: "LE      ", action: le}, // if ((RK(B) <= RK(C)) ~= A) then pc++
    Opcode{ test_flag: true, set_a_flag: false, arg_b_mode: OpArgMode::OpArgN, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "TEST    ", action: test}, // if not (R(A) <=> C) then pc++
    Opcode{ test_flag: true, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "TESTSET ", action: test_set}, // if (R(B) <=> C) then R(A) := R(B) else pc++
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "CALL    ", action: fail}, // R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "TAILCALL", action: fail}, // return R(A)(R(A+1), ... ,R(A+B-1))
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "RETURN  ", action: fail}, // return R(A), ... ,R(A+B-2)
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IAsBx, name: "FORLOOP ", action: for_loop}, // R(A)+=R(A+2); if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IAsBx, name: "FORPREP ", action: for_prep}, // R(A)-=R(A+2); pc+=sBx
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgN, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "TFORCALL", action: fail},  // R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgR, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IAsBx, name: "TFORLOOP", action: fail}, // if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IABC, name: "SETLIST ", action: set_list},  // R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABx, name: "CLOSURE ", action: fail},  // R(A) := closure(KPROTO[Bx])
    Opcode{ test_flag: false, set_a_flag: true, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgN, op_mode: OpMode::IABC, name: "VARARG  ", action: fail},  // R(A), R(A+1), ..., R(A+B-2) = vararg
    Opcode{ test_flag: false, set_a_flag: false, arg_b_mode: OpArgMode::OpArgU, arg_c_mode: OpArgMode::OpArgU, op_mode: OpMode::IAx, name: "EXTRAARG", action: fail},   // extra (larger) argument for previous opcode
];

fn fail(_: u32, _: &mut LuaState) {
    unimplemented!()
}