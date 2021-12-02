mod day01;

pub fn run() {
    let input = crate::advent_helper::read_file_lines("resources/2021/day01.txt");
    let a = day01::run_a(&input);
    info!("Day01::a -> {:?}", a);
    let b = day01::run_b(&input);
    info!("Day01::b -> {:?}", b);
}