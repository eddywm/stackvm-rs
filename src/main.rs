use crate::lib::IADD;

mod lib;
fn main() {
    let iadd = lib::get_instruction(IADD);

    println!("Instruction : {}", iadd.str());
}
