use std::collections::HashMap;
use std::env;
use std::fs;
use std::string::String;

fn count(map: &HashMap<&str, &str>, item: &str) -> usize {
    map.get(item).map(|item| count(map, item) + 1).unwrap_or(0)
}

fn get_orbit_map(content: &String) -> HashMap<&str, &str> {
    let map: HashMap<&str, &str> = content
        .lines()
        .map(|line| line.split(")").collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut map, po| {
            map.insert(po[1], po[0]);
            map
        });
    map
}

fn orbit_count_checksum(content: &String) -> usize {
    let map: HashMap<&str, &str> = get_orbit_map(content);
    map.keys().map(|planet| count(&map, planet)).sum::<usize>()
}

fn distance(content: &String, from: &str, to: &str) -> usize {
    let map: HashMap<&str, &str> = get_orbit_map(content);
    let mut route = vec![from];
    while let Some(item) = map.get(route.last().unwrap()) {
        route.push(item);
    }

    let mut count = 0;
    let mut last = to;
    while let Some(item) = map.get(last) {
        if let Some(i) = route.iter().position(|i| item == i) {
            return count + i - 1;
        }
        last = item;
        count += 1;
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }
    let file: String = fs::read_to_string(&args[1]).unwrap();
    println!("Answer 1: {}", orbit_count_checksum(&file));
    println!("Answet 2: {}", distance(&file, "YOU", "SAN"));
}
