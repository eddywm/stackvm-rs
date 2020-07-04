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
pub const IEQ: i8 = 5;

// Branching (Branch; Branch if true & if false)
pub const BR: i8 = 6;
pub const BRT: i8 = 7;
pub const BRF: i8 = 8;

// Push constant integer
pub const ICONST: i8 = 9;

// Load from local context
pub const LOAD: i8 = 10;

// Load from global memory
pub const GLOAD: i8 = 11;

// Store in local context
pub const STORE: i8 = 12;

// Store in global memory
pub const GSTORE: i8 = 13;

// Print stack top
pub const PRINT: i8 = 14;

// Throw away top of stack
pub const POP: i8 = 15;

// Return with/without value
pub const CALL: i8 = 16;
pub const RET: i8 = 17;
pub const HALT: i8 = 18;


pub struct Instruction {
    // Instruction name
    pub name: String,

    // Number of arguments
    pub agrs: i8,
}

impl Instruction {
    pub fn str(&self) -> String {
        return format!("[name:{} args: {}]", self.name, self.agrs);
    }
    pub fn new(_name: &str, _args: i8) -> Instruction {
        return Instruction { name: _name.to_string(), agrs: _args };
    }
}

// Instructions mapping
pub fn inst_mapping(_opcode: i8) -> Instruction {
    match _opcode {
        IADD => Instruction::new("iadd", 0),
        ISUB => Instruction::new("isub", 0),
        IMULT => Instruction::new("imult", 0),

        ILET => Instruction::new("ilet", 0),
        IEQ => Instruction::new("ieq", 0),
        BR => Instruction::new("ieq", 1),
        BRT => Instruction::new("brt", 1),
        BRF => Instruction::new("brf", 1),

        ICONST => Instruction::new("iconst", 1),
        LOAD => Instruction::new("load", 1),
        GLOAD => Instruction::new("gload", 1),
        STORE => Instruction::new("store", 1),
        GSTORE => Instruction::new("gstore", 1),

        PRINT => Instruction::new("print", 0),
        POP => Instruction::new("pop", 0),
        CALL => Instruction::new("call", 1),

        RET => Instruction::new("ret", 0),
        HALT => Instruction::new("hat", 0),

        _ => Instruction::new("nil", 0)
    }
}


pub struct FuncMetadata {
    pub name: String,
    pub nargs: i8,
    pub nlocals: i8,

    // byte-code address
    pub address: i8,
}

pub struct Context {
    pub context: Box<Context>,
    pub metadata: FuncMetadata,

    // Return instruction pointer
    pub returnip: i32,

    pub locals: Vec<i8>,
}

const DEFAULT_STACK_SIZE: i32 = 1000;
const DEFAULT_CALL_STACK_SIZE: i32 = 1000;
const FALSE: i32 = 0;
const TRUE: i32 = 1;

pub struct VM {
    // Instruction pointer register
    pub ipr: i32,

    // Stack pointer register
    pub spr: i32,

    pub code_memory: Vec<i8>,
    pub globals_space: Vec<i8>,
    pub operand_stack: Vec<i8>,

    pub context: Context,
    pub metadata: Vec<FuncMetadata>,

    pub is_trace_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(iadd.name, String::from("iadd"));

        let gstore = inst_mapping(GSTORE);
        assert_eq!(gstore.name, String::from("gstore"));
    }
}