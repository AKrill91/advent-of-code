
#[derive(Clone, Copy, Eq, PartialEq)]
enum BlockType {
    File,
    Free
}

impl BlockType {
    pub fn other(self) -> Self {
        match self {
            BlockType::File => BlockType::Free,
            BlockType::Free => BlockType::File,
        }
    }
}

fn parse(input: &str) -> Vec<Option<u32>> {
    let mut out = vec![];

    let mut next_char_type = BlockType::File;
    let mut next_file_id = 0;

    input.trim().chars()
        .for_each(|c| {
            let size = char::to_digit(c, 10).unwrap();
            let file_id = match next_char_type {
                BlockType::File => {
                    Some(next_file_id)
                }
                BlockType::Free => {
                    None
                }
            };

            if file_id.is_some() {
                next_file_id += 1;
            }

            for _ in 0..size {
                out.push(file_id);
            }
            next_char_type = next_char_type.other();
        });

    out
}

fn display(blocks: &Vec<Option<u32>>) -> String {
    blocks.iter()
        .map(|b| {
            match b {
                None => '.',
                Some(f) => {
                    if *f < 10 {
                        char::from_digit(*f, 10).unwrap()
                    } else {
                        '?'
                    }
                }
            }
        })
        .collect()
}

fn defragment(blocks: &Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut out = vec![None; blocks.len()];
    let mut next_to_take = blocks.len() - 1;
    let mut stop = blocks.len();

    let mut i = 0;

    while i < stop {
        let block = blocks[i];

        if block.is_none() {
            let mut take = blocks[next_to_take];
            while take.is_none() && next_to_take > i {
                next_to_take -= 1;
                take = blocks[next_to_take];
            }
            out[i] = take;
            stop = next_to_take;
            next_to_take -= 1;
        } else {
            out[i] = block;
        }
        i += 1;
    }

    out
}

fn checksum(blocks: &Vec<Option<u32>>) -> i64 {
    blocks.iter().enumerate()
        .map(|(index, block)| {
            if let Some(file_id) = block {
                (index as i64) * ((*file_id) as i64)
            } else {
                0
            }
        })
        .sum()
}

pub fn run_a(input: &str) -> i64 {
    let blocks = parse(input);
    let defragmented = defragment(&blocks);
    checksum(&defragmented)
}

pub fn run_b(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use crate::day09::display;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn small_example() -> &'static str {
        "12345"
    }

    fn example() -> &'static str {
        "2333133121414131402"
    }

    #[test]
    fn parse_small() {
        init();
        let parsed = super::parse(small_example());

        assert_eq!("0..111....22222", display(&parsed));
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());

        assert_eq!("00...111...2...333.44.5555.6666.777.888899", display(&parsed));
    }

    #[test]
    fn defragment_small() {
        init();
        let parsed = super::parse(small_example());
        let defragged = super::defragment(&parsed);

        assert_eq!("022111222......", display(&defragged));
    }

    #[test]
    fn defragment() {
        init();
        let parsed = super::parse(example());
        let defragged = super::defragment(&parsed);

        assert_eq!("0099811188827773336446555566..............", display(&defragged));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(1928, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(2858, super::run_b(example()));
    }
}