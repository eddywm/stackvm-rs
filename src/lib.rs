
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

pub fn get_instruction(opcode: i16) -> Instruction {
    match opcode {
        IADD => Instruction { name: String::from("iadd"), agrs: 0 },
        ISUB => Instruction { name: String::from("isub"), agrs: 0 },
        IMULT => Instruction { name: String::from("imult"), agrs: 0 },
        ILET => Instruction { name: String::from("ilet"), agrs: 0 },
        IEQ => Instruction { name: String::from("ieq"), agrs: 0 },
        BR => Instruction { name: String::from("ieq"), agrs: 1 },
        BRT => Instruction { name: String::from("ieq"), agrs: 1 },

        _ => Instruction { name: String::from("nil"), agrs: 0 }
    }
}