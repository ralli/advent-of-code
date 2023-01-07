use anyhow::Context;
use day_16::{input_data, packet, read_file};

fn main() -> anyhow::Result<()> {
    let filename = "./day-16/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, input_data) = input_data(input).unwrap();
    let input = (input_data.as_ref(), 0usize);
    let (_, packet) = packet(input).unwrap();
    // dbg!(&packet);
    packet.sum_of_packet_versions()
}

fn part2(input: &str) -> u64 {
    let (_, input_data) = input_data(input).unwrap();
    let input = (input_data.as_ref(), 0usize);
    let (_, packet) = packet(input).unwrap();
    // dbg!(&packet);
    packet.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("A0016C880162017C3686B18A3D4780");
        let expected = 31;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2("9C0141080250320F1802104A08");
        let expected = 1;
        assert_eq!(result, expected);
    }
}
