#[derive(Debug, Clone)]
pub enum Opcode {
    Const,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    SetGlobal,
    //GetGlobal,
    Halt,
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> u8 {
        match opcode {
            Opcode::Const => 0,
            Opcode::Add => 1,
            Opcode::Sub => 2,
            Opcode::Mul => 3,
            Opcode::Div => 4,
            Opcode::Negate => 5,
            Opcode::SetGlobal => 6,
            Opcode::Halt => 7,
        }
    }
}

impl From<u8> for Opcode {
    fn from(val: u8) -> Opcode {
        match val {
            0 => Opcode::Const,
            1 => Opcode::Add,
            2 => Opcode::Sub,
            3 => Opcode::Mul, 
            4 => Opcode::Div,
            5 => Opcode::Negate,
            6 => Opcode::SetGlobal,
            7 => Opcode::Halt,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByteCode {
    pub opcodes: Vec<u8>,
    pub constants: Vec<f64>,
}
