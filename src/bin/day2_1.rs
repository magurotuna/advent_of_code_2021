enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

const ERROR: &str = "failed to convert";

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace();
        let inst = iter.next().ok_or(ERROR)?;
        let delta = iter
            .next()
            .ok_or(ERROR)?
            .parse::<i32>()
            .map_err(|_| ERROR)?;

        let result = match inst {
            "forward" => Instruction::Forward(delta),
            "down" => Instruction::Down(delta),
            "up" => Instruction::Up(delta),
            _ => {
                return Err(ERROR);
            }
        };

        Ok(result)
    }
}

fn main() {
    let input = include_str!("../../input/day2_1.txt");
    let insts = input
        .split('\n')
        .filter_map(|line| line.try_into().ok())
        .collect::<Vec<Instruction>>();

    let mut depth = 0;
    let mut horizontal = 0;

    for inst in insts {
        match inst {
            Instruction::Forward(d) => {
                horizontal += d;
            }
            Instruction::Down(d) => {
                depth += d;
            }
            Instruction::Up(d) => {
                depth -= d;
            }
        }
    }

    println!("{}", depth * horizontal);
}
