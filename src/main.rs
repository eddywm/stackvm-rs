use crate::lib::IADD;

mod lib;
fn main() {
    let iadd = lib::inst_mapping(IADD);

    println!("Instruction : {}", iadd.str());
}
