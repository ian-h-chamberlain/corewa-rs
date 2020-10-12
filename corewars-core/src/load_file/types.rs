use std::fmt;

/// Type alias for signed integer offset. Even though all offsets are modulo
/// `CORESIZE`, they may be larger before `CORESIZE` is known.
pub type Offset = i32;

/// Type alias for unsigned integer offset. Even though all offsets are modulo
/// `CORESIZE`, they may be larger before `CORESIZE` is known.
pub type UOffset = u32;

enum_string!(pub Opcode {
    Dat => "DAT",
    Mov => "MOV",
    Add => "ADD",
    Sub => "SUB",
    Mul => "MUL",
    Div => "DIV",
    Mod => "MOD",
    Jmp => "JMP",
    Jmz => "JMZ",
    Jmn => "JMN",
    Djn => "DJN",
    Cmp => "CMP",
    Seq => "SEQ",
    Sne => "SNE",
    Slt => "SLT",
    Spl => "SPL",
    Nop => "NOP",
});

enum_string!(pub PseudoOpcode {
    Org => "ORG",
    End => "END",
    Equ => "EQU",
    For => "FOR",
});

impl Default for Opcode {
    fn default() -> Self {
        Self::Dat
    }
}

enum_string!(pub Modifier {
    A   => "A",
    B   => "B",
    AB  => "AB",
    BA  => "BA",
    F   => "F",
    X   => "X",
    I   => "I",
});

impl Default for Modifier {
    fn default() -> Self {
        Self::F
    }
}

impl Modifier {
    pub fn default_88_to_94(opcode: Opcode, a_mode: AddressMode, b_mode: AddressMode) -> Self {
        /// Implemented based on the ICWS '94 document,
        /// section A.2.1.2: ICWS'88 to ICWS'94 Conversion
        use Opcode::*;

        match opcode {
            Dat => Modifier::F,
            Jmp | Jmz | Jmn | Djn | Spl | Nop => Modifier::B,
            opcode => {
                if a_mode == AddressMode::Immediate {
                    Modifier::AB
                } else if b_mode == AddressMode::Immediate {
                    Modifier::B
                } else {
                    match opcode {
                        Mov | Cmp | Seq | Sne => Modifier::I,
                        Slt => Modifier::B,
                        Add | Sub | Mul | Div | Mod => Modifier::F,
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

enum_string!(pub AddressMode {
    Immediate           => "#",
    Direct              => "$",
    IndirectA           => "*",
    IndirectB           => "@",
    PreDecIndirectA     => "{",
    PreDecIndirectB     => "<",
    PostIncIndirectA    => "}",
    PostIncIndirectB    => ">",
});

impl Default for AddressMode {
    fn default() -> Self {
        Self::Direct
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Label(String),
    Literal(Offset),
}

impl Default for Value {
    fn default() -> Self {
        Self::Literal(0)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_string = match self {
            Self::Label(value) => value.to_string(),
            Self::Literal(value) => value.to_string(),
        };
        f.pad(&value_string)
    }
}

#[cfg(test)]
mod test {
    use itertools::iproduct;

    use super::*;
    use Opcode::*;

    #[test]
    fn dat_default() {
        for (&a_mode, &b_mode) in iproduct!(AddressMode::iter_values(), AddressMode::iter_values())
        {
            assert_eq!(
                Modifier::default_88_to_94(Opcode::Dat, a_mode, b_mode),
                Modifier::F
            );
        }
    }

    #[test]
    fn modifier_b_default() {
        let opcodes = [Mov, Cmp, Seq, Sne];

        for (&opcode, &a_mode) in iproduct!(opcodes.iter(), AddressMode::iter_values()) {
            if a_mode != AddressMode::Immediate {
                assert_eq!(
                    Modifier::default_88_to_94(opcode, a_mode, AddressMode::Immediate),
                    Modifier::B
                );
            }
        }

        let opcodes = [Add, Sub, Mul, Div, Mod];

        for (&opcode, &a_mode) in iproduct!(opcodes.iter(), AddressMode::iter_values()) {
            if a_mode != AddressMode::Immediate {
                assert_eq!(
                    Modifier::default_88_to_94(opcode, a_mode, AddressMode::Immediate),
                    Modifier::B
                );
            }
        }

        for (&a_mode, &b_mode) in iproduct!(AddressMode::iter_values(), AddressMode::iter_values())
        {
            if a_mode != AddressMode::Immediate {
                assert_eq!(
                    Modifier::default_88_to_94(Opcode::Slt, a_mode, b_mode),
                    Modifier::B
                )
            }
        }

        let opcodes = [Jmp, Jmz, Jmn, Djn, Spl, Nop];

        for (&opcode, &a_mode, &b_mode) in iproduct!(
            opcodes.iter(),
            AddressMode::iter_values(),
            AddressMode::iter_values()
        ) {
            assert_eq!(
                Modifier::default_88_to_94(opcode, a_mode, b_mode),
                Modifier::B
            );
        }
    }

    #[test]
    fn modifier_ab_default() {
        let opcodes = [Mov, Cmp, Seq, Sne, Add, Sub, Mul, Div, Mod, Slt];

        for (&opcode, &b_mode) in iproduct!(opcodes.iter(), AddressMode::iter_values()) {
            assert_eq!(
                Modifier::default_88_to_94(opcode, AddressMode::Immediate, b_mode),
                Modifier::AB
            );
        }
    }

    #[test]
    fn modifier_i_default() {
        let opcodes = [Mov, Cmp, Seq, Sne];

        for (&opcode, &a_mode, &b_mode) in iproduct!(
            opcodes.iter(),
            AddressMode::iter_values(),
            AddressMode::iter_values()
        ) {
            if a_mode != AddressMode::Immediate && b_mode != AddressMode::Immediate {
                assert_eq!(
                    Modifier::default_88_to_94(opcode, a_mode, b_mode),
                    Modifier::I
                );
            }
        }
    }

    #[test]
    fn modifier_f_default() {
        let opcodes = [Add, Sub, Mul, Div, Mod];

        for (&opcode, &a_mode, &b_mode) in iproduct!(
            opcodes.iter(),
            AddressMode::iter_values(),
            AddressMode::iter_values()
        ) {
            if a_mode != AddressMode::Immediate && b_mode != AddressMode::Immediate {
                assert_eq!(
                    Modifier::default_88_to_94(opcode, a_mode, b_mode),
                    Modifier::F
                );
            }
        }
    }

    #[test]
    fn value_to_string() {
        assert_eq!(
            String::from("some_label"),
            Value::Label(String::from("some_label")).to_string()
        );

        assert_eq!(String::from("123"), Value::Literal(123).to_string());
    }
}