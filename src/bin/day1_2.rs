fn main() {
    let input = include_str!("../../input/day1_2.txt");
    let numbers = input
        .split('\n')
        .filter_map(|line| line.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut iter = numbers
        .windows(3)
        .filter_map(|w| {
            if let &[a, b, c] = w {
                Some(a + b + c)
            } else {
                None
            }
        })
        .peekable();

    let mut ans = 0;
    while let Some(cur) = iter.next() {
        if let Some(&next) = iter.peek() {
            if next > cur {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
