use std::collections::HashMap;

static SANITY_INPUTS_1: &'static [(i32, bool)] = &[
    (111111, true),
    (223450, false),
    (123789, false),
    (122345, true),
    (111123, true),
    (135679, false)
];

static SANITY_INPUTS_2: &'static [(i32, bool)] = &[
    (112233, true),
    (123444, false),
    (111122, true)
];

fn validate_password(password: i32, can_group: bool) -> bool {
    let mut prev: i32 = -1;
    let mut pass: i32 = password;
    let mut dupplicates: HashMap<i32, u32> = HashMap::new();
    while pass > 0 {
        let digit: i32 = pass % 10;
        pass = pass / 10;
        if prev == -1 {
            prev = digit;
            continue;
        }
        if prev < digit {
            return false
        }
        if prev == digit {
            *dupplicates.entry(digit).or_insert(0) += 1;
        }
        prev = digit;
    }
    let groups: usize = dupplicates.keys().count();
    if groups == 0 {
        return false
    }
    if !can_group {
        for k in dupplicates.keys() {
            if *dupplicates.get(k).unwrap() == 1 as u32 {
                return true
            }
        }
        return false
    }
    true
}

fn sanity_checks() -> () {
    for si in SANITY_INPUTS_1 {
        assert_eq!(validate_password(si.0, true), si.1, "{} should be: {}", si.0, si.1);
    }
    for si in SANITY_INPUTS_2 {
        assert_eq!(validate_password(si.0, false), si.1, "{} should be: {}", si.0, si.1);
    }
}

fn puzzle_check(can_group: bool) -> u32 {
    let mut valid_count: u32 = 0;
    for pass in 172930 .. 683082 {
        valid_count += if validate_password(pass, can_group) {1} else {0};
    }
    valid_count
}

pub fn solution() -> () {
    sanity_checks();
    println!("Answer 1: {}", puzzle_check(true));
    println!("Answer 2: {}", puzzle_check(false));
}
