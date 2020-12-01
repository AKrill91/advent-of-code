use std::collections::{HashMap, HashSet};

use ego_tree::{Tree, NodeMut};

struct SpaceObject<'a> {
    label: &'a str,
    depth: i32
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let tree = parse_orbits(input);

    sum_depths(&tree)
}

pub fn run_b(input: &Vec<String>) -> i32 {
    let (parents, _) = parse_relations(input);

    let you_stack = build_lineage(&parents, "YOU");
    let santa_stack = build_lineage(&parents, "SAN");

    let mut you_shared_index = 0;
    let mut santa_shared_index = 0;

    for you_pos in 0..you_stack.len() {
        let you_label = you_stack[you_pos];

        if santa_stack.contains(&you_label) {
            you_shared_index = you_pos;

            for santa_pos in 0..santa_stack.len() {
                let santa_label = santa_stack[santa_pos];

                if santa_label == you_label {
                    santa_shared_index = santa_pos;
                    break
                }
            }
            break
        }
    }

    (you_shared_index + santa_shared_index - 2)  as i32
}

fn parse_relations(input: &Vec<String>) -> (HashMap<&str, &str>, HashMap<&str, Vec<&str>>) {
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input {
        let parts = line.split(")").collect::<Vec<&str>>();

        let parent: &str = parts[0];
        let child: &str = parts[1];

        parents.insert(child, parent);

        if !children.contains_key(parent) {
            children.insert(parent, vec![]);
        }

        let kids = children.get_mut(parent).unwrap();

        kids.push(child);
    }

    (parents, children)
}

fn parse_orbits(input: &Vec<String>) -> Tree<SpaceObject> {
    let (parents, children) = parse_relations(input);

    let have_parent: HashSet<&str> = parents.keys().cloned().collect();
    let have_children: HashSet<&str> = children.keys().cloned().collect();

    let no_parents = have_children.difference(&have_parent);

    let mut roots: Vec<&str> = vec![];

    for root in no_parents {
        roots.push(*root);
    }

    assert!(1 == roots.len());

    let root_label = roots[0];

    let mut tree = ego_tree::Tree::new(
        SpaceObject{
            label:root_label,
            depth: 0
        }
    );

    let root = tree.root_mut();

    build_node(&children, root);

    tree
}

fn build_node<'a>(children: &HashMap<&'a str, Vec<&'a str>>, mut me: NodeMut<SpaceObject<'a>>) {
    let my_label = me.value().label;
    let my_depth = me.value().depth;

    match children.get(my_label) {
        Some(child_labels) => {
            for child_label in child_labels {
                let child = SpaceObject {
                    label: child_label,
                    depth: my_depth + 1
                };

                let kid = me.append(child);

                build_node(children, kid);
            }
        }
        None => {}
    };
}

fn sum_depths(tree: &Tree<SpaceObject>) -> i32 {
    let mut sum = 0;

    for object in tree.values() {
        sum += object.depth;
    }

    sum
}

fn build_lineage<'a>(parents: &HashMap<&'a str, &'a str>, label: &'a str) -> Vec<&'a str> {
    let mut out = vec![];
    out.push(label);

    let mut parent = parents.get(&label);

    while parent.is_some() {
        let parent_label = *parent.unwrap();
        out.push(parent_label);
        parent = parents.get(parent_label);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("COM)B"),
            String::from("B)C"),
            String::from("C)D"),
            String::from("D)E"),
            String::from("E)F"),
            String::from("B)G"),
            String::from("G)H"),
            String::from("D)I"),
            String::from("E)J"),
            String::from("J)K"),
            String::from("K)L"),
        ];

        assert_eq!(42, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("COM)B"),
            String::from("B)C"),
            String::from("C)D"),
            String::from("D)E"),
            String::from("E)F"),
            String::from("B)G"),
            String::from("G)H"),
            String::from("D)I"),
            String::from("E)J"),
            String::from("J)K"),
            String::from("K)L"),
            String::from("K)YOU"),
            String::from("I)SAN")
        ];

        assert_eq!(4, run_b(&input));
    }
}