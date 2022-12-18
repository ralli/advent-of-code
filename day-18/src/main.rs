use std::collections::{HashSet, VecDeque};

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-18/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, cubes) = cubes(input).unwrap();
    let cube_map = HashSet::from_iter(cubes.iter());

    cubes
        .iter()
        .map(|cube| number_of_free_sides(cube, &cube_map))
        .sum()
}

fn number_of_free_sides(cube: &(i32, i32, i32), cube_map: &HashSet<&(i32, i32, i32)>) -> i32 {
    let (x, y, z) = *cube;

    [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .iter()
    .filter(|(dx, dy, dz)| !cube_map.contains(&(x + *dx, y + *dy, z + *dz)))
    .count() as i32
}

fn part2(input: &str) -> i32 {
    let (_, cubes) = cubes(input).unwrap();
    // minimum x, y, z coordinate minus (1, 1, 1)
    let (xmin, ymin, zmin) = find_min(&cubes);
    // maximum x, y, z coordinate plus (1, 1, 1)
    let (xmax, ymax, zmax) = find_max(&cubes);
    let cube_map: HashSet<(i32, i32, i32)> = HashSet::from_iter(cubes);
    //
    // xmin, ymin, zmin are 1 smaller than the smallest coordinate of the cube
    // so xmin, ymin, zmin is guaranteed to lie outside of the blob
    //
    let mut q = VecDeque::from([(xmin, ymin, zmin)]);
    let directions = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    let mut result = 0;
    let mut visited = HashSet::from([(xmin, ymin, zmin)]);

    //
    // Flood fill of the outer area of the blob.
    // Each time we find a lava-cube, we have found an outer side of the blob
    //
    while let Some((x, y, z)) = q.pop_front() {
        for &(dx, dy, dz) in directions.iter() {
            let (next_x, next_y, next_z) = (x + dx, y + dy, z + dz);
            if (xmin..=xmax).contains(&next_x)
                && (ymin..=ymax).contains(&next_y)
                && (zmin..=zmax).contains(&next_z)
            {
                if cube_map.contains(&(next_x, next_y, next_z)) {
                    result += 1;
                } else if !visited.contains(&(next_x, next_y, next_z)) {
                    q.push_back((next_x, next_y, next_z));
                    visited.insert((next_x, next_y, next_z));
                }
            }
        }
    }

    result
}

fn find_min(cubes: &[(i32, i32, i32)]) -> (i32, i32, i32) {
    let (mut xmin, mut ymin, mut zmin) = cubes.first().copied().unwrap_or((0, 0, 0));
    for &(x, y, z) in cubes {
        xmin = xmin.min(x);
        ymin = ymin.min(y);
        zmin = zmin.min(z);
    }
    (xmin - 1, ymin - 1, zmin - 1)
}

fn find_max(cubes: &[(i32, i32, i32)]) -> (i32, i32, i32) {
    let (mut xmax, mut ymax, mut zmax) = cubes.first().copied().unwrap_or((0, 0, 0));
    for &(x, y, z) in cubes {
        xmax = xmax.max(x);
        ymax = ymax.max(y);
        zmax = zmax.max(z);
    }
    (xmax + 1, ymax + 1, zmax + 1)
}

fn cubes(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    separated_list1(line_ending, cube)(input)
}

fn cube(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i32(input)?;

    Ok((input, (x, y, z)))
}
fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 64;
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 58;
        assert_eq!(result, expected);
    }
}
