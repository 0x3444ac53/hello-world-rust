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

    fn move_right(&mut self) {
        self.pointer += 1;
        self.ensure_memory();
        self.i_pointer += 1;
    }

    fn move_left(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
        self.i_pointer += 1;
    }

    fn increment(&mut self) {
        self.memory[self.pointer] += 1;
        self.i_pointer += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer] -= 1;
        self.i_pointer += 1;
    }

    fn loop_start(&mut self) {
        self.stack.push(self.i_pointer);
        self.i_pointer += 1;
    }

    fn loop_end(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.stack.pop();
            self.i_pointer += 1;
        } else if let Some(ip) = self.stack.pop() {
            self.i_pointer = ip;
        } else {
            self.i_pointer = 30000;
        }
    }

    fn output(&mut self) {
        let _char = char::from(self.memory[self.pointer]);
        print!("{}", _char);
        self.i_pointer += 1;
    }

    fn step(&mut self) {
        let instr = self.prog[self.i_pointer];
        match instr {
            '>' => self.move_right(),
            '<' => self.move_left(),
            '+' => self.increment(),
            '-' => self.decrement(),
            '[' => self.loop_start(),
            ']' => self.loop_end(),
            '.' => self.output(),
            _ => self.i_pointer += 1,
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
