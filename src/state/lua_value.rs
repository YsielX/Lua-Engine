use crate::api::consts;
use std::cell::RefCell;
use std::fmt;
use std::ptr;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use super::lua_table::LuaTable;

#[derive(Clone, PartialEq)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
    Table(Rc<RefCell<LuaTable>>)
}

impl LuaValue {
    pub fn ty(&self) -> i8 {
        match self {
            LuaValue::Nil => consts::LUA_TNIL,
            LuaValue::Boolean(_) => consts::LUA_TBOOLEAN,
            LuaValue::Integer(_) => consts::LUA_TNUMBER,
            LuaValue::Number(_) => consts::LUA_TNUMBER,
            LuaValue::Str(_) => consts::LUA_TSTRING,
            LuaValue::Table(_) => consts::LUA_TTABLE
        }
    }

    pub fn _to_boolean(&self) -> bool {
        match self {
            LuaValue::Nil => false,
            LuaValue::Boolean(b) => *b, // TODO
            _ => true,
        }
    }

    pub fn to_number(&self) -> Option<f64> {
        match self {
            LuaValue::Integer(i) => Some(*i as f64),
            LuaValue::Number(n) => Some(*n),
            LuaValue::Str(s) => s.parse::<f64>().ok(), // TODO
            _ => None,
        }
    }

    pub fn to_integer(&self) -> Option<i64> {
        match self {
            LuaValue::Integer(i) => Some(*i),
            LuaValue::Number(n) => float_to_integer(*n),
            LuaValue::Str(s) => string_to_integer(s),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            LuaValue::Nil => true,
            _ => false,
        }
    }
}

fn float_to_integer(n: f64) -> Option<i64> {
    let i = n as i64;
    if i as f64 == n {
        Some(i)
    } else {
        None
    }
}

fn string_to_integer(s: &String) -> Option<i64> {
    if let Ok(i) = s.parse::<i64>() {
        Some(i)
    } else if let Ok(n) = s.parse::<f64>() {
        float_to_integer(n)
    } else {
        None
    }
}

// impl PartialEq for LuaValue {
//     fn eq(&self, other: &LuaValue) -> bool {
//         if let (LuaValue::Nil, LuaValue::Nil) = (self, other) {
//             true
//         } else if let (LuaValue::Boolean(x), LuaValue::Boolean(y)) = (self, other) {
//             x == y
//         } else if let (LuaValue::Integer(x), LuaValue::Integer(y)) = (self, other) {
//             x == y
//         } else if let (LuaValue::Number(x), LuaValue::Number(y)) = (self, other) {
//             x == y // TODO
//         } else if let (LuaValue::Str(x), LuaValue::Str(y)) = (self, other) {
//             x == y
//         } else if let (LuaValue::Table(x), LuaValue::Table(y)) = (self, other) {
//             x == y
//         } else {
//             false
//         }
//     }
// }

// the trait `std::cmp::Eq` is not implemented for `f64`
impl Eq for LuaValue {} // TODO

// // the trait `std::hash::Hash` is not implemented for `f64`
impl Hash for LuaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::Nil => 0.hash(state),
            LuaValue::Boolean(b) => b.hash(state),
            LuaValue::Integer(i) => i.hash(state),
            LuaValue::Number(n) => n.to_bits().hash(state),
            LuaValue::Str(s) => s.hash(state),
            LuaValue::Table(t) => ptr::hash(t, state),
        }
    }
}

impl fmt::Debug for LuaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuaValue::Nil => write!(f, "(nil)"),
            LuaValue::Boolean(b) => write!(f, "({})", b),
            LuaValue::Integer(i) => write!(f, "({})", i),
            LuaValue::Number(n) => write!(f, "({})", n),
            LuaValue::Str(s) => write!(f, "({})", s),
            LuaValue::Table(_) => write!(f, "(table)"),
        }
    }
}
