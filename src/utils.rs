pub fn parse_block_range(
    from: u64,
    to: Option<u64>,
    chunk_size: u64,
    max_block: u64,
) -> Vec<(u64, u64)> {
    let to_block = to.unwrap_or(max_block);

    let mut ranges = Vec::new();
    let mut start = from;

    while start <= to_block {
        let end = std::cmp::min(start + chunk_size - 1, to_block);
        ranges.push((start, end));
        start = end + 1;
    }

    ranges
}
