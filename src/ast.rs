use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub enum InstructionTypes {
    Hlt,
    Add,
    Sub,
    Or,
    And,
    Nand,
    Xor,
    Ldi,
    Mst,
    Mld,
    Brc,
    Pst,
    Pld,
    Cmp,
    Mov
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: InstructionTypes,
    pub operands: Vec<Operand>
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Eq,
    Neq,
    Lt,
    Gt,
    Gteq,
    Lteq,
    Even,
    Jmp
}

impl Condition {
    pub fn from_str(s: &str) -> Condition {
        match s.to_lowercase().as_str() {
            "eq" => Condition::Eq,
            "neq" => Condition::Neq,
            "lt" => Condition::Lt,
            "gt" => Condition::Gt,
            "gteq" => Condition::Gteq,
            "lteq" => Condition::Lteq,
            "even" => Condition::Even,
            "jmp" => Condition::Jmp,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Label(String),
    Reg(u8),
    Imm(u8),
    Cond(Condition),
    Address(u8)
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Address(a) => write!(f, "${a}").unwrap(),
            Operand::Cond(cond) => write!(f, "{}", format!("{:?}", cond).to_uppercase()).unwrap(),
            Operand::Imm(i) => write!(f, "{i}").unwrap(),
            Operand::Label(l) => write!(f, "[{}]", { let mut x = l.to_string(); x.remove(0); x }).unwrap(),
            Operand::Reg(r) => write!(f, "R{r}").unwrap()
        };
        Ok(())
    }
}

impl Operand {
    pub fn unwrap_label(&self) -> String {
        match self {
            Operand::Label(l) => l.clone(),
            _ => unreachable!()
        }
    }

    pub fn unwrap_imm(&self) -> u8 {
        match self {
            Operand::Imm(i) => *i,
            _ => unreachable!()
        }
    }

    pub fn unwrap_reg(&self) -> u8 {
        match self {
            Operand::Reg(i) => *i,
            _ => unreachable!()
        }
    }
    
    pub fn unwrap_cond(&self) -> Condition {
        match self {
            Operand::Cond(c) => *c,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProgramBody {
    Label(String),
    Instruction(Instruction)
}

#[derive(Debug, Clone)]
pub struct Program(pub Vec<ProgramBody>);

impl Program {
    pub fn new() -> Program { Program(Vec::new()) }
    pub fn generate_label_map(&self) -> HashMap<String, u8> {
        let mut pc = 0;
        let mut ret = HashMap::new();
        for instr in self.0.iter() {
            match instr {
                ProgramBody::Instruction(i) => {
                    pc += 1;
                    for operand in i.operands.iter() {
                        match operand {
                            Operand::Imm(_) | Operand::Label(_) => pc += 1,
                            _ => ()
                        }
                    }
                }
                ProgramBody::Label(l) => { ret.insert(l.clone(), pc / 32); }
            }
        }
        ret
    }
}

