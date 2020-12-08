pub mod utils {

    use std::boxed::Box;
    use std::error::Error;
    use std::fs;
    
    pub fn read_file_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file_str = fs::read_to_string(path)?;
        let lines = file_str
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_owned())
            .collect();
        Ok(lines)
    }
}

pub mod console_day08 {

    use std::collections::HashMap;

    pub enum Instruction {
        Add { val: i32 },
        Nop { val: i32 },
        Jmp { val: i32 },
    }

    pub struct HandheldConsole<'a> { 
        pub instructions: &'a Vec<Instruction>,
    }

    impl HandheldConsole<'_> {
        pub fn run_until_loop(&self) -> i32 {
            let mut pos: i32 = 0;
            let mut acc: i32 = 0;
            let mut seen: HashMap<i32, i32> = HashMap::new();
            
            while !seen.contains_key(&pos) {
                seen.insert(pos, 1);
                let mut incr_pos = 1;

                match self.instructions[pos as usize] {
                    Instruction::Add{val} => { acc += val },
                    Instruction::Jmp{val} => { incr_pos = val },
                    Instruction::Nop{val: _} => { },
                }

                pos += incr_pos;
            }

            return acc;
        }

        pub fn run_yolo(&self) -> i32 {
            let mut pos: i32 = 0;
            let mut acc: i32 = 0;
            let last_instr = self.instructions.len() as i32;
            
            loop {
                let mut incr_pos = 1;

                match self.instructions[pos as usize] {
                    Instruction::Add{val} => { acc += val },
                    Instruction::Jmp{val} => { incr_pos = val },
                    Instruction::Nop{val: _} => { },
                }

                pos += incr_pos;
                if pos >= last_instr {
                    break;
                }
            }

            return acc;
        }
    }
}
