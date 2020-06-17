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

    ICONST = 9,   // Push constant integer
    LOAD   = 10,  // Load from local context
    GLOAD  = 11,  // Load from global memory
    STORE  = 12,  // Store in local context
    GSTORE = 13,  // Store in global memory
    PRINT  = 14,  // Print stack top
    POP  =   15,  // Throw away top of stack
    CALL =   16,
    RET  =   17,  // Return with/without value
}
