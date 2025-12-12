use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;
use petgraph::graph::{Node, NodeIndex};

struct IndexedGraph {
    graph: petgraph::Graph<String, ()>,
    indexes: HashMap<String, NodeIndex>,
    inverse_indexes: HashMap<NodeIndex, String>,
}

impl IndexedGraph {
    fn index(&self, node: &str) -> NodeIndex {
        self.indexes[node]
    }

    fn has_edge(&self, from: &str, to: &str) -> bool {
        let from_index = match self.indexes.get(from) {
            Some(index) => index,
            None => return false,
        };
        let to_index = match self.indexes.get(to) {
            Some(index) => index,
            None => return false,
        };

        self.graph.find_edge(*from_index, *to_index).is_some()
    }

    fn children(&self, node: &str) -> Vec<&str> {
        let node_index = self.indexes.get(node).unwrap();

        self.graph
            .neighbors(*node_index)
            .map(|n| self.inverse_indexes.get(&n).unwrap())
            .map(|s| s.as_str())
            .collect()
    }

    fn children_indexes(&self, index: NodeIndex) -> Vec<NodeIndex> {
        self.graph.neighbors(index)
            .collect()
    }

    fn name(&self, index: NodeIndex) -> &str {
        self.inverse_indexes.get(&index).unwrap()
    }
}

struct ServerRack {
    graph: IndexedGraph,
    reverse: IndexedGraph,
}

impl ServerRack {
    fn has_edge(&self, from: &str, to: &str) -> bool {
        self.graph.has_edge(from, to)
    }
    fn node_count(&self) -> usize {
        self.graph.graph.node_count()
    }
    fn edge_count(&self) -> usize {
        self.graph.graph.edge_count()
    }

    fn ways_out(&self) -> usize {
        self.ways_from_to("you", "out", &mut HashMap::new())
    }

    fn ways_from_to<'a, 'b>(&'a self, start: &'a str, end: &'a str, cache: &'b mut HashMap<&'a str, usize>) -> usize {
        log::trace!("Finding number of ways to {}", end);
        if start == end {
            log::trace!("    Reached end");
            return 1;
        }


        if let Some(cached) = cache.get(end) {
            log::trace!("   {} cached: {}", end, cached);
            return *cached;
        }

        let children = self.reverse.children(end);

        let mut num_ways = 0;

        for child in children {
            num_ways += self.ways_from_to(start, child, cache);
        }

        cache.insert(end, num_ways);
        log::trace!("   {} computed: {}", end, num_ways);
        num_ways
    }

    fn paths_out(&self) -> usize {
        let dac_to_fft = self.ways_from_to("dac", "fft", &mut HashMap::new());
        let fft_to_dac = self.ways_from_to("fft", "dac", &mut HashMap::new());

        assert!(dac_to_fft + fft_to_dac > 0);
        assert!(dac_to_fft == 0 || fft_to_dac == 0);

        let (order, count) = if dac_to_fft > 0 {
            (["svr", "dac", "fft", "out"], dac_to_fft)
        } else {
            (["svr", "fft", "dac", "out"], fft_to_dac)
        };

        log::debug!("Order: {:?}", order);

        let first_hop = self.ways_from_to(order[0], order[1], &mut HashMap::new());
        let last_hop = self.ways_from_to(order[2], order[3], &mut HashMap::new());

        log::debug!("{} ways from {} to {}", first_hop, order[0], order[1]);
        log::debug!("{} ways from {} to {}", count, order[1], order[2]);
        log::debug!("{} ways from {} to {}", last_hop, order[2], order[3]);

        first_hop * count * last_hop
    }
}

struct Device {
    name: String,
    outputs: Vec<String>,
}

impl TryFrom<&str> for Device {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(':');

        let name = parts.next().ok_or("Missing device name")?.trim().to_string();
        let outputs_part = parts.next().ok_or("Missing device outputs")?.trim().to_string();

        let outputs = outputs_part
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok(Device {
            name,
            outputs
        })
    }
}

fn parse(input: &str) -> ServerRack {
    let mut devices: Vec<Device> = input.trim().lines()
        .map(|l| Device::try_from(l).unwrap())
        .collect();

    devices.push(Device {
        name: "out".to_string(),
        outputs: Vec::new(),
    });

    connect(devices)
}

fn connect(mut devices: Vec<Device>) -> ServerRack {
    let mut graph = petgraph::Graph::new();
    let mut reverse = petgraph::Graph::new();

    let mut indexes: HashMap<String, NodeIndex> = HashMap::new();
    let mut reverse_indexes: HashMap<String, NodeIndex> = HashMap::new();
    let mut inverse_indexes = HashMap::new();
    let mut inverse_reverse_indexes = HashMap::new();

    for device in &devices {
        let index = graph.add_node(device.name.clone());
        let reverse_index = reverse.add_node(device.name.clone());
        indexes.insert(device.name.clone(), index);
        reverse_indexes.insert(device.name.clone(), reverse_index);
        inverse_indexes.insert(index, device.name.clone());
        inverse_reverse_indexes.insert(reverse_index, device.name.clone());
    }

    while let Some(device) = devices.pop() {
        let index = indexes.get(&device.name).unwrap();
        for output in device.outputs {
            log::trace!("Connecting {} -> {}", device.name, output);
            let output_index = indexes.get(&output).unwrap();
            graph.add_edge(*index, *output_index, ());
            reverse.add_edge(*output_index, *index, ());
        }
    }

    assert_eq!(graph.node_count(), reverse.node_count());
    assert_eq!(graph.edge_count(), reverse.edge_count());

    ServerRack {
        graph: IndexedGraph {
            graph,
            indexes,
            inverse_indexes

        },
        reverse: IndexedGraph {
            graph: reverse,
            indexes: reverse_indexes,
            inverse_indexes: inverse_reverse_indexes
        },
    }
}

pub async fn run_a(input: &str) -> i64 {
    let rack = parse(input);

    rack.ways_out() as i64
}

pub async fn run_b(input: &str) -> i64 {
    parse(input).paths_out() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
    }

    fn example_b() -> &'static str {
        r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());

        assert_eq!(11, parsed.node_count());
        assert_eq!(17, parsed.edge_count());
        assert!(parsed.has_edge("aaa", "you"));
    }

    #[tokio::test]
    async fn part_a_example() {
        init();
        assert_eq!(5, run_a(example()).await);
    }

    #[tokio::test]
    async fn part_b_example() {
        init();
        assert_eq!(2, run_b(example_b()).await);
    }

    mod device {
        use super::*;

        #[test]
        fn try_from() {
            let line = example().trim().lines().nth(0).unwrap();
            let device = Device::try_from(line).unwrap();

            assert_eq!(device.name, "aaa");
            assert_eq!(device.outputs, vec!["you", "hhh"]);
        }
    }
}