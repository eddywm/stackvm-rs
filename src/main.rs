mod lib;
fn main() {
    let iadd = lib::Instruction{ name: String::from("iadd"), agrs: 0 };

    println!("Instruction : {}", iadd.str());
}
