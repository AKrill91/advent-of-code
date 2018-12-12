use std::slice::Iter;

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    pub fn metadata_sum(&self) -> i32 {
        let mut sum = 0;

        for child in &self.children {
            sum += child.metadata_sum();
        }

        for val in &self.metadata {
            sum += *val;
        }

        sum
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let root = parse_input(input);

    let sum = root.metadata_sum();

    println!("Sum of metadata is {}", sum);

    sum
}

fn parse_input(input: &Vec<String>) -> Node {
    let mut numbers: Vec<i32> = Vec::new();

    for line in input {
        let parts = line.split(" ");

        for n in parts {
            numbers.push(n.parse::<i32>().unwrap());
        }
    }

    println!("Found {} numbers", numbers.len());

    let mut num_iter = numbers.iter();

    parse_child(&mut num_iter)
}

fn parse_child(iter: &mut Iter<i32>) -> Node {
    let num_children = *iter.next().unwrap();
    let num_metadata = *iter.next().unwrap();

    let mut children = Vec::with_capacity(num_children as usize);

    for _ in 0..num_children {
        children.push(parse_child(iter));
    }

    let mut metadata = Vec::with_capacity(num_metadata as usize);

    for _ in 0..num_metadata {
        metadata.push(*iter.next().unwrap());
    }

    Node { children, metadata}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_a() {
        let input = vec![String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")];

        assert_eq!(138, run_a(&input));
    }
}