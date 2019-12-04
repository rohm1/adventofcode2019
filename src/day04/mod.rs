pub fn run (lines: Vec<String>) {
    let mut range_start= 1;
    let mut range_end= 1;
    let mut i = 0;
    for v in lines[0].split('-') {
        let v: i32 = v.parse().unwrap();
        if i == 0 {
            range_start = v;
        } else {
            range_end = v;
        }
        i += 1;
    }

    let mut valid_passwords_1 = 0;
    let mut valid_passwords_2 = 0;
    for p in range_start..=range_end {
        let password = p.to_string();
        let bytes = password.as_bytes();
        if valid_password_1(bytes) {
            valid_passwords_1 += 1;

            if valid_password_2(bytes) {
                valid_passwords_2 += 1;
            }
        }
    }

    println!("part 1: {}", valid_passwords_1);
    println!("part 2: {}", valid_passwords_2);
}

fn valid_password_1(bytes: &[u8]) -> bool
{
    if bytes.len() != 6 {
        return false;
    }

    let mut doubles = false;
    let mut ascending = true;
    for i in 0..5 {
        if bytes[i] == bytes[i+1] {
            doubles = true;
        }

        if bytes[i] > bytes[i+1] {
            ascending = false;
        }
    }

    doubles && ascending
}

fn valid_password_2(bytes: &[u8]) -> bool {
    let mut i = 0;
    loop {
        let mut j = i;
        loop {
            if j != 5 && bytes[i] == bytes[j + 1] {
                j += 1;
            } else {
                break;
            }
        }

        if j > i {
            if j - i == 1 {
                return true;
            }

            i = j - 1;
        }

        i += 1;
        if i == 6 {
            break
        }
    }

    false
}
