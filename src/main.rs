use std::io::{stdin, stdout, Write};

struct BfInterperter {
    memory: Vec<u8>,
    pointer: usize,
    i_pointer: usize,
    stack: Vec<usize>,
    prog: Vec<char>,
}

impl BfInterperter {
    fn new(prog: Vec<char>) -> Self {
        Self {
            memory: vec![0; 1],
            pointer: 0,
            i_pointer: 0,
            stack: vec![0; 0],
            prog: prog,
        }
    }

    fn ensure_memory(&mut self) {
        if self.pointer >= self.memory.len() {
            self.memory.resize(self.pointer + 1, 0);
        }
    }

    fn move_right(&mut self) -> Option<usize> {
        self.pointer += 1;
        self.ensure_memory();
        return None;
    }

    fn move_left(&mut self) -> Option<usize> {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
        return None;
    }

    fn increment(&mut self) -> Option<usize> {
        self.memory[self.pointer] += 1;
        return None;
    }

    fn decrement(&mut self) -> Option<usize> {
        self.memory[self.pointer] -= 1;
        return None;
    }

    fn loop_start(&mut self) -> Option<usize> {
        if self.memory[self.pointer] != 0 {
            self.stack.push(self.i_pointer);
            return None;
        }
        let mut peek = self.i_pointer;
        let mut counter = 0;
        loop {
            if peek >= self.prog.len() {
                return Some(peek);
            };
            match self.prog[peek] {
                '[' => counter += 1,
                ']' => {
                    if counter == 0 {
                        return Some(peek);
                    }
                    {
                        counter -= 1
                    }
                }
                _ => {}
            }
            if self.prog[peek] == ']' {
                return Some(peek);
            };
            peek += 1;
        }
    }

    fn loop_end(&mut self) -> Option<usize> {
        return self.stack.pop();
    }

    fn output(&mut self) -> Option<usize> {
        let _char = char::from(self.memory[self.pointer]);
        print!("{}", _char);
        return None;
    }

    fn step(&mut self) {
        let instr = self.prog[self.i_pointer];
        match match instr {
            '>' => self.move_right(),
            '<' => self.move_left(),
            '+' => self.increment(),
            '-' => self.decrement(),
            '[' => self.loop_start(),
            ']' => self.loop_end(),
            '.' => self.output(),
            _ => None,
        } {
            Some(np) => self.i_pointer = np,
            None => self.i_pointer += 1,
        }
    }

    fn execute(&mut self) {
        while self.i_pointer < self.prog.len() {
            self.step();
        }
    }
}

fn main() {
    let mut prog = String::new();
    let mut bf = BfInterperter::new(prog.chars().filter(|c| "+-><[].".contains(*c)).collect());
    loop {
        print!("|~ ");
        let _ = stdout().flush();
        stdin().read_line(&mut prog).expect("mi sona ala");
        bf.prog = prog.chars().filter(|c| "+-><[].".contains(*c)).collect();
        bf.execute();
        println!("mem: {:?}", bf.memory)
    }
}
