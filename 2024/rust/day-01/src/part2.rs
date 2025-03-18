#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in _input.lines() {
        let mut tokens = line.split_whitespace();
        left.push(tokens.next().unwrap().parse::<usize>().unwrap());
        right.push(tokens.next().unwrap().parse::<usize>().unwrap());
    }

    let result = left
        .iter()
        .map(|l| {
            l * right
                .iter()
                .filter(|r| &l == r)
                .count()
        })
        .sum::<usize>();

    dbg!(result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", crate::part2::process(input)?);
        Ok(())
    }
}
