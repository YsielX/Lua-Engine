use crate::binchunk::binary_chunk::{Constant,Prototype,Upvalue,LocVar};
use crate::binchunk::header_const;
use crate::binchunk::tag_const;


pub struct Reader {
    pub data: Vec<u8>
}

impl Reader {
    pub fn read_byte(&mut self) -> u8 {
        let b = self.data[0];
        self.data.remove(0);
        b
    }

    pub fn read_uint32(&mut self) -> u32 {
        let a0 = self.read_byte() as u32;
        let a1 = self.read_byte() as u32;
        let a2 = self.read_byte() as u32;
        let a3 = self.read_byte() as u32;
        (a3 << 24) | (a2 << 16) | (a1 << 8) | a0
    }

    pub fn read_uint64(&mut self) -> u64 {
        let a0 = self.read_uint32() as u64;
        let a1 = self.read_uint32() as u64;
        (a1 << 32) | a0
    }

    pub fn read_int64(&mut self) -> i64 {
        self.read_uint64() as i64
    }

    pub fn read_float64(&mut self) -> f64 {
        use std::f64;
        f64::from_bits(self.read_uint64())
    }

    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut vec = Vec::new();
        for _ in 0..n {
            vec.push(self.read_byte());
        }
        vec
    }

    pub fn read_string(&mut self) -> String {
        let mut size = self.read_byte() as usize;
        if size == 0 {
            return String::from("");
        }
        if size == 0xff {
            size = self.read_uint64() as usize;
        }
        let bytes = self.read_bytes(size - 1);
        String::from_utf8(bytes).unwrap()
    }

    pub fn read_constant(&mut self) -> Constant {
        match self.read_byte() {
            tag_const::TAG_NIL => Constant::Nil,
            tag_const::TAG_BOOLEAN => Constant::Boolean(self.read_byte() != 0),
            tag_const::TAG_INTEGER => Constant::Integer(self.read_int64()),
            tag_const::TAG_NUMBER => Constant::Number(self.read_float64()),
            tag_const::TAG_SHORT_STR => Constant::Str(self.read_string()),
            tag_const::TAG_LONG_STR => Constant::Str(self.read_string()),
            _ => unreachable!()
        }
    }

    pub fn read_upvalue(&mut self) -> Upvalue {
        Upvalue {
            instack: self.read_byte(),
            idx: self.read_byte()
        }
    }

    pub fn read_loc_var(&mut self) -> LocVar {
        LocVar {
            var_name: self.read_string(),
            start_pc: self.read_uint32(),
            end_pc: self.read_uint32()
        }
    }

    pub fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let n = self.read_uint32() as usize;
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(f(self));
        }
        vec
    }

    pub fn check_header(&mut self) {
        assert_eq!(self.read_bytes(4), header_const::LUA_SIGNATURE, "Not a precompiled chunk!");
        assert_eq!(self.read_byte(), header_const::LUAC_VERSION, "Version mismatch!");
        assert_eq!(self.read_byte(), header_const::LUAC_FORMAT, "Format mismatch!");
        assert_eq!(self.read_bytes(6), header_const::LUAC_DATA, "Corrupted!");
        assert_eq!(self.read_byte(), header_const::CINT_SIZE, "int size mismamtch!");
        assert_eq!(self.read_byte(), header_const::CSIZET_SIZE, "size_t size mismatch!");
        assert_eq!(self.read_byte(), header_const::INSTRUCTION_SIZE, "Instruction size mismatch!");
        assert_eq!(self.read_byte(), header_const::LUA_INTEGER_SIZE, "Lua Integer size mismatch!");
        assert_eq!(self.read_byte(), header_const::LUA_NUMBER_SIZE, "Lua Number size mismatch!");
        assert_eq!(self.read_int64(), header_const::LUAC_INT, "Endianness mismatch!");
        assert_eq!(self.read_float64(), header_const::LUAC_NUM, "Float format mismatch!");
    }

    pub fn read_proto(&mut self, parent_source: &String) -> Box<Prototype> {
        let mut source = self.read_string();
        if source == String::from("") {
            source = parent_source.to_string();
        }
        Box::new(Prototype {
            source,
            line_defined: self.read_uint32(),
            last_line_defined: self.read_uint32(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_stack_size: self.read_byte(),
            code: self.read_vec(|r| r.read_uint32()),
            constants: self.read_vec(|r| r.read_constant()),
            upvalues: self.read_vec(|r| r.read_upvalue()),
            protos: self.read_vec(|r| r.read_proto(&parent_source)),
            line_info: self.read_vec(|r| r.read_uint32()),
            loc_vars: self.read_vec(|r| r.read_loc_var()),
            upvalue_names: self.read_vec(|r| r.read_string())
        })
            
    }
}