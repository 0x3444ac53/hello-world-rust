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
            stack: vec![0; 1],
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
            self.i_pointer += 1;
        } else if let Some(ip) = self.stack.pop() {
            self.i_pointer = ip;
        } else {
            self.i_pointer = 30000;
        }
    }

    fn output(&mut self) {
        let _char = char::from_u32(self.memory[self.pointer].into());
        print!("{:?}", _char);
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
        println!(
            "  instr: {instr}\n    mem: {:?}\npointer: {}\n------------------------",
            self.memory, self.pointer
        )
    }

    fn execute(&mut self) {
        while self.i_pointer < self.prog.len() {
            self.step();
        }
    }
}

fn main() {
    let prog = r#"Hello! and Welcome!
    Let's start by placing 'A' in the first cell and 'a' in the second
    that'll make writing messages a little easier
    >++++++++++[<++++>-]+.
    "#;
    let mut bf = BfInterperter::new(prog.chars().filter(|c| "+-><[].".contains(*c)).collect());
    bf.execute();
}
