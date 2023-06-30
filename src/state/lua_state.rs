use std::cell::RefCell;
use std::rc::Rc;

use crate::api::consts;
use crate::api::lua_state::LuaAPI;
use crate::api::lua_vm::LuaVM;
use crate::binchunk::binary_chunk::Constant;
use crate::binchunk::binary_chunk::Prototype;
use super::lua_stack::LuaStack;
use super::lua_table::LuaTable;
use super::lua_value::LuaValue;
use super::arith_ops::*;
use super::compare_ops::*;

// #[derive(Debug)]
pub struct LuaState {
    stack: LuaStack,
    proto: Prototype,
    pc: isize
}

impl LuaState {
    pub fn new(stack_size: usize, proto: Prototype) -> LuaState {
        LuaState { 
            stack: LuaStack::new(stack_size),
            proto,
            pc: 0 
        }
    }
}

impl LuaAPI for LuaState {
    fn get_top(&self) -> usize {
        self.stack.top
    }

    fn abs_index(&self, idx: isize) -> usize {
        self.stack.abs_index(idx).unwrap()
    }

    fn check_stack(&mut self, _n: isize) -> bool {
        true
    }

    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.stack.pop();
        }
    }

    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        let val = self.stack.get(from_idx).unwrap();
        self.stack.set(to_idx, val);
    }

    fn push_value(&mut self, idx: isize) {
        let val = self.stack.get(idx).unwrap();
        self.stack.push(val);
    }

    fn replace(&mut self, idx: isize) {
        let val = self.stack.pop();
        self.stack.set(idx, val);
    }

    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1);
    }

    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        let m = if n >= 0 { -n } else { idx-n };
        self.stack.reverse(idx, m-1);
        self.stack.reverse(m, -1);
        self.stack.reverse(idx, -1);
    }

    fn set_top(&mut self, idx: isize) {
        self.stack.set_top(idx);
    }

    fn type_name(&self, tp: i8) -> &str {
        match tp {
            consts::LUA_TNONE => "no value",
            consts::LUA_TNIL => "nil",
            consts::LUA_TBOOLEAN => "boolean",
            consts::LUA_TNUMBER => "number",
            consts::LUA_TSTRING => "string",
            consts::LUA_TTABLE => "table",
            consts::LUA_TFUNCTION => "function",
            consts::LUA_TTHREAD => "thread",
            consts::LUA_TUSERDATA => "userdata",
            _ => unreachable!()
        }
    }

    fn type_id(&self, idx: isize) -> i8 {
        if let Some(val) = self.stack.get(idx){
            val.ty()
        }
        else {
            consts::LUA_TNONE
        }
    }

    fn is_none(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TNONE
    }

    fn is_nil(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TNIL
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TNONE || self.type_id(idx) == consts::LUA_TNIL
    }

    fn is_boolean(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TBOOLEAN
    }
    
    fn is_string(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TSTRING || self.type_id(idx) == consts::LUA_TNUMBER
    }

    fn is_function(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TFUNCTION
    }

    fn is_table(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TTABLE
    }

    fn is_thread(&self, idx: isize) -> bool {
        self.type_id(idx) == consts::LUA_TTHREAD
    }

    fn is_number(&self, idx: isize) -> bool {
        self.to_numberx(idx).is_some()
    }

    fn is_integer(&self, idx: isize) -> bool {
        if let LuaValue::Integer(_) = self.stack.get(idx).unwrap() {
            true
        }
        else {
            false
        }
    }

    fn to_boolean(&self, idx: isize) -> bool {
        match self.stack.get(idx).unwrap() {
            LuaValue::Nil => false,
            LuaValue::Boolean(b) => b,
            _ => true
        }
    }

    fn to_number(&self, idx: isize) -> f64 {
        self.to_numberx(idx).unwrap()
    }

    fn to_numberx(&self, idx: isize) -> Option<f64> {
        match self.stack.get(idx).unwrap() {
            LuaValue::Number(n) => Some(n),
            LuaValue::Integer(i) => Some(i as f64),
            _ => None
        }
    }

    fn to_integer(&self, idx: isize) -> i64 {
        self.to_integerx(idx).unwrap()
    }

    fn to_integerx(&self, idx: isize) -> Option<i64> {
        match self.stack.get(idx).unwrap() {
            LuaValue::Integer(i) => Some(i),
            _ => None,
        }
    }

    fn to_string(&self, idx: isize) -> String {
        self.to_stringx(idx).unwrap()
    }

    fn to_stringx(&self, idx: isize) -> Option<String> {
        match self.stack.get(idx).unwrap() {
            LuaValue::Str(s) => Some(s),
            LuaValue::Number(n) => Some(n.to_string()),
            LuaValue::Integer(i) => Some(i.to_string()),
            _ => None,
        }
    }

    fn push_nil(&mut self) {
        self.stack.push(LuaValue::Nil);
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack.push(LuaValue::Boolean(b));
    }

    fn push_integer(&mut self, n: i64) {
        self.stack.push(LuaValue::Integer(n));
    }

    fn push_number(&mut self, n: f64) {
        self.stack.push(LuaValue::Number(n));
    }

    fn push_string(&mut self, s: String) {
        self.stack.push(LuaValue::Str(s));
    }

    fn arith(&mut self, op: u8) {
        let b = if op != consts::LUA_OPUNM && op != consts::LUA_OPBNOT { self.stack.pop() } else { LuaValue::Integer(0) };
        let a = self.stack.pop();

        if let Some(result) = _arith(&a, &b, op) {
            self.stack.push(result);
        } else {
            panic!("arithmetic error!");
        }
    }

    fn compare(&self, idx1: isize, idx2: isize, op: u8) -> bool {
        if let Some(a) = self.stack.get(idx1) {
            if let Some(b) = self.stack.get(idx2) {
                match op {
                    consts::LUA_OPEQ => return _eq(&a, &b),
                    consts::LUA_OPLT => return _lt(&a, &b).unwrap(),
                    consts::LUA_OPLE => return _le(&a, &b).unwrap(),
                    _ => panic!()
                }
            }
        }
        panic!()
    }

    fn len(&mut self, idx: isize) {
        if let Some(val) = self.stack.get(idx) {
            let _len = match val {
                LuaValue::Str(s) => LuaValue::Integer(s.len() as i64),
                LuaValue::Table(t) => LuaValue::Integer(t.borrow().len() as i64),
                _ => panic!("length error!")
            };
            self.stack.push(_len);
        }
    }

    fn concat(&mut self, n: isize) {
        if n == 0 {
            self.stack.push(LuaValue::Str(String::new()));
        } else if n >= 2 {
            for _ in 1..n {
                if self.is_string(-1) && self.is_string(-2) {
                    let s2: String = self.to_string(-1);
                    let mut s1 = self.to_string(-2);
                    s1.push_str(&s2);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(LuaValue::Str(s1));
                } else {
                    panic!("concatenation error!");
                }
            }
        }
    }

    fn new_table(&mut self) {
        self.create_table(0, 0);
    }

    fn create_table(&mut self, n_arr: usize, n_rec: usize) {
        let table = LuaTable::new(n_arr, n_rec);
        self.stack.push(LuaValue::Table(Rc::new(RefCell::new(table))));
    }

    fn get_table(&mut self, idx: isize) -> i8 {
        if let Some(t) = self.stack.get(idx) {
            let k = self.stack.pop();
            if let LuaValue::Table(tbl) = t {
                let v = tbl.borrow().get(&k);
                let vty = v.ty();
                self.stack.push(v);
                return vty;
            } 
        }
        panic!()
    }

    fn get_field(&mut self, idx: isize, k: &str) -> i8 {
        if let Some(t) = self.stack.get(idx) {
            if let LuaValue::Table(tbl) = t {
                let v = tbl.borrow().get(&LuaValue::Str(k.to_string()));
                let vty = v.ty();
                self.stack.push(v);
                return vty;
            } 
        }
        panic!()
    }

    fn get_i(&mut self, idx: isize, i: i64) -> i8 {
        if let Some(t) = self.stack.get(idx) {
            if let LuaValue::Table(tbl) = t {
                let v = tbl.borrow().get(&LuaValue::Integer(i));
                let vty = v.ty();
                self.stack.push(v);
                return vty;
            } 
        }
        panic!()
    }

    fn set_table(&mut self, idx: isize) {
        if let Some(t) = self.stack.get(idx) {
            let v = self.stack.pop();
            let k = self.stack.pop();
            if let LuaValue::Table(tbl) = t {
                tbl.borrow_mut().put(&k, &v);
                return;
            }
        }
        panic!()
    }

    fn set_field(&mut self, idx: isize, k: &str) {
        if let Some(t) = self.stack.get(idx) {
            let v = self.stack.pop();
            if let LuaValue::Table(tbl) = t {
                tbl.borrow_mut().put(&LuaValue::Str(k.to_string()), &v);
                return;
            }
        }
    }

    fn set_i(&mut self, idx: isize, i: i64) {
        if let Some(t) = self.stack.get(idx) {
            let v = self.stack.pop();
            if let LuaValue::Table(tbl) = t {
                tbl.borrow_mut().put(&LuaValue::Integer(i), &v);
                return;
            }
        }
    }



}

impl LuaVM for LuaState {
    fn pc(&self) -> isize {
        self.pc
    }

    fn add_pc(&mut self, n: isize) {
        self.pc += n;
    }

    fn fetch(&mut self) -> u32 {
        let instr = self.proto.code[self.pc as usize];
        self.pc += 1;
        instr
    }

    fn get_const(&mut self, idx: isize) {
        let c = &self.proto.constants[idx as usize];
        let val = match c {
            Constant::Nil => LuaValue::Nil,
            Constant::Boolean(b) => LuaValue::Boolean(*b),
            Constant::Integer(i) => LuaValue::Integer(*i),
            Constant::Number(n) => LuaValue::Number(*n),
            Constant::Str(s) => LuaValue::Str((*s).clone())
        };
        self.stack.push(val);
    }

    fn get_rk(&mut self, rk: isize) {
        if rk > 0xff {
            self.get_const(rk & 0xff);
        } else {
            self.push_value(rk + 1);
        }
    }
}
