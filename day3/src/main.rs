#![feature(array_chunks)]
use std::fs;
use std::str;

fn find_num(input: char) -> usize {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .find_map(|(idx, c)| if c == input { Some(idx + 1) } else { None })
        .unwrap()
}

fn main() {
    // split string in half
    // check which letter is doubled between both halves
    // calculate value
    // add to sum

    let input = fs::read_to_string("./src/input.txt").unwrap();
    let input = input.lines().collect::<Vec<&str>>();

    let mut sum: usize = 0;

    // for bag in input {
    //     let mut chr_of_interest: char = ' ';
    //     let (comp_a, comp_b) = bag.split_at(bag.len() / 2);
    //     for i in comp_a.chars() {
    //         for j in comp_b.chars() {
    //             if i == j {
    //                 chr_of_interest = j;
    //             }
    //         }
    //     }
    //     sum += find_num(chr_of_interest);
    // }

    sum = input
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            find_num(common_char)
        })
        .sum::<usize>();

    println!("{}", sum);
}
