// Day 1
pub fn count_increases(data: &[u32]) -> usize {
    data.windows(2).filter(|win| win[0] < win[1]).count()
}
