#[derive(Copy, Clone)]
pub struct Instruction<'a> {
    pub name: &'a str, // Instruction name
    pub agrs: i8, // Number of arguments
}

impl Instruction<'_> {
    pub fn str(&self) -> String {
        return format!("name:{} args: {}", self.name, self.agrs);
    }
}