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

fn print_img(img: &Vec<u32>, width: usize) {
    println!("Answer 2:");
    for chunk in img.chunks_exact(width) {
        println!("{:?}", chunk);
    }
}

fn solution2(input: &String, width: usize, height: usize) -> Vec<u32> {
    let layers = get_layers(&input, &width, &height);
    let first_layer = layers[0].clone();

    let result = first_layer.into_iter()
        .enumerate()
        .map ( |(idx, pixel)| {
            layers.iter()
                .skip(1) // 1st layer is already present
                .fold(pixel, |acc, layer| {
                    let r: u32;
                    match acc {
                        0 => { r = 0},
                        1 => { r = 1},
                        2 => { r = layer[idx]},
                        _ => panic!("Invalid pixel value")
                    }
                    r
                })
        })
        .collect();
    result
}

const IM_WIDTH: usize = 25;
const IM_HEIGHT: usize = 6;
fn main() {
    println!("Answer 1: {}", solution1(&input::puzzle_input(), IM_WIDTH, IM_HEIGHT));
    print_img(&solution2(&input::puzzle_input(), IM_WIDTH, IM_HEIGHT), IM_WIDTH);
}

