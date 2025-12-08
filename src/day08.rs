use std::cmp::min;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &JunctionBox) -> f64 {
        (((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

impl TryFrom<&str> for JunctionBox {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',')
            .map(|s| s.parse::<i64>());

        Ok(JunctionBox::new(
            parts.next().unwrap().map_err(|_| ())?,
            parts.next().unwrap().map_err(|_| ())?,
            parts.next().unwrap().map_err(|_| ())?,
        ))
    }
}

#[derive(Debug)]
struct Circuit {
    connections: Vec<(JunctionBox, JunctionBox)>,
    unique: HashSet<JunctionBox>,
}

impl Circuit {
    pub fn new(initial: (JunctionBox, JunctionBox)) -> Self {
        Self {
            connections: vec![initial],
            unique: HashSet::from([initial.0, initial.1]),
        }
    }

    pub fn add(&mut self, a: JunctionBox, b: JunctionBox) {

        let mut already_exists = false;
        //Make sure either a or b are already in the circuit

        self.connections.iter()
            .for_each(|(x, y)| {
                if (*x == a && *y == b) || (*x == b && *y == a) {
                    already_exists = true;
                }
            });

        if !already_exists {
            self.connections.push((a, b));
            self.unique.extend([a, b]);
        }
    }

    pub fn contains(&self, junction_box: &JunctionBox) -> bool {
        self.unique.contains(junction_box)
    }

    pub fn size(&self) -> usize {
        self.unique.len()
    }
}

struct CircuitSet {
    circuits: Vec<Circuit>
}

impl CircuitSet {
    pub fn new() -> Self {
        Self {
            circuits: vec![]
        }
    }

    pub fn add(&mut self, a: JunctionBox, b: JunctionBox) {
        let mut a_index = None;
        let mut b_index = None;

        for (i, circuit) in self.circuits.iter().enumerate() {
            if circuit.contains(&a) {
                a_index = Some(i);
            }
            if circuit.contains(&b) {
                b_index = Some(i);
            }
        }

        match (a_index, b_index) {
            (Some(a_index), Some(b_index)) if a_index == b_index =>{
                //Both already exist in the same circuit, do nothing
            },
            (Some(a_index), Some(b_index)) => {
                //Both already exist, but they're separate circuits so we need to merge them
                let b_circuit = self.circuits.remove(b_index);

                // If a_index > b_index, then removing b_index has shifted a_index down by 1
                let a_index_adjusted = if a_index > b_index {
                    a_index - 1
                } else {
                    a_index
                };

                let a_circuit = & mut self.circuits[a_index_adjusted];

                for (x, y) in b_circuit.connections {
                    a_circuit.add(x, y);
                }
            },
            (Some(index), None) | (None, Some(index)) => {
                let circuit = &mut self.circuits[index];
                circuit.add(a, b);
            },
            (None, None) => {
                //Create new circuit
                self.circuits.push(Circuit::new((a, b)));
            }
        }
    }

    pub fn sort(&mut self) {
        self.circuits
            .sort_by(|a, b| a.size().cmp(&b.size()).reverse());
    }

    pub fn largest_n_product(&self, n: usize) -> i64 {
        assert!(self.circuits.len() >= n);

        self.circuits.iter()
            .take(n)
            .map(|c| c.size() as i64)
            .product()
    }

    pub fn contains_all(&self, junction_boxes: &[JunctionBox]) -> bool {
        junction_boxes.iter()
            .all(|b| {
                self.circuits.iter()
                    .any(|c| c.contains(b))
            })
    }
}

fn parse(input: &str) -> Vec<JunctionBox> {
    input.trim()
        .lines()
        .into_iter()
        .map(JunctionBox::try_from)
        .map(Result::unwrap)
        .collect()
}
pub fn run_a(input: &str) -> i64 {
    let junction_boxes = parse(input);

    //Hacky way to handle example only wanting 10 steps
    let limit = if junction_boxes.len() > 20 {
        1000
    } else {
        10
    };

    let mut circuits = CircuitSet::new();

    let pairs= connections(junction_boxes);

    for pair in pairs.into_iter().take(limit) {
        circuits.add(pair.0, pair.1);
    }

    circuits.sort();

    circuits.largest_n_product(3)
}

pub fn run_b(input: &str) -> i64 {
    let junction_boxes = parse(input);

    let mut circuits = CircuitSet::new();

    let pairs = connections(junction_boxes.clone());

    let mut last = pairs[0];

    for pair in pairs {
        if circuits.circuits.len() == 1 && circuits.contains_all(&junction_boxes) {
            break;
        }

        circuits.add(pair.0, pair.1);
        last = pair;
    }

    last.0.x * last.1.x
}

fn connections(junction_boxes: Vec<JunctionBox>) -> Vec<(JunctionBox, JunctionBox, f64)> {
    let mut out = vec![];

    for i in 0..junction_boxes.len() - 1 {
        for j in (i + 1)..junction_boxes.len() {
            let a = junction_boxes[i];
            let b = junction_boxes[j];
            let dist = a.distance_to(&b);
            out.push((a, b, dist));
        }
    }

    out.sort_by(|a, b| {
        a.2.total_cmp(&b.2)
    });

    out
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
        assert_eq!(parsed.len(), 20);
        assert_eq!(parsed[0], JunctionBox::new(162, 817, 812));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(40, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(25272, super::run_b(example()));
    }

    mod junction_box {
        use std::convert::TryFrom;
        use super::super::*;
        use super::*;

        #[test]
        fn try_from() {
            init();

            let from = JunctionBox::try_from("162,817,812").unwrap();
            assert_eq!(from.x, 162);
            assert_eq!(from.y, 817);
            assert_eq!(from.z, 812);
        }
    }

    mod circuit {
        use super::super::*;
        use super::*;

        #[test]
        fn contains() {
            init();
            let circuit = Circuit::new((
                JunctionBox::new(0, 0, 0),
                JunctionBox::new(1, 1, 1),
            ));

            assert!(circuit.contains(&JunctionBox::new(0, 0, 0)));
            assert!(circuit.contains(&JunctionBox::new(1, 1, 1)));
            assert!(!circuit.contains(&JunctionBox::new(2, 2, 2)));
        }
    }
}