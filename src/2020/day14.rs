use std::collections::HashMap;
use regex::Regex;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let program = Program::from(input);

    let mut execution = Execution::new(&program);

    execution.run();

    execution.memory_sum() as i64
}

pub fn run_b<'a, I, J>(input: I) -> u64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let program = Program::from(input);

    let mut execution = ExecutionV2::new(&program);

    execution.run();

    execution.memory_sum()
}

struct Execution<'a> {
    program: &'a Program,
    instruction_pointer: usize,
    mask: Mask,
    memory: HashMap<usize, u64>,
    has_run: bool,
}

impl<'a> Execution<'a> {
    pub fn new(program: &Program) -> Execution {
        Execution {
            program,
            instruction_pointer: 0,
            mask: Mask::default(),
            memory: HashMap::new(),
            has_run: false,
        }
    }

    pub fn run(&mut self) {
        if self.has_run {
            panic!();
        }

        for instruction in &self.program.instructions {
            match instruction {
                Instruction::Mask(mask) => {
                    self.mask = *mask;
                }
                Instruction::Memory(address, val) => {
                    let real = self.mask.apply(*val);
                    if real == 0 {
                        self.memory.remove(address);
                    } else {
                        self.memory.insert(*address, real);
                    }
                }
            }
        }

        self.has_run = true;
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values()
            .sum()
    }
}

struct ExecutionV2<'a> {
    program: &'a Program,
    instruction_pointer: usize,
    mask: Mask,
    memory: HashMap<u64, u64>,
    has_run: bool,
}

impl<'a> ExecutionV2<'a> {
    pub fn new(program: &Program) -> ExecutionV2 {
        ExecutionV2 {
            program,
            instruction_pointer: 0,
            mask: Mask::default(),
            memory: HashMap::new(),
            has_run: false,
        }
    }

    pub fn run(&mut self) {
        if self.has_run {
            panic!();
        }

        for instruction in &self.program.instructions {
            match instruction {
                Instruction::Mask(mask) => {
                    self.mask = *mask;
                }
                Instruction::Memory(address, val) => {
                    let permutations = self.mask.apply_to_memory(*address as u64);

                    for p in permutations {
                        if *val == 0 {
                            self.memory.remove(&p);
                        } else {
                            self.memory.insert(p, *val);
                        }
                    }
                }
            }
        }

        self.has_run = true;
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values()
            .sum()
    }
}

struct Program {
    instructions: Vec<Instruction>
}

impl<'a, I, J> From<I> for Program
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    fn from(input: I) -> Self {
        Program {
            instructions: input.into_iter()
                .map(|s| s.as_ref())
                .map(|s| Instruction::from(s))
                .collect::<Vec<_>>()
        }
    }
}

enum Instruction {
    Mask(Mask),
    Memory(usize, u64),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Mask {
    and_bits: u64,
    or_bits: u64,
}

impl From<&str> for Mask {
    fn from(input: &str) -> Self {
        let mask_length = 35;
        let mut and_bits = 2u64.pow(mask_length + 1) - 1;
        let mut or_bits = 0;

        for (i, c) in input.chars().enumerate() {
            let pow_two = 2u64.pow(mask_length - (i as u32));

            match c {
                '0' => {
                    and_bits -= pow_two;
                }
                '1' => {
                    or_bits += pow_two;
                }
                _ => {}
            }
        }

        Mask {
            and_bits,
            or_bits,
        }
    }
}

impl Default for Mask {
    fn default() -> Self {
        Mask {
            and_bits: 0,
            or_bits: u64::max_value(),
        }
    }
}

impl Mask {
    pub fn apply(&self, value: u64) -> u64 {
        (value & self.and_bits) | self.or_bits
    }

    pub fn apply_to_memory(&self, address: u64) -> Vec<u64> {
        let applied = address | self.or_bits;
        let permutations = self.floating_permutations();

        permutations.iter()
            .map(|&p| applied ^ p)
            .collect()
    }

    pub fn floating_bits(&self) -> u64 {
        self.and_bits ^ self.or_bits
    }

    pub fn floating_permutations(&self) -> Vec<u64> {
        let mut out = vec![];
        let floating = self.floating_bits();
        let set_bits = get_set_bits(floating);
        let num_permutations = 2u64.pow(set_bits.len() as u32);

        for i in 0..num_permutations {
            let mut pattern = 0;

            for bit_index in 0..set_bits.len() {
                let index_pow = 2u64.pow(bit_index as u32);
                if i & index_pow > 0 {
                    pattern += set_bits[bit_index];
                }
            }

            out.push(pattern);
        }

        out
    }
}

fn get_set_bits(val: u64) -> Vec<u64> {
    let mut out = vec![];

    for i in 0..36 {
        let pow_two = 2u64.pow(i);

        if val & pow_two != 0 {
            out.push(pow_two);
        }
    }

    out
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut parts = input.split(" ");
        let kind = parts.next().unwrap();
        parts.next();
        let value = parts.next().unwrap();

        if kind == "mask" {
            Instruction::Mask(Mask::from(value))
        } else {
            let pattern = Regex::new(r"^mem\[(\d+)]").unwrap();

            let captures = pattern.captures(kind).unwrap();

            Instruction::Memory(captures.get(1).unwrap().as_str().parse::<usize>().unwrap(), value.parse::<u64>().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_set_bits_15() {
        let set_bits = get_set_bits(15);

        assert_eq!(4, set_bits.len());
        assert!(set_bits.contains(&1));
        assert!(set_bits.contains(&2));
        assert!(set_bits.contains(&4));
        assert!(set_bits.contains(&8));
    }

    #[test]
    pub fn get_set_bits_11() {
        let set_bits = get_set_bits(11);

        assert_eq!(3, set_bits.len());
        assert!(set_bits.contains(&1));
        assert!(set_bits.contains(&2));
        assert!(set_bits.contains(&8));
    }


    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
    }

    pub fn get_sample_b() -> Vec<&'static str> {
        vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
    }

    fn get_sample_mask(instr: &'static str) ->  &'static str {
        let mut iter = instr.split(" ");
        iter.next();
        iter.next();
        iter.next().unwrap()
    }

    fn get_sample_mask_a() -> &'static str {
        get_sample_mask(get_sample()[0])
    }

    fn get_sample_mask_b() -> &'static str {
        get_sample_mask(get_sample_b()[0])
    }

    fn get_sample_mask_c() -> &'static str {
        get_sample_mask(get_sample_b()[2])
    }

    #[test]
    pub fn mask_from_str() {
        let input = get_sample_mask_a();

        let mask = Mask::from(input);

        assert_eq!(2u64.pow(36) - 3, mask.and_bits);
        assert_eq!(64, mask.or_bits);
    }

    #[test]
    pub fn mask_floating_bits() {
        let input = get_sample_mask_b();

        let mask = Mask::from(input);

        let floating = mask.floating_bits();

        assert_eq!(51, mask.and_bits);
        assert_eq!(18, mask.or_bits);
        assert_eq!(33, floating);
    }

    #[test]
    pub fn mask_floating_permutations() {
        let input = get_sample_mask_b();

        let mask = Mask::from(input);

        let permutations = mask.floating_permutations();

        assert_eq!(4, permutations.len());
        assert!(permutations.contains(&0));
        assert!(permutations.contains(&1));
        assert!(permutations.contains(&32));
        assert!(permutations.contains(&33));
    }

    #[test]
    pub fn mask_floating_permutations_c() {
        let input = get_sample_mask_c();

        let mask = Mask::from(input);

        let permutations = mask.floating_permutations();

        assert_eq!(8, permutations.len());
        assert!(permutations.contains(&0));
        assert!(permutations.contains(&1));
        assert!(permutations.contains(&2));
        assert!(permutations.contains(&3));
        assert!(permutations.contains(&8));
        assert!(permutations.contains(&9));
        assert!(permutations.contains(&10));
        assert!(permutations.contains(&11));
    }

    #[test]
    pub fn mask_apply() {
        let input = get_sample_mask_a();

        let mask = Mask::from(input);

        assert_eq!(73, mask.apply(11));
        assert_eq!(101, mask.apply(101));
        assert_eq!(64, mask.apply(0));
    }

    #[test]
    pub fn mask_apply_to_memory() {
        let input = get_sample_mask_b();

        let mask = Mask::from(input);

        let addresses = mask.apply_to_memory(42);

        assert_eq!(4, addresses.len());
        assert!(addresses.contains(&26));
        assert!(addresses.contains(&27));
        assert!(addresses.contains(&58));
        assert!(addresses.contains(&59));
    }

    #[test]
    pub fn mask_apply_to_memory_c() {
        let input = get_sample_mask_c();

        let mask = Mask::from(input);

        let addresses = mask.apply_to_memory(26);

        assert_eq!(8, addresses.len());
        assert!(addresses.contains(&16));
        assert!(addresses.contains(&17));
        assert!(addresses.contains(&18));
        assert!(addresses.contains(&19));
        assert!(addresses.contains(&24));
        assert!(addresses.contains(&25));
        assert!(addresses.contains(&26));
        assert!(addresses.contains(&27));
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(165, run_a(&sample));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample_b();

        assert_eq!(208, run_b(&sample));
    }
}