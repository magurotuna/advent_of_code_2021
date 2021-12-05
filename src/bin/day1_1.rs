fn main() {
    let input = include_str!("../../input/day1_1.txt");
    let mut iter = input
        .split('\n')
        .filter_map(|line| line.parse::<u32>().ok())
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
