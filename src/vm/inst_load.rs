use crate::state::lua_state::LuaState;
use crate::api::lua_state::LuaAPI;
use crate::api::lua_vm::LuaVM;

use super::instruction::Instruction;

pub fn load_nil(i: u32, vm: &mut LuaState) {
    let (mut a, b, _) = i.abc();
    a += 1;

    vm.push_nil();
    for i in a..a+b+1 {
        vm.copy(-1, i);
    }
    vm.pop(1);
}

pub fn load_bool(i: u32, vm: &mut LuaState) {
    let (mut a, b, c) = i.abc();
    a += 1;
    vm.push_boolean(b != 0);
    vm.replace(a);
    if c != 0 {
        vm.add_pc(1);
    }
}

pub fn load_k(i: u32, vm: &mut LuaState) {
    let (mut a, bx) = i.a_bx();
    a += 1;

    vm.get_const(bx);
    vm.replace(a);
}

pub fn load_kx(i: u32, vm: &mut LuaState) {
    let (mut a, _) = i.a_bx();
    a += 1;
    let ax = vm.fetch().ax();

    vm.get_const(ax);
    vm.replace(a);
}