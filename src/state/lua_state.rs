use crate::api::consts;
use super::lua_stack::LuaStack;
use super::lua_value::LuaValue;
use crate::api::lua_state::LuaAPI;

#[derive(Debug)]
pub struct LuaState {
    stack: LuaStack
}

impl LuaState {
    pub fn new() -> LuaState {
        LuaState { stack: LuaStack::new(20) }
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
        let m = if n >= 0 { idx + n } else { n-1 };
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
}
