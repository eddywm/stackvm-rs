
// Integers basic ops (Addition, Subtraction & Multiplication)
pub const IADD: i16 = 1;
pub const ISUB: i16 = 2;
pub const IMULT: i16 = 3;

// Comparison (Less Than & Less Than)
pub const ILET: i16 = 4;
pub const IEQ : i16= 5;

// Branching (Branch; Branch if true & if false)
pub const BR : i16= 6;
pub const BRT: i16 = 7;
pub const BRF: i16 = 8;

pub const ICONST: i16 = 9; // Push constant integer
pub const LOAD: i16 = 10; // Load from local context
pub const GLOAD: i16 = 11; // Load from global memory
pub const STORE: i16 = 12; // Store in local context
pub const GSTORE: i16 = 13; // Store in global memory

pub const PRINT: i16 = 14; // Print stack top
pub const POP: i16 = 15;// Throw away top of stack

// Return with/without value
pub const CALL: i16 = 16;
pub const RET : i16= 17;


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

pub fn instruction_new(_name: &str, _args: i8) -> Instruction {
    return Instruction { name: _name.to_string(), agrs: _args };
}

// Instructions mapping
pub fn inst_mapping(_opcode: i16) -> Instruction {
    match _opcode {
        IADD => instruction_new("iadd", 0),
        ISUB => instruction_new("isub", 0),
        IMULT => instruction_new("imult", 0),

        ILET => instruction_new("ilet", 0),
        IEQ => instruction_new("ieq", 0),
        BR => instruction_new("ieq", 1),
        BRT => instruction_new("brt", 1),
        BRF => instruction_new("brf", 1),

        ICONST => instruction_new("iconst", 1),
        LOAD => instruction_new("load", 1),
        GLOAD => instruction_new("gload", 1),
        STORE => instruction_new("store", 1),
        GSTORE => instruction_new("gstore", 1),

        _ => instruction_new("nil", 0 )
    }
}


#[test]
fn test_instruction_mapping() {
    let iadd = inst_mapping(IADD);
    assert_eq!(iadd.name,   String::from("iadd"))
}