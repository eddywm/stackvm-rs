pub struct Instruction {
    pub name: String, // Instruction name
    pub agrs: i8, // Number of arguments
}

impl Instruction {
    pub fn str(&self) -> String {
        return format!("| name:{} args: {} |", self.name, self.agrs);
    }

    pub fn get_all(_opcode: i16) -> Vec<Instruction>{
        return vec![]
    }
}