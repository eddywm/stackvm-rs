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

#[derive(Debug, PartialEq)]
pub enum Opcode {
    // Integers basic ops (Addition, Subtraction & Multiplication)
    IADD,
    ISUB,
    IMULT,

    // Comparison (Less Than & Less Than)
    ILET,
    IEQ,

    // Branching (Branch, Branch if true & if false)
    BR,
    BRT,
    BRF,

    // Push constant integer
    ICONST,

    // Load from local context
    LOAD,

    // Load from global memory
    GLOAD,

    // Store in local context
    STORE,

    // Store in global memory
    GSTORE,

    // Print stack top
    PRINT,

    // Throw away top of stack
    POP,

    // Return with/without value
    CALL,
    RET,
    HALT,

    // Illegacl opcode
    IGL,
}

impl From<u8> for Opcode {
     fn from(byte: u8) -> Self {
        match byte {
            1 => Opcode::IADD,
            2 => Opcode::ISUB,
            3 => Opcode::IMULT,
            4 => Opcode::ILET,
            5 => Opcode::IEQ,
            _ => Opcode::IGL
        }
    }
}


pub struct Instruction {
    // Instruction name
    pub name: String,

    // Number of arguments
    pub agrs: u8,
}

impl Instruction {
    pub fn str(&self) -> String {
        return format!("[name:{} args: {}]", self.name, self.agrs);
    }
    pub fn new(_name: &str, _args: u8) -> Instruction {
        return Instruction { name: _name.to_string(), agrs: _args };
    }
}

// Instructions mapping
pub fn inst_mapping(_opcode: Opcode) -> Instruction {
    return match _opcode {
        Opcode::IADD => Instruction::new("iadd", 0),
        Opcode::ISUB => Instruction::new("isub", 0),
        Opcode::IMULT => Instruction::new("imult", 0),

        Opcode::ILET => Instruction::new("ilet", 0),
        Opcode::IEQ => Instruction::new("ieq", 0),
        Opcode::BR => Instruction::new("ieq", 1),
        Opcode::BRT => Instruction::new("brt", 1),
        Opcode::BRF => Instruction::new("brf", 1),

        Opcode::ICONST => Instruction::new("iconst", 1),
        Opcode::LOAD => Instruction::new("load", 1),
        Opcode::GLOAD => Instruction::new("gload", 1),
        Opcode::STORE => Instruction::new("store", 1),
        Opcode::GSTORE => Instruction::new("gstore", 1),

        Opcode::PRINT => Instruction::new("print", 0),
        Opcode::POP => Instruction::new("pop", 0),
        Opcode::CALL => Instruction::new("call", 1),

        Opcode::RET => Instruction::new("ret", 0),
        Opcode::HALT => Instruction::new("hat", 0),
        _ => Instruction::new("igl", 0)
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
        assert_eq!(Opcode::IADD, Opcode::from(0x1));
        assert_eq!(Opcode::IEQ, Opcode::from(0x5));
//        assert_eq!(BRF, 8);
//        assert_eq!(GSTORE, 13);
//        assert_eq!(RET, 17);
    }

    #[test]
    fn test_instruction_mapping() {
        let iadd = inst_mapping(Opcode::IADD);
        assert_eq!(iadd.name, String::from("iadd"));

        let gstore = inst_mapping(Opcode::GSTORE);
        assert_eq!(gstore.name, String::from("gstore"));
    }
}