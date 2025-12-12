use anyhow::anyhow;
use winnow::ascii::{digit1, line_ending, multispace0, space1};
use winnow::combinator::{alt, eof, repeat, separated, separated_pair, terminated};
use winnow::{ModalResult, Parser};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-12.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Data {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone)]
struct Shape {
    id: usize,
    values: Vec<Vec<bool>>,
}

impl Shape {
    fn num_filled(&self) -> usize {
        self.values
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let data = terminated(parse_data, (multispace0, eof))
        .parse(input)
        .map_err(|e| anyhow!("{e}"))?;
    let result = data
        .regions
        .iter()
        .filter(|region| {
            region.width * region.height
                >= region
                    .presents
                    .iter()
                    .enumerate()
                    .map(|(i, &n)| data.shapes[i].num_filled() * n)
                    .sum::<usize>()
        })
        .count();
    Ok(result)
}

fn parse_data(input: &mut &str) -> ModalResult<Data> {
    separated_pair(parse_shapes, (line_ending, line_ending), parse_regions)
        .map(|(shapes, regions)| Data { shapes, regions })
        .parse_next(input)
}

fn parse_regions(input: &mut &str) -> ModalResult<Vec<Region>> {
    separated(1.., parse_region, line_ending).parse_next(input)
}

fn parse_region(input: &mut &str) -> ModalResult<Region> {
    (parse_dimensions, parse_presents)
        .map(|((width, height), presents)| Region {
            width: width,
            height: height,
            presents,
        })
        .parse_next(input)
}

fn parse_presents(input: &mut &str) -> ModalResult<Vec<usize>> {
    separated(1.., parse_int, space1).parse_next(input)
}

fn parse_dimensions(input: &mut &str) -> ModalResult<(usize, usize)> {
    terminated(separated_pair(parse_int, 'x', parse_int), ": ").parse_next(input)
}

fn parse_shapes(input: &mut &str) -> ModalResult<Vec<Shape>> {
    separated(0.., parse_shape, (line_ending, line_ending)).parse_next(input)
}

fn parse_shape(input: &mut &str) -> ModalResult<Shape> {
    let id = terminated(parse_int, (':', line_ending)).parse_next(input)?;
    let values = parse_rows.parse_next(input)?;
    Ok(Shape { id, values })
}

fn parse_rows(input: &mut &str) -> ModalResult<Vec<Vec<bool>>> {
    separated(0..4, parse_row, line_ending).parse_next(input)
}

fn parse_row(input: &mut &str) -> ModalResult<Vec<bool>> {
    repeat(0..4, parse_piece).parse_next(input)
}

fn parse_piece(input: &mut &str) -> ModalResult<bool> {
    alt(('.'.value(false), '#'.value(true))).parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<usize> {
    digit1.parse_to::<usize>().parse_next(input)
}
