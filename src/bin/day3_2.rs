fn main() {
    let input = include_str!("../../input/day3_2.txt");
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

    let oxygen = calc_oxygen(&lines);
    let co2 = calc_co2(&lines);

    println!("{}", oxygen * co2);
}

#[derive(Copy, Clone)]
enum RatingKind {
    Oxygen,
    Co2,
}

fn calc_oxygen(lines: &[Vec<char>]) -> u32 {
    calc(lines, RatingKind::Oxygen)
}

fn calc_co2(lines: &[Vec<char>]) -> u32 {
    calc(lines, RatingKind::Co2)
}

fn calc(lines: &[Vec<char>], rating_kind: RatingKind) -> u32 {
    let mut candidate_lines = (0..lines.len()).collect::<Vec<_>>();

    let line_len = lines.get(0).expect("No inputs given").len();
    for i in 0..line_len {
        if candidate_lines.len() == 1 {
            break;
        }

        filter(&mut candidate_lines, i, &lines, rating_kind);
    }

    assert_eq!(candidate_lines.len(), 1);
    interpret_binary(&lines[candidate_lines[0]])
}

fn filter(
    candidate_lines: &mut Vec<usize>,
    cur_index: usize,
    lines: &[Vec<char>],
    rating_kind: RatingKind,
) {
    let mut ones = Vec::new();
    let mut zeros = Vec::new();

    for &candidate in candidate_lines.iter() {
        match lines[candidate][cur_index] {
            '0' => zeros.push(candidate),
            '1' => ones.push(candidate),
            _ => unreachable!(),
        }
    }

    *candidate_lines = match rating_kind {
        RatingKind::Oxygen => match (zeros.len(), ones.len()) {
            (x, y) if x > y => zeros,
            (x, y) if x <= y => ones,
            _ => unreachable!(),
        },
        RatingKind::Co2 => match (zeros.len(), ones.len()) {
            (x, y) if x <= y => zeros,
            (x, y) if x > y => ones,
            _ => unreachable!(),
        },
    };
}

fn interpret_binary(binary: &[char]) -> u32 {
    let mut ret = 0;
    for (place, &number) in binary.iter().rev().enumerate() {
        if number == '1' {
            ret += 2_u32.pow(place as u32);
        }
    }
    ret
}
