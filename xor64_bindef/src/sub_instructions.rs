
pub enum SubArgType {
    ImmInt(usize),
    Ptr(usize),
    Reg(RegisterType),
    // TODO: Floats
}

/// # Registers
/// 
/// All regsiters in xor64_ftb
/// 
/// ## 32 bit INT registers
/// 
/// r0, r1, r2, r3, r4, r5, r6, r7, r8, r9
/// 
/// ## 32 bit FLOAT
/// 
/// f0, f1, f2, f3, f4, f5, f6, f7, f8, f9
/// 
/// ## Internal registers
/// 
/// i0 - internal regiter used only for dereferencing, for now
/// 
/// ## controll registers
/// 
/// ip - Instruction pointer  
/// 
/// ### compare flag register (cf)
/// 
/// Stores the flags in a 3 bit bitmap:
/// 
/// GEL
/// 
/// G - Greater than
/// E - Equal
/// L - Less than
/// 
#[allow(non_camel_case_types)]
pub enum RegisterType {
    r0, r1, r2, r3, r4, r5, r6, r7, r8, r9,
    f0, f1, f2, f3, f4, f5, f6, f7, f8, f9,
    i0,
    ip, cf
}

pub struct SubInstruction {
    pub typ: SubInstructionType,
    pub dst: Option<SubInstructionDstTyp>,
    pub dst_size: Option<SubInstructionDstSizeType>,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SubInstructionDstTyp {
    Reg(RegisterType),
    mem(u32)
}

#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SubInstructionDstSizeType {
    /// u8 1
    Byte,
    /// u16 2
    Word,
    /// u32 4
    Double
}


/// # Instructions
/// 
/// reg - `([rf][0-9]|i0)` - Destination of instruction, can only be a register
/// 
/// arg - `([rf][0-9]|i0|MEM_ADDR)` - Argument of instruction, can be a register, or a memory ptr
/// 
/// t - `[bwd]` - The type of reg and arg, can be, b (byte, u8), w (word, u16), d (double, u32)
/// 
/// ## Load (ld_{reg}{T} {arg})
/// 
/// `reg = (*arg) as {T};`
/// 
/// `arg` should be a register or pointer containing a memory address
/// 
/// The instruction copies the value from `arg`'s containing pointer to `reg` transfering only the bits of the specified size.
/// 
/// It does not zero the bits that were not transfered.
/// 
/// ## Store (st_{reg}{T} {arg})
/// 
/// `*arg = reg as T;`
/// 
/// `arg` should be a register or pointer containing a memory address
/// 
/// The instruction copies the value from `reg` to `reg`'s ptr transfering only the bits of the specified size.
/// 
/// It does not zero the bits that were not transfered.
/// 
/// ## Compare (cmp_{reg}{T} {arg} {arg})
/// 
// TODO: add proper link to RegisterType
/// See [RegisterType](#) for more info on it
/// 
/// ```rs
/// cf =      ((reg as T) > (arg as T)) as u8; 
/// cf = cf | ((reg as T) == (arg as T)) as u8 << 1;
/// cf = cf | ((reg as T) < (arg as T)) as u8 << 2;
/// ```
/// 
/// Compares `reg` and `arg`, both as `T` and stores the result in cf
/// 
/// ## Add (add_{reg}{T} {arg})
/// 
/// `*reg += arg as T`
/// 
/// ## Subtract (sub_{reg}{T} {arg})
/// 
/// `*reg -= arg as T`
/// 
/// ## Multiply (mul_{reg}{T} {arg})
/// 
/// `*reg *= arg as T`
/// 
/// ## Divide (div_{reg}{T} {arg})
///  
/// `*reg /= arg as T`
///
/// ## Modulus (mod_{reg}{T} {arg})
///  
/// `*reg %= arg as T`
///
/// ## Unconditional jump (jmp {arg})
/// 
/// `ip = addrof(arg)`
/// 
/// ## Jump if equals (je {arg})
/// 
/// ```rs
/// if cf & 0b010 == 0b010 {
///     ip = addrof(arg)
/// }
/// ```
/// 
/// ## Jump if not equals (jne {arg})
/// 
/// ```rs
/// if cf & 0b010 != 0b010 {
///     ip = addrof(arg)
/// }
/// ```
/// 
/// ## Jump if greater than (jg {arg})
/// 
/// ```rs
/// if cf & 0b100 == 0b100 {
///     ip = addrof(arg)
/// }
/// ```
/// 
/// ## Jump if greater or equals than(jge {arg})
/// 
/// ```rs
/// if cf & 0b010 == 0b010 || cf & 0b100 == 0b100{
///     ip = addrof(arg)
/// }
/// ```
/// 
/// ## Jump if less than  (jl {arg})
/// 
/// 
/// ```rs
/// if cf & 0b001 == 0b001 {
///     ip = addrof(arg)
/// }
/// ```
/// 
/// ## Jump if less or equals than (jle {arg})
/// 
/// ```rs
/// if cf & 0b010 == 0b010 || cf & 0b001 == 0b001{
///     ip = addrof(arg)
/// }
/// ```
/// 
/// ## Interrupts (int {arg})
/// 
/// See [Interrupts](#) for more info
/// 
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SubInstructionType {
    ld,
    st,
    cmp,
    add,
    sub,
    mul,
    div,
    r#mod,
    jmp,
    je,
    jne,
    jg,
    jge,
    jl,
    jle,
    int,
}