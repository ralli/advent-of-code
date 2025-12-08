use anyhow::anyhow;
use std::collections::BTreeMap;
use winnow::ascii::{digit1, line_ending, multispace0};
use winnow::combinator::{eof, separated, terminated};
use winnow::{ModalResult, Parser};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-08.txt")?;
    let result = part1(&input, 1000)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str, n: usize) -> anyhow::Result<usize> {
    let mut inp = input;
    let points = terminated(parse_points, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let mut connections: Vec<(usize, usize)> =
        Vec::with_capacity(points.len() * (points.len() + 1) / 2);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            connections.push((i, j));
        }
    }
    connections.sort_unstable_by_key(|&(i, j)| distance(&points[i], &points[j]));
    let mut uf = UnionFind::new(points.len());
    for &(i, j) in connections.iter().take(n) {
        uf.union_sets(i, j);
    }
    let mut sizes: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..points.len() {
        let root = uf.find_set(i);
        *sizes.entry(root).or_insert(0) += 1;
    }
    let mut sizes: Vec<_> = sizes.into_iter().collect();
    sizes.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
    Ok(sizes.iter().take(3).map(|(_, v)| *v).product())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let points = terminated(parse_points, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;

    let mut connections: Vec<(usize, usize)> =
        Vec::with_capacity(points.len() * (points.len() + 1) / 2);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            connections.push((i, j));
        }
    }
    connections.sort_unstable_by_key(|&(i, j)| distance(&points[i], &points[j]));

    let mut uf = UnionFind::new(points.len());
    let mut result: (usize, usize) = (0, 0);
    for (i, j) in connections.iter().copied() {
        uf.union_sets(i, j);
        let root = uf.find_set(i);
        if uf.sizes[root] == points.len() {
            result = (i, j);
            break;
        }
    }

    let (i, j) = result;
    let (x1, _y1, _z1) = points[i];
    let (x2, _y2, _z2) = points[j];

    Ok((x1 * x2) as usize)
}

fn distance((x1, y1, z1): &Point, (x2, y2, z2): &Point) -> i64 {
    fn sqr(a: i64) -> i64 {
        a * a
    }
    (sqr(x1 - x2) + sqr(y1 - y2) + sqr(z1 - z2)).isqrt()
}

// https://cp-algorithms.com/data_structures/disjoint_set_union.html
struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parents = Vec::with_capacity(n);
        for i in 0..n {
            parents.push(i);
        }
        let sizes = vec![1; n];
        Self { parents, sizes }
    }

    fn find_set(&mut self, a: usize) -> usize {
        if a != self.parents[a] {
            self.parents[a] = self.find_set(self.parents[a])
        }
        self.parents[a]
    }

    fn union_sets(&mut self, a: usize, b: usize) {
        let mut a = self.find_set(a);
        let mut b = self.find_set(b);
        if a == b {
            return;
        }
        if (self.sizes[a] > self.sizes[b]) {
            std::mem::swap(&mut a, &mut b);
        }
        self.parents[a] = b;
        self.sizes[b] += self.sizes[a];
        self.sizes[a] = 0;
    }
}

type Point = (i64, i64, i64);

fn parse_points(input: &mut &str) -> ModalResult<Vec<Point>> {
    separated(1.., parse_point, line_ending).parse_next(input)
}

fn parse_point(input: &mut &str) -> ModalResult<Point> {
    (parse_int, ',', parse_int, ',', parse_int)
        .map(|(x, _, y, _, z)| (x, y, z))
        .parse_next(input)
}
fn parse_int(input: &mut &str) -> ModalResult<i64> {
    digit1.parse_to::<i64>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_part1() {
        let result = part1(INPUT, 10).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 25272);
    }
}
