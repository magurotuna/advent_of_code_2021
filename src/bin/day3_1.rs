fn main() {
    let input = include_str!("../../input/day3_1.txt");
    let lines = input
        .split('\n')
        .filter_map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            if chars.is_empty() {
                None
            } else {
                Some(chars)
            }
        })
        .collect::<Vec<_>>();

    let line_len = lines.get(0).expect("No inputs given").len();
    let mut gamma = 0_u32;
    let mut epsilon = 0_u32;
    for i in 0..line_len {
        let index = line_len - i - 1;
        let x = 2_u32.pow(i as u32);
        if one_appears_more_often(&lines, index) {
            gamma += x;
        } else {
            epsilon += x;
        }
    }

    println!("{}", gamma * epsilon);
}

fn one_appears_more_often(lines: &[Vec<char>], index: usize) -> bool {
    let mut one_count = 0;
    let mut zero_count = 0;

    for line in lines {
        if let Some(&ch) = line.get(index) {
            match ch {
                '1' => one_count += 1,
                '0' => zero_count += 1,
                _ => unreachable!(),
            }
        }
    }

    one_count > zero_count
}
