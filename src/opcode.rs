#[derive(Debug, Enum)]
enum OpCode {
    // Integers ops (Addition, Subtraction & Multiplication)
    IADD =  1,
    ISUB =  2,
    IMULT = 3,

    // Comparison (Less Than & Less Than)
    ILET =  4,
    IEQ =   5,

    // Branching (Branch, Branch if true & if false)
    BR =    6,
    BRT =   7,
    BRF =   8,
}
