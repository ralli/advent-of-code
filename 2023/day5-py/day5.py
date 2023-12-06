import os
from dataclasses import dataclass


@dataclass()
class Function:
    mappings: list[(int, int, int)]

    def apply(self, n: int):
        for (dst_start, src_start, length) in self.mappings:
            src_end = src_start + length
            if src_start <= n < src_end:
                return n - src_start + dst_start
        return n

    def apply_ranges(self, ranges: list[(int, int)]) -> list[(int, int)]:
        results = []
        for (dst_start, src_start, length) in self.mappings:
            src_end = src_start + length
            next_ranges = []
            while ranges:
                (start, end) = ranges.pop()
                #
                # three possible intervals:
                #   before: valid, if there is a part of the input range before the current mapping interval.
                #   inner:  valid, if there is a part of the input range within the current mapping interval.
                #   after:  valid, if there is a part of the input range after the current mapping interval.
                # if the input range does not fulfill the before/within/after condition, the range becomes invalid,
                # i.e. the start of the range is greater or equal to the end of the range.
                #
                before = (start, min(src_start, end))
                within = (max(src_start, start), min(src_end, end))
                after = (max(start, src_end), end)

                if before[0] < before[1]:
                    next_ranges.append(before)

                if after[0] < after[1]:
                    next_ranges.append(after)

                if within[0] < within[1]:
                    next_start = within[0] - src_start + dst_start
                    next_end = within[1] - src_start + dst_start
                    #
                    # transformed range is already a result-
                    # the mapping ranges do not overlap, so there is no need
                    # to put the range into the next_range list
                    #
                    results.append((next_start, next_end))
            #
            # next_ranges contains the initial range with all inner found so far
            # ranges cut out
            #
            ranges = next_ranges

        #
        # add the before and after ranges to the result
        #
        # ranges contains the input range with all found inner ranges cut out
        #
        return results + ranges


@dataclass
class Input:
    seeds: list[int]
    ranges: list[(int, int)]
    functions: list[Function]


def parse_input(input: str) -> Input:
    seeds, *mappings = input.split("\n\n")
    seeds = [int(seed) for seed in seeds.split(":")[1].split()]

    bla = [mapping.split(":\n")[1].split("\n") for mapping in mappings]
    functions = []
    for m in bla:
        mappings = []
        for s in m:
            mappings.append(tuple([int(x) for x in s.split()]))
        functions.append(Function(mappings))
    ranges = [(seeds[i], seeds[i] + seeds[i + 1]) for i in range(0, len(seeds), 2)]
    return Input(seeds=seeds, ranges=ranges, functions=functions)


def part1(input: str) -> int:
    data = parse_input(input)
    results = []
    for seed in data.seeds:
        result = seed
        for f in data.functions:
            result = f.apply(result)
        results.append(result)
    return min(results)


def part2(input: str) -> int:
    data = parse_input(input)
    results = []
    for range in data.ranges:
        result = [range]
        for f in data.functions:
            result = f.apply_ranges(result)
        results = results + result
    starts = [r[0] for r in results]
    return min(starts)


def load_file(filename: str) -> str:
    # filename = "day-5.txt"
    with open(filename, "r", encoding="utf-8") as f:
        input = f.read()
    return input


input = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""

if __name__ == '__main__':
    input = load_file("day-5.txt")
    result = part1(input)
    print(result)

    result = part2(input)
    print(result)
