use crate::api::consts::{LUA_OPSUB, LUA_OPLE, LUA_OPADD};
use crate::state::lua_state::LuaState;
use crate::api::lua_state::LuaAPI;
use crate::api::lua_vm::LuaVM;

use super::instruction::Instruction;
use crate::api::consts::*;

pub fn for_prep(i: u32, vm: &mut LuaState) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;

    if vm.type_id(a) == LUA_TSTRING {
        vm.push_number(vm.to_number(a));
        vm.replace(a);
    }
    if vm.type_id(a + 1) == LUA_TSTRING {
        vm.push_number(vm.to_number(a + 1));
        vm.replace(a + 1);
    }
    if vm.type_id(a + 2) == LUA_TSTRING {
        vm.push_number(vm.to_number(a + 2));
        vm.replace(a + 2);
    }

    vm.push_value(a);
    vm.push_value(a + 2);
    vm.arith(LUA_OPSUB);
    vm.replace(a);
    vm.add_pc(sbx);
}

pub fn for_loop(i: u32, vm: &mut LuaState) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;

    vm.push_value(a + 2);
    vm.push_value(a);
    vm.arith(LUA_OPADD);
    vm.replace(a);

    let is_positive_step = vm.to_number(a+2) >= 0.0;
    if is_positive_step && vm.compare(a, a+1, LUA_OPLE) ||
      !is_positive_step && vm.compare(a+1, a, LUA_OPLE) {
        vm.add_pc(sbx);
        vm.copy(a, a+3);
    }
}