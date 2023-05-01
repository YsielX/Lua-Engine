use super::lua_value::LuaValue;

#[derive(Clone, Debug)]
pub struct LuaStack {
    pub slots: Vec<LuaValue>,
    pub top: usize,  // equal to the length of vector
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        LuaStack {
            slots: Vec::with_capacity(size),
            top: 0,
        }
    }

    pub fn _check(&mut self, n: usize) {
        self.slots.reserve(n);
    }

    pub fn set_top(&mut self, idx: isize) {
        let new_top = if idx<0 {self.abs_index(idx).unwrap() + 1} else {idx as usize}; // if start with 0, it may change

        for _ in new_top..self.top {
            self.pop();
        }
        for _ in self.top..new_top {
            self.push(LuaValue::Nil);
        }
    }

    pub fn push(&mut self, val: LuaValue) {
        self.slots.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> LuaValue {
        self.top -= 1;
        self.slots.pop().expect("LuaStack is empty!")
    }

    pub fn abs_index(&self, idx: isize) -> Option<usize> {
        // parameter index start with 1 => return index start with 0
        if idx >= 0 && (idx as usize - 1) < self.top {
            Some(idx as usize - 1)
        } else if (idx + self.top as isize) >= 0 {
            Some((idx + self.top as isize) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, idx: isize) -> Option<LuaValue> {
        if let Some(u_idx) = self.abs_index(idx) {
            Some(self.slots[u_idx].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        let i = self.abs_index(idx).expect("invalid index!");
        self.slots[i] = val;
    }

    pub fn reverse(&mut self, from: isize, to: isize) {
        let mut from_abs = self.abs_index(from).unwrap();
        let mut to_abs = self.abs_index(to).unwrap();
        while from_abs < to_abs {
            self.slots.swap(from_abs, to_abs);
            from_abs += 1;
            to_abs -= 1;
        }
    }
}
