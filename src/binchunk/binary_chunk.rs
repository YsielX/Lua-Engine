pub struct Header {
    pub signature: [u8;4],
    pub version: u8,
    pub format: u8,
    pub luac_data: [u8;6],
    pub cint_size: u8,
    pub sizet_size: u8,
    pub instruction_size: u8,
    pub lua_integer_size: u8,
    pub lua_number_size: u8,
    pub luac_int: i64,
    pub luac_num: f64,
}

pub struct Prototype {
    pub source: String,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<Box<Prototype>>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<String>,
}

pub struct BinaryChunk {
    pub header: Header,
    pub size_upvalues: u8,
    pub main_func: Box<Prototype>
}

pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String)
}

pub struct Upvalue {
    pub instack: u8,
    pub idx: u8
}

pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32
}

