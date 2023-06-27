use crate::state::lua_state::LuaState;
use crate::api::lua_state::LuaAPI;
use crate::api::lua_vm::LuaVM;

use super::instruction::Instruction;

pub fn move_(i: u32, vm: &mut LuaState) {
    let (a, b, _) = i.abc();
    vm.copy(b+1, a+1);
}

pub fn jmp(i: u32, vm: &mut LuaState) {
    let (a, sbx) = i.a_sbx();
    vm.add_pc(sbx);
    if a != 0 {
        unimplemented!()
    }
}