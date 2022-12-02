use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let input = input.lines().collect::<Vec<&str>>();

    let mut score = 0;

    // for i in input {
    //     let vals = i.split(' ').collect::<Vec<&str>>();
    //     match vals[1] {
    //         "X" => score += 1,
    //         "Y" => score += 2,
    //         "Z" => score += 3,
    //         _ => continue,
    //     }

    //     match vals[..] {
    //         ["A", "X"] => score += 3,
    //         ["A", "Y"] => score += 6,
    //         ["A", "Z"] => continue,
    //         ["B", "X"] => continue,
    //         ["B", "Y"] => score += 3,
    //         ["B", "Z"] => score += 6,
    //         ["C", "X"] => score += 6,
    //         ["C", "Y"] => continue,
    //         ["C", "Z"] => score += 3,
    //         _ => continue,
    //     }
    // }

    for i in input {
        let vals = i.split(' ').collect::<Vec<&str>>();
        match vals[..] {
            // A = Rock = 1
            // B = Paper = 2
            // C = Scissors = 3
            ["A", "X"] => score += 3, // loose, choose scissors
            ["A", "Y"] => score += 4, // draw, choose rock
            ["A", "Z"] => score += 8, // win, choose paper
            ["B", "X"] => score += 1, // loose, choose rock
            ["B", "Y"] => score += 5, // draw, choose paper
            ["B", "Z"] => score += 9, // win, choose scissors
            ["C", "X"] => score += 2, // loose, choose paper
            ["C", "Y"] => score += 6, // draw, choose scissors
            ["C", "Z"] => score += 7, // win, choose rock
            _ => continue,
        }
    }

    println!("{}", score);
}
