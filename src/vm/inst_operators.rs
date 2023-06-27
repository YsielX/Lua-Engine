use crate::state::lua_state::LuaState;
use crate::api::lua_state::LuaAPI;
use crate::api::lua_vm::LuaVM;
use crate::api::consts::*;

use super::instruction::Instruction;

pub fn add(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPADD)
}

pub fn sub(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPSUB)
}

pub fn mul(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPMUL)
}

pub fn mod_(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPMOD)
}

pub fn pow(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPPOW)
}

pub fn div(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPDIV)
}

pub fn idiv(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPIDIV)
}

pub fn band(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPBAND)
}

pub fn bor(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPBOR)
}

pub fn bxor(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPBXOR)
}

pub fn shl(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPSHL)
}

pub fn shr(i: u32, vm: &mut LuaState) {
    _binary_arith(i, vm, LUA_OPSHR)
}

pub fn unm(i: u32, vm: &mut LuaState) {
    _unary_arith(i, vm, LUA_OPUNM)
}

pub fn bnot(i: u32, vm: &mut LuaState) {
    _unary_arith(i, vm, LUA_OPBNOT)
}

pub fn len(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.len(b);
    vm.replace(a);
}

pub fn concat(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, mut c) = i.abc();
    a += 1;
    b += 1;
    c += 1;

    let n = c - b + 1;
    for i in b..c+1 {
        vm.push_value(i);
    }
    vm.concat(n);
    vm.replace(a);
}

pub fn eq(i: u32, vm: &mut LuaState) {
    _compare(i, vm, LUA_OPEQ)
}

pub fn lt(i: u32, vm: &mut LuaState) {
    _compare(i, vm, LUA_OPLT)
}

pub fn le(i: u32, vm: &mut LuaState) {
    _compare(i, vm, LUA_OPLE)
}

pub fn not(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.push_boolean(!vm.to_boolean(b));
    vm.replace(a);
}

pub fn test_set(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, c) = i.abc();
    a += 1;
    b += 1;

    if vm.to_boolean(b) == (c != 0) {
        vm.copy(b, a);
    } else {
        vm.add_pc(1);
    }
}

pub fn test(i: u32, vm: &mut LuaState) {
    let (mut a, _, c) = i.abc();
    a += 1;

    if vm.to_boolean(a) != (c != 0) {
        vm.add_pc(1);
    }
}

fn _binary_arith(i: u32, vm: &mut LuaState, op: u8) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.get_rk(b);
    vm.get_rk(c);
    vm.arith(op);
    vm.replace(a);
}

fn _unary_arith(i: u32, vm: &mut LuaState, op: u8) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.push_value(b);
    vm.arith(op);
    vm.replace(a);
}

fn _compare(i: u32, vm: &mut LuaState, op: u8) {
    let (a, b, c) = i.abc();

    vm.get_rk(b);
    vm.get_rk(c);
    if vm.compare(-2, -1, op) != (a != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
}