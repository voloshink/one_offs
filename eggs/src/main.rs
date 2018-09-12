fn main() {
    let (min, max) = test_method(&find_egg_binary);
    println!("Binary Search: Worst Case: {}, Best Case: {}", max, min);

    let (min, max) = test_method(&find_egg_const_step);
    println!(
        "Constant Step Search: Worst Case: {}, Best Case: {}",
        max, min
    );

    let (min, max) = test_method(&find_egg_dynamic_step);
    println!(
        "Dynamic Step Search: Worst Case: {}, Best Case: {}",
        max, min
    );
}

fn test_method(f: &Fn(u8) -> u8) -> (u8, u8) {
    let mut min = u8::max_value();
    let mut max = u8::min_value();

    for i in 1..101 {
        let drops = f(i);
        if drops > max {
            max = drops;
        }

        if drops < min {
            min = drops;
        }
    }

    (min, max)
}

fn find_egg_binary(answer: u8) -> u8 {
    assert!(
        answer <= 100,
        "Answer can't be less than the total number of floors"
    );
    let mut drops = 0;

    let mut point = 0;
    let mut safe_floor = 0;
    loop {
        drops += 1;
        if 100 - point == 1 {
            point += 1;
        } else {
            point = ((100 - point) / 2) + point;
        }

        if answer > point {
            safe_floor = point;
            continue;
        } else if answer == point {
            safe_floor = point;
            break;
        } else {
            break;
        }
    }

    while safe_floor != answer {
        safe_floor += 1;
        drops += 1;
    }

    drops
}

fn find_egg_const_step(answer: u8) -> u8 {
    assert!(
        answer <= 100,
        "Answer can't be less than the total number of floors"
    );

    let step = 10;

    let mut drops = 0;
    let mut safe_floor = 0;
    let mut point = 0;

    loop {
        drops += 1;
        point += step;
        if answer > point {
            safe_floor = point;
            continue;
        } else if answer == point {
            safe_floor = point;
            break;
        } else {
            break;
        }
    }

    while safe_floor != answer {
        safe_floor += 1;
        drops += 1;
    }

    drops
}

fn find_egg_dynamic_step(answer: u8) -> u8 {
    assert!(
        answer <= 100,
        "Answer can't be less than the total number of floors"
    );

    // n∗(n+1)/2 = floors
    let mut step = 14;

    let mut drops = 0;
    let mut safe_floor = 0;
    let mut point = 0;

    loop {
        drops += 1;
        point += step;
        if answer > point {
            step -= 1;
            safe_floor = point;
            continue;
        } else if answer == point {
            safe_floor = point;
            break;
        } else {
            break;
        }
    }

    while safe_floor != answer {
        safe_floor += 1;
        drops += 1;
    }

    drops
}
