pub mod sanity_inputs;
pub mod input;

fn get_layers(input: &String, width: &usize, height: &usize) -> Vec<Vec<u32>> {
    let sub_size = width * height;
    let chars: Vec<char> = input.chars().collect();

    let layers: Vec<Vec<u32>> = chars.chunks_exact(sub_size)
        .map( |window| {
            window.iter().map( |&c| c.to_digit(10).unwrap() as u32 ).collect()
        })
    .collect();
    layers
}

fn count_digits(layer: &Vec<u32>, digit: u32) -> usize {
    layer.iter().filter(|&&d| d == digit).count()
}

fn solution1(input: &String, width: usize, height: usize) -> usize {
    let layers = get_layers(input, &width, &height);
    let zeros_cnt: Vec<usize> = layers.iter()
        .map(|l| {
            count_digits(l, 0)
        })
    .collect();
    let mzi: usize = zeros_cnt.iter()
        .enumerate()
        .min_by_key(|&(_, &num)| num)
        .map(|(index, _)| index)
        .unwrap() as usize;
    let ones_cnt = count_digits(&layers[mzi], 1);
    let twos_cnt = count_digits(&layers[mzi], 2);
    ones_cnt * twos_cnt
}

fn main() {
    println!("Answer 1: {}", solution1(&input::puzzle_input(), 25, 6));
}

