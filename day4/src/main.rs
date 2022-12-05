use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let input = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;

    for i in input {
        let pairs = i.split(',').collect::<Vec<&str>>();
        let mut vals: Vec<i32> = Vec::new();

        for j in pairs {
            vals.append(
                &mut j
                    .split('-')
                    .map(|s| s.parse().unwrap()) // turn the &str to i32s before actually committing them to the vec
                    .collect::<Vec<i32>>(),
            );
        }

        // if vals[0] >= vals[2] && vals[1] <= vals[3] {
        //     sum += 1;
        // } else if vals[2] >= vals[0] && vals[3] <= vals[1] {
        //     sum += 1;
        // }

        if vals[1] >= vals[2] && vals[3] >= vals[0] {
            sum += 1;
        }
    }
    println!("{}", sum);
}
