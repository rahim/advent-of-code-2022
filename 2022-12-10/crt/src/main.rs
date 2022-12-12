use std::fs;
use std::io::{self, Write};

fn main() {
    let path = "input.txt";
    let input = fs::read_to_string(path).expect("Failed to read file");
    let instructions = parse_input(&input);
    let mut cpu = CPU::new(&instructions);

    println!(
        "Signal strength sum: {}",
        cpu.signal_strength_sum()
    );

    let mut cpu_b = CPU::new(&instructions);
    let mut crt = CRT::new();
        
    Runner::cycle(&mut cpu_b, &mut crt, 240);
    println!("{}", crt.print());
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l: &str| Instruction::from_str(l)).collect()
}

#[derive(Debug)]
struct CPU {
    x: i32,
    completed_cycles: usize,
    instructions: Vec<Instruction>,
    expanded_instructions: Vec<Instruction>
}

impl CPU {
    fn new(instructions: &Vec<Instruction>) -> CPU {
        CPU {
            x: 1,
            completed_cycles: 0,
            instructions: instructions.clone(),
            expanded_instructions: CPU::expanded_instructions(instructions)
        }
    }
    fn cycle(&mut self) {
        let instruction = self.expanded_instructions.get(self.completed_cycles).unwrap();
        
        println!("{:?} {:?} {:?}", self.completed_cycles+1, self.x, instruction);

        match instruction {
            Instruction::AddX { v } => self.x += v,
            _ => ()
        }

        self.completed_cycles += 1;
    }
    fn current_cycle_number(&mut self) -> usize {
        return self.completed_cycles + 1;
    }
    fn signal_strength(&mut self) -> i32 {
        println!("signal strength: {:?}x{:?}={:?}", self.current_cycle_number(), self.x, (self.completed_cycles as i32) * (self.x as i32));
        return (self.current_cycle_number() as i32) * (self.x as i32);
    }
    // pad the first cycle of two cycle instructions with noop
    fn expanded_instructions(instructions: &Vec<Instruction>) -> Vec<Instruction> {
        instructions
            .iter()
            .map(|i| {
                match i {
                    Instruction::Noop => vec![*i],
                    Instruction::AddX { v: _ } => vec![Instruction::Noop, *i]
                }
            })
            .flatten()
            .collect()
    }
    fn signal_strength_sum(&mut self) -> i32 {
        let mut sum = 0;
        for _i in 1..=19 { self.cycle(); }
        sum += self.signal_strength();
        for _i in 1..=40 { self.cycle(); }
        sum += self.signal_strength();
        for _i in 1..=40 { self.cycle(); }
        sum += self.signal_strength();
        for _i in 1..=40 { self.cycle(); }
        sum += self.signal_strength();
        for _i in 1..=40 { self.cycle(); }
        sum += self.signal_strength();
        for _i in 1..=40 { self.cycle(); }
        sum += self.signal_strength();
        return sum;
    }
}

struct CRT {
    pixels: Vec<bool>,
    completed_cycles: i32
}

impl CRT {
    const PIXEL_COUNT:usize = 240;
    const ROW_SIZE:usize = 40;

    fn new() -> CRT {
        CRT {
            pixels: vec![false; CRT::PIXEL_COUNT],
            completed_cycles: 0
        }
    }

    fn matrix(&self) -> Vec<Vec<bool>> {
        return self.pixels.chunks(CRT::ROW_SIZE).map(|chunk| chunk.to_vec()).collect();
    }

    fn cycle(&mut self, x: i32) -> () {
        let sprite_window = x-1..=(x+1);
        if sprite_window.contains(&(self.completed_cycles % (CRT::ROW_SIZE as i32)) ) {
            self.pixels[self.completed_cycles as usize] = true;
        }
        self.completed_cycles += 1;
    }

    fn print(&self) -> String {
        self.matrix().iter()
            .map(|row| {
                row.iter().map(|col| if *col {'#'} else {'.'} ).collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

struct Runner {
}

impl Runner {
    fn cycle(cpu: &mut CPU, crt: &mut CRT, cycles: i32) {
        for _i in 0..cycles {
            crt.cycle(cpu.x);
            cpu.cycle();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX { v: i32 }
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        if s == "noop" {
            Instruction::Noop
        }
        else if s.starts_with("addx") {
            let delta_str: &str = s.split(" ").nth(1).unwrap();
            let delta: i32 = delta_str.parse().unwrap();
            Instruction::AddX { v: delta }
        }
        else {
            unreachable!("unknown instruction")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;
    
    const SMALL_EXAMPLE: &str = indoc! {"
        noop
        addx 3
        addx -5
    "};

    #[test]
    fn test_small() {
        let instructions = parse_input(SMALL_EXAMPLE);
        assert_eq!(instructions.len(), 3);

        let mut cpu = CPU::new(&instructions);
        assert_eq!(cpu.x,1);
        cpu.cycle(); // noop
        assert_eq!(cpu.x,1);
        cpu.cycle(); // addx 3 tick 1
        assert_eq!(cpu.x,1);
        cpu.cycle(); // addx 3 tick 2
        assert_eq!(cpu.x,4);
        cpu.cycle(); // addx -5 tick 1
        assert_eq!(cpu.x,4);
        cpu.cycle(); // addx -5 tick 2
        assert_eq!(cpu.x,-1);
    }

    #[test]
    fn test_big() {
        let instructions = parse_input(BIG_EXAMPLE);
        let mut cpu = CPU::new(&instructions);
        // 19 here, because we start with current cycle = 1
        for _i in 1..=19 { cpu.cycle(); }
        assert_eq!(cpu.current_cycle_number(),20);
        assert_eq!(cpu.signal_strength(),420);
        for _i in 1..=40 { cpu.cycle(); }
        assert_eq!(cpu.current_cycle_number(),60);
        assert_eq!(cpu.signal_strength(),1140);
        for _i in 1..=40 { cpu.cycle(); }
        assert_eq!(cpu.current_cycle_number(),100);
        assert_eq!(cpu.signal_strength(),1800);
        for _i in 1..=40 { cpu.cycle(); }
        assert_eq!(cpu.current_cycle_number(),140);
        assert_eq!(cpu.signal_strength(),2940);
        for _i in 1..=40 { cpu.cycle(); }
        assert_eq!(cpu.current_cycle_number(),180);
        assert_eq!(cpu.signal_strength(),2880);
        for _i in 1..=40 { cpu.cycle(); }
        assert_eq!(cpu.current_cycle_number(),220);
        assert_eq!(cpu.signal_strength(),3960);
    }

    #[test]
    fn test_signal_strength_sum() {
        let instructions = parse_input(BIG_EXAMPLE);
        let mut cpu = CPU::new(&instructions);
        assert_eq!(cpu.signal_strength_sum(), 13140);
    }

    #[test]
    fn test_crt_print() {
        let crt = CRT::new();
        let empty_crt: &str = indoc! {"
            ........................................
            ........................................
            ........................................
            ........................................
            ........................................
            ........................................
        "};
        assert_eq!(crt.print(), empty_crt.trim());
    }

    #[test]
    fn test_example_run() {
        let instructions = parse_input(BIG_EXAMPLE);
        let mut cpu = CPU::new(&instructions);
        let mut crt = CRT::new();
        
        Runner::cycle(&mut cpu, &mut crt, 240);

        let expected_image: &str = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "};
        assert_eq!(crt.print(), expected_image.trim());
    }

    const BIG_EXAMPLE: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

}