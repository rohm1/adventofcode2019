use std::collections::HashMap;

pub fn run (lines: Vec<String>) {
    let path1: HashMap<(i32, i32), i32> = run_path(&lines[0]);
    let path2: HashMap<(i32, i32), i32> = run_path(&lines[1]);

    let mut min_distance = 99999999;
    let mut best_steps = 99999999;

    for p in path1.keys() {
        if path2.contains_key(p) {
            let d = p.0.abs() + p.1.abs();
            if d < min_distance {
                min_distance = d;
            }

            let steps = *(path1.get(p).unwrap()) + *(path2.get(p).unwrap());
            if steps < best_steps {
                best_steps = steps;
            }
        }
    }

    println!("part 1: {}", min_distance);
    println!("part 2: {}", best_steps);
}

fn run_path(points: &String) -> HashMap<(i32, i32), i32> {
    let mut path: HashMap<(i32, i32), i32> = HashMap::new();

    let mut steps = 0;
    let mut x = 0;
    let mut y = 0;
    for movement in points.split(',') {
        let direction: char = movement.as_bytes()[0] as char;
        let distance = &movement[1..];
        let distance: i32 = distance.parse().unwrap();
        match direction {
            'R' => { for _x in (x+1)..=(x+distance)         { steps +=1; path.insert((_x,  y), steps); } ; x += distance },
            'L' => { for _x in ((x-distance)..=(x-1)).rev() { steps +=1; path.insert((_x,  y), steps); } ; x -= distance },
            'U' => { for _y in (y+1)..=(y+distance)         { steps +=1; path.insert(( x, _y), steps); } ; y += distance },
            'D' => { for _y in ((y-distance)..=(y-1)).rev() { steps +=1; path.insert(( x, _y), steps); } ; y -= distance },
            _ => {}
        }
    }

    path
}
