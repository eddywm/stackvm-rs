/// This is a single file implementation of a fairly simple stack based VM
/// The goal is to support the common basic primitives operations like
/// iadd, isub, imult, comparisons, branching, memory ops, etc, ....
///
///          __
///  _(\    |@@|
/// (__/\__ \--/ __
///    \___|----|  |   __
///        \ }{ /\ )_ / _\
///        /\__/\ \__O (__
///       (--/\--)    \__/
///        _ )(  )(_
///      `---''---`
///
/// /////////////////////////////////////////////////////////////////////


// Integers basic ops (Addition, Subtraction & Multiplication)
pub const IADD: i8 = 1;
pub const ISUB: i8 = 2;
pub const IMULT: i8 = 3;

// Comparison (Less Than & Less Than)
pub const ILET: i8 = 4;
pub const IEQ : i8= 5;

// Branching (Branch; Branch if true & if false)
pub const BR : i8= 6;
pub const BRT: i8 = 7;
pub const BRF: i8 = 8;

pub const ICONST: i8 = 9; // Push constant integer
pub const LOAD: i8 = 10; // Load from local context
pub const GLOAD: i8 = 11; // Load from global memory
pub const STORE: i8 = 12; // Store in local context
pub const GSTORE: i8 = 13; // Store in global memory

pub const PRINT: i8 = 14; // Print stack top
pub const POP: i8 = 15;// Throw away top of stack

// Return with/without value
pub const CALL: i8 = 16;
pub const RET : i8= 17;
pub const HALT : i8= 18;


pub struct Instruction {
    pub name: String,
    // Instruction name
    pub agrs: i8, // Number of arguments
}

impl Instruction {
    pub fn str(&self) -> String {
        return format!("[name:{} args: {}]", self.name, self.agrs);
    }
}

pub fn inst_new(_name: &str, _args: i8) -> Instruction {
    return Instruction { name: _name.to_string(), agrs: _args };
}

// Instructions mapping
pub fn inst_mapping(_opcode: i8) -> Instruction {
    match _opcode {
        IADD => inst_new("iadd", 0),
        ISUB => inst_new("isub", 0),
        IMULT => inst_new("imult", 0),

        ILET => inst_new("ilet", 0),
        IEQ => inst_new("ieq", 0),
        BR => inst_new("ieq", 1),
        BRT => inst_new("brt", 1),
        BRF => inst_new("brf", 1),

        ICONST => inst_new("iconst", 1),
        LOAD => inst_new("load", 1),
        GLOAD => inst_new("gload", 1),
        STORE => inst_new("store", 1),
        GSTORE => inst_new("gstore", 1),

        PRINT => inst_new("print", 0),
        POP => inst_new("pop", 0),
        CALL => inst_new("call", 1),

        RET => inst_new("ret", 0),
        HALT => inst_new("hat", 0),

        _ => inst_new("nil", 0 )
    }
}


pub struct FuncMetadata {
    pub name: String,
    pub nargs: i8,
    pub nlocals: i8,
    pub address: i8 // byte-code address
}

pub struct Context {
    pub context: Box<Context>,
    pub metadata: FuncMetadata,
    pub returnip: i32, // Return instruction pointer
    pub locals: Vec<i8>
}

const  DEFAULT_STACK_SIZE: i32 = 1000;
const  DEFAULT_CALL_STACK_SIZE: i32 = 1000;
const  FALSE: i32 = 0;
const  TRUE: i32 = 1;

pub struct VM {
    pub ipr: i32, // Instruction pointer register
    pub spr: i32, // Stack pointer register

    pub code_memory: Vec<i8>,
    pub globals_space: Vec<i8>,
    pub operand_stack: Vec<i8>,

    pub context: Context,
    pub metadata: Vec<FuncMetadata>,

    pub is_trace_enabled: bool
}

#[test]
fn test_instruction_byte_mapping() {
    assert_eq!(IADD, 1);
    assert_eq!(IEQ, 5);
    assert_eq!(BRF, 8);
    assert_eq!(GSTORE, 13);
    assert_eq!(RET, 17);
}

#[test]
fn test_instruction_mapping() {
    let iadd = inst_mapping(IADD);
    assert_eq!(iadd.name,   String::from("iadd"));

    let gstore = inst_mapping(GSTORE);
    assert_eq!(gstore.name,   String::from("gstore"));
}