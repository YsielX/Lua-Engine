use crate::{state::lua_state::LuaState, api::{lua_state::LuaAPI, lua_vm::LuaVM}};

use super::instruction::Instruction;

const LFIELDS_PER_FLUSH: isize = 50;

pub fn new_table(i: u32, vm: &mut LuaState) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.create_table(fb2int(b as usize), fb2int(c as usize));
    vm.replace(a);
}

pub fn get_table(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, c) = i.abc();
    a += 1;
    b += 1;

    vm.get_rk(c);
    vm.get_table(b);
    vm.replace(a);
}

pub fn set_table(i: u32, vm: &mut LuaState) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.get_rk(b);
    vm.get_rk(c);
    vm.set_table(a);
}

pub fn set_list(i: u32, vm: &mut LuaState) {
    let (mut a, b, mut c) = i.abc();
    a += 1;

    if c > 0 {
        c -= 1;
    } else {
        c = vm.fetch().ax();
    }

    let mut idx = (c * LFIELDS_PER_FLUSH) as i64;
    for j in 1..b+1 {
        idx += 1;
        vm.push_value(a + j);
        vm.set_i(a, idx);
    }
}

#[warn(dead_code)]
fn int2fb(mut x: usize) -> usize {
    let mut e = 0; /* exponent */
    if x < 8 {
        return x;
    }
    while x >= (8 << 4) {
        /* coarse steps */
        x = (x + 0xf) >> 4; /* x = ceil(x / 16) */
        e += 4;
    }
    while x >= (8 << 1) {
        /* fine steps */
        x = (x + 1) >> 1; /* x = ceil(x / 2) */
        e += 1;
    }
    return ((e + 1) << 3) | (x - 8);
}

/* converts back */
fn fb2int(x: usize) -> usize {
    if x < 8 {
        x
    } else {
        ((x & 7) + 8) << ((x >> 3) - 1)
    }
}