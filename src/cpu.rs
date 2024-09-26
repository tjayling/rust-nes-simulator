pub mod cpu {
    pub const LDA: u8 = 0xA9;
    pub const TAX: u8 = 0xAA;
    pub const INX: u8 = 0xE8;
    pub const BRK: u8 = 0x00;

    pub struct CPU {
        pub register_a: u8,
        pub register_x: u8,
        pub status: u8,
        pub program_counter: u16,
    }

    impl CPU {
        pub fn new() -> Self {
            CPU {
                register_a: 0,
                register_x: 0,
                status: 0,
                program_counter: 0,
            }
        }

        pub fn interpret(&mut self, program: Vec<u8>) {
            self.program_counter = 0;

            loop {
                let opscode = self.get_next_op(&program);

                match opscode {
                    LDA => {
                        self.register_a = self.get_next_op(&program);
                        self.update_zero_and_negative_flags(self.register_a)
                    }
                    TAX => {
                        self.register_x = self.register_a;
                        self.update_zero_and_negative_flags(self.register_x)
                    }
                    INX => {
                        self.register_x = self.register_x.wrapping_add(1);
                        self.update_zero_and_negative_flags(self.register_x)
                    }
                    BRK => {
                        return;
                    }
                    _ => todo!()
                }
            }
        }

        fn update_zero_and_negative_flags(&mut self, result: u8) {
            self.status = if result == 0 { self.status | 0b0000_0010 } else { self.status & 0b1111_1101 };
            self.status = if result & 0b1000_0000 != 0 { self.status | 0b1000_0000 } else { self.status & 0b0111_1111 }
        }

        fn get_next_op(&mut self, program: &Vec<u8>) -> u8 {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;
            opscode
        }
    }
}

#[cfg(test)]
mod test {
    use super::cpu::*;

    #[test]
    fn test_0xa9_lda_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LDA, 0x05, BRK]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & 0b0000_0010, 0b00);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec!(LDA, 0x00, BRK));
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![TAX, BRK]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LDA, 0xc0, TAX, INX, BRK]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![INX, INX, BRK]);

        assert_eq!(cpu.register_x, 1)
    }
}