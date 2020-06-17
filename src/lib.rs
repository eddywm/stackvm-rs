
// Integers basic ops (Addition, Subtraction & Multiplication)
pub static IADD: i16 = 1;
pub static ISUB: i16 = 2;
pub static IMULT: i16 = 3;

// Comparison (Less Than & Less Than)
pub static ILET: i16 = 4;
pub static IEQ : i16= 5;

// Branching (Branch; Branch if true & if false)
pub static BR : i16= 6;
pub static BRT: i16 = 7;
pub static BRF: i16 = 8;

pub static ICONST: i16 = 9; // Push constant integer
pub static LOAD: i16 = 10; // Load from local context
pub static GLOAD: i16 = 11; // Load from global memory
pub static STORE: i16 = 12; // Store in local context
pub static GSTORE: i16 = 13; // Store in global memory

pub static PRINT: i16 = 14; // Print stack top
pub static POP: i16 = 15;// Throw away top of stack

// Return with/without value
pub static CALL: i16 = 16;
pub static RET : i16= 17;


pub struct Instruction {
    pub name: String,
    // Instruction name
    pub agrs: i8, // Number of arguments
}

impl Instruction {
    pub fn str(&self) -> String {
        return format!("| name:{} args: {} |", self.name, self.agrs);
    }

    pub fn get_all(_opcode: i16) -> Vec<Instruction> {
        return vec![];
    }
}

pub fn get_instruction(opcode: i16) -> Instruction {
    match opcode {
        IADD => Instruction { name: String::from("iadd"), agrs: 0 },

        _ => Instruction { name: String::from("nil"), agrs: 0 }
    }
}