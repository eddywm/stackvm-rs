mod bytecode;
fn main() {
    let iadd = bytecode::Instruction{ name: String::from("iadd"), agrs: 0 };

    println!("Instruction : {}", iadd.str());
}
