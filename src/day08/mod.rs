const HEIGHT: usize = 6;
const WIDTH: usize = 25;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

pub fn run (lines: Vec<String>) {
    let bytes = lines[0].as_bytes();
    let layers_count = bytes.len() / LAYER_SIZE;
    let mut zeros_count = LAYER_SIZE + 1;
    let mut check_code = 0;
    let mut image: [u8; LAYER_SIZE] = [TRANSPARENT; LAYER_SIZE];

    for l in 0..layers_count {
        let mut z = 0;
        let mut one = 0;
        let mut two = 0;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = y * WIDTH + x;
                match bytes[l * LAYER_SIZE + pos] {
                    BLACK => {
                        z += 1;
                        if image[pos] == TRANSPARENT {
                            image[pos] = BLACK;
                        }
                    },
                    WHITE => {
                        one += 1;
                        if image[pos] == TRANSPARENT {
                            image[pos] = WHITE;
                        }
                    },
                    TRANSPARENT => two += 1,
                    _ => continue,
                };
            }
        }

        if z < zeros_count {
            zeros_count = z;
            check_code = one * two;
        }
    }

    println!("part 1: {}", check_code);
    println!("part 2:");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pos = y * WIDTH + x;
            print!("{}", if image[pos] == WHITE { '#' } else { ' ' });
        }
        println!();
    }
}
