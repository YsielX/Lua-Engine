use std::collections::HashMap;

use super::lua_value::LuaValue;

#[derive(PartialEq, Clone)]
pub struct LuaTable {
    arr: Vec<LuaValue>,
    map: HashMap<LuaValue, LuaValue>
}

impl LuaTable {
    pub fn new(n_arr: usize, n_rec: usize) -> LuaTable {
        LuaTable { arr: Vec::with_capacity(n_arr), map: HashMap::with_capacity(n_rec) }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn get(&self, key: &LuaValue) -> LuaValue {
        if let Some(idx) = to_index(key) {
            if idx <= self.arr.len() {
                return self.arr[idx-1].clone();
            }
        }
        if let Some(val) = self.map.get(&key) {
            return val.clone();
        } else {
            return LuaValue::Nil;
        }
    }

    pub fn put(&mut self, key: &LuaValue, val: &LuaValue) {
        if key.is_nil() {
            panic!("table index is nil!");
        }
        if let LuaValue::Number(n) = key {
            if n.is_nan() {
                panic!("table index is NaN!");
            }
        }

        if let Some(idx) = to_index(&key) {
            let arr_len = self.arr.len();
            if idx <= arr_len {
                let val_is_nil = val.is_nil();
                self.arr[idx - 1] = val.clone();
                if idx == arr_len && val_is_nil {
                    self.shrink_array();
                }
                return;
            }
            if idx == arr_len + 1 {
                self.map.remove(&key);
                if !val.is_nil() {
                    self.arr.push(val.clone());
                    self.expand_array();
                }
                return;
            }
        }

        if !val.is_nil() {
            self.map.insert(key.clone(), val.clone());
        } else {
            self.map.remove(&key);
        }
    }

    pub fn shrink_array(&mut self) {
        while !self.arr.is_empty() {
            if self.arr.last().unwrap().is_nil() {
                self.arr.pop();
            } else {
                break;
            }
        }
    }

    pub fn expand_array(&mut self) {
        let mut idx = self.arr.len() + 1;
        loop {
            let key = LuaValue::Integer(idx as i64);
            if self.map.contains_key(&key) {
                let val = self.map.remove(&key).unwrap();
                self.arr.push(val);
                idx += 1;
            } else {
                break;
            }
        }
    }
}

pub fn float_to_integer(n: f64) -> Option<i64> {
    let i = n as i64;
    if i as f64 == n {
        Some(i)
    } else {
        None
    }
}

pub fn to_index(key: &LuaValue) -> Option<usize> {
    if let LuaValue::Integer(i) = key {
        if *i >= 1 {
            return Some(*i as usize);
        }
    } else if let LuaValue::Number(n) = key {
        if let Some(i) = float_to_integer(*n) {
            if i >= 1 {
                return Some(i as usize);
            }
        }
    }
    None
}
