use crate::state::lua_state::LuaState;

use super::opcodes::{OpArgMode, OpMode, OPCODES};

const MAXARG_BX: isize = (1 << 18) - 1;
const MAXARG_SBX: isize = MAXARG_BX >> 1;

pub trait Instruction {
    fn opname(self) -> &'static str;
    fn opmode(self) -> OpMode;
    fn b_mode(self) -> OpArgMode;
    fn c_mode(self) -> OpArgMode;
    fn opcode(self) -> u8;
    fn abc(self) -> (isize, isize, isize);
    fn a_bx(self) -> (isize, isize);
    fn a_sbx(self) -> (isize, isize);
    fn ax(self) -> isize;
    fn execute(self, _: &mut LuaState);
}

impl Instruction for u32 {
    fn opname(self) -> &'static str {
        OPCODES[self.opcode() as usize].name
    }

    fn opmode(self) -> OpMode {
        OPCODES[self.opcode() as usize].op_mode
    }

    fn b_mode(self) -> OpArgMode {
        OPCODES[self.opcode() as usize].arg_b_mode
    }

    fn c_mode(self) -> OpArgMode {
        OPCODES[self.opcode() as usize].arg_c_mode
    }

    fn opcode(self) -> u8 {
        self as u8 & 0x3F
    }

    fn abc(self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let c = (self >> 14 & 0x1FF) as isize;
        let b = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    fn a_bx(self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    fn a_sbx(self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_SBX)
    }

    fn ax(self) -> isize {
        (self >> 6) as isize
    }

    fn execute(self, vm: &mut LuaState) {
        let action = OPCODES[self.opcode() as usize].action;
        action(self, vm);
    }
}
