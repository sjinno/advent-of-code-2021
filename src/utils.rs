// Day 1
pub fn count_increases(data: &[u32]) -> usize {
    data.windows(2)
        .fold(0, |acc, win| if win[0] < win[1] { acc + 1 } else { acc })
}
