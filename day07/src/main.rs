use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::io::{self, BufRead};
use regex::Regex;

struct Dependency {
    number: usize,
    node: Rc<RefCell<Node>>,
}

#[derive(Default)]
struct Node {
    color: String,
    contains: Vec<Dependency>,
    is_contained: BTreeSet<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(color: &String) -> Node {
        let mut result = Node::default();
        result.color = color.clone();
        return result;
    }

    fn get_total_bag_count(&self) -> usize {
        let mut result: usize = 1; // 1 == self
        for c in self.contains.iter() {
            result += c.number * c.node.borrow().get_total_bag_count();
        }
        return result;
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.color == other.color
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.color.cmp(&other.color)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    graph: HashMap<String, Rc<RefCell<Node>>>,
}

impl Graph {
    fn new() -> Graph {
        Graph { graph: HashMap::new() }
    }

    fn add_node(&mut self, color: &String) {
        if self.graph.contains_key(color) { return; }
        let node = Rc::new(RefCell::new(Node::new(color)));
        self.graph.insert(color.clone(), node);
    }

    fn add_dependency(&mut self, color: &String, number: usize, dependent: &String) {
        let this: &Rc<RefCell<Node>> = self.graph.get(color).unwrap();
        let other: &Rc<RefCell<Node>> = self.graph.get(dependent).unwrap();

        this.borrow_mut().contains.push(Dependency { number: number, node: other.clone() });
        other.borrow_mut().is_contained.insert(this.clone());
    }

    fn print_node(&self, color: &str) {
        let node: &Rc<RefCell<Node>> = self.graph.get(color).unwrap();
        println!("my_color: {}", node.borrow().color);
        println!("contains: ");
        for dep in node.borrow().contains.iter() {
            println!("  {}x {}", dep.number, dep.node.borrow().color);
        }
        println!("in: ");
        for dep in node.borrow().is_contained.iter() {
            println!("  {}", dep.borrow().color);
        }
    }

    fn get_node(&self, color: &str) -> Rc<RefCell<Node>> {
        self.graph.get(color).unwrap().clone()
    }

    fn get_direct_parents(&self, color: &str) -> HashSet<String> {
        let node: &Rc<RefCell<Node>> = self.graph.get(color).unwrap();
        let mut parents: HashSet<String> = HashSet::new();
        for dep in node.borrow().is_contained.iter() {
            parents.insert(dep.borrow().color.clone());
        }
        return parents;
    }
}

fn main() {
    let re = Regex::new(r"([0-9]+) (\w+ \w+)").unwrap();

    let mut graph = Graph::new();

    let stdin = io::stdin();
    for strange_line in stdin.lock().lines() {

        // One line is one rule -> split into the name of the bag 'bag_color' and all 'contained_bags'
        let simplified_line: String = strange_line.unwrap().replace("bags", "").replace("bag", "").replace(".", "");
        let mut parts = simplified_line.split("contain");
        let bag_color = parts.next().unwrap().trim().to_string();
        let contained_bags = parts.next().unwrap().trim();

        graph.add_node(&bag_color);

        // Process the 'contained_bags'
        for rule in contained_bags.split(',') {
            let bag: String = rule.to_string().trim().to_string();
            if bag == "no other" { break; }

            let caps = re.captures(&bag).unwrap();
            let number = caps.get(1).map_or("xx", |m| m.as_str());
            let dependent_number = number.parse::<usize>().unwrap();
            let dependent_color = caps.get(2).map_or("xx", |m| m.as_str()).to_string();

            // println!("{} -> {}x {}", bag_color, dependent_number, dependent_color);
            // Add the node and the dependency
            graph.add_node(&dependent_color);
            graph.add_dependency(&bag_color, dependent_number, &dependent_color);
        }
    }

    graph.print_node("shiny gold");
    println!("---");

    let mut parents: HashSet<String> = graph.get_direct_parents("shiny gold");

    loop {
        let mut new_parents = parents.clone();
        for p in parents.iter() {
            let direct_parents = graph.get_direct_parents(p);
            new_parents = new_parents.union(&direct_parents).map(|s| s.to_string()).collect();
        }
        if parents.len() == new_parents.len() { break; }
        parents = new_parents;
    }

    for p in parents.iter() {
        println!("{}", p);
    }
    println!("Total number: {}", parents.len());

    let required_bag_count = graph.get_node("shiny gold").borrow().get_total_bag_count();
    println!("Required bags: {}", required_bag_count);
}
