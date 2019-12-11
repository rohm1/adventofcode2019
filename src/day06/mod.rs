use std::collections::HashMap;

pub fn run (lines: Vec<String>) {
    let mut objects: HashMap<String, bool> = HashMap::new();
    let mut has_parent: HashMap<String, bool> = HashMap::new();
    let mut parent_to_childs: HashMap<String, Vec<String>> = HashMap::new();
    let mut child_to_parent: HashMap<String, String> = HashMap::new();

    for line in lines {
        let s: Vec<&str> = line.split(')').collect();
        let parent = s[0].to_string();
        let child = s[1].to_string();

        objects.entry(parent.clone()).or_insert(true);
        objects.entry(child.clone()).or_insert(true);

        parent_to_childs
            .entry(parent.clone())
            .or_insert(Vec::new())
            .push(child.clone());

        child_to_parent.insert(child.clone(), parent.clone());
        has_parent.entry(child.clone()).or_insert(true);
    }

    let mut first = String::new();
    for (object, _) in objects {
        if !has_parent.contains_key(&object) {
            first = object;
            break;
        }
    }

    println!("part 1: {}", visit_object(&parent_to_childs, first, 0));
    println!("part 2: {}", min_distance(child_to_parent));
}

fn visit_object(parent_to_childs: &HashMap<String, Vec<String>>, first: String, l: i32) -> i32
{
    if !parent_to_childs.contains_key(&first) {
        return 0;
    }

    let mut c = 0;
    for o in parent_to_childs.get(&first).unwrap() {
        c += l + 1;
        c += visit_object(parent_to_childs, (*o).to_string(), l + 1);
    }

    c
}

fn min_distance(child_to_parent: HashMap<String, String>) -> i32
{
    let mut dist = 0;
    let mut me = child_to_parent.get(&"YOU".to_string()).unwrap();
    let mut my_distances: HashMap<String, i32> = HashMap::new();

    loop {
        if !child_to_parent.contains_key(&me.to_string()) {
            break;
        }

        me = child_to_parent.get(&me.to_string()).unwrap();
        my_distances.insert(me.to_string(), dist);
        dist += 1;
    }

    let d;
    let mut dist = 0;
    let mut santa = "SAN";
    loop {
        if my_distances.contains_key(&santa.to_string()) {
            d = dist + *my_distances.get(&santa.to_string()).unwrap();
            break;
        }

        santa = child_to_parent.get(&santa.to_string()).unwrap();
        dist += 1;
    }

    d
}
