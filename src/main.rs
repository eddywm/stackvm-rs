mod bytecode;
fn main() {
    let iadd = bytecode::Instruction{ name: "iadd", agrs: 0 };

    println!("Instruction : {}", iadd.str());
}
