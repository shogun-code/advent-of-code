#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in _input.lines() {
        let mut tokens = line.split_whitespace();
        left.push(tokens.next().unwrap().parse::<i32>().unwrap());
        right.push(tokens.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result = std::iter::zip(left, right)
        .map(|(a, b)| (a-b).abs())
        .sum::<i32>();

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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
