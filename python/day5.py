import unittest

def problem_one(input: str) -> int:
    ranges: list[tuple[int, int]] = []
    total = 0
    for line in input.split("\n"):
        if "-" in line:
            dashIndex = line.index("-")
            ranges.append((int(line[:dashIndex]), int(line[dashIndex+1:])))
        elif line.strip():
            num = int(line)
            for r in ranges:
                if num >= r[0] and num <= r[1]:
                    total += 1
                    break
    return total

class RangeUnion:
    def __init__(self, ranges: list[tuple[int, int]]) -> None:
        self.ranges: list[tuple[int, int]] = ranges

    def concat(self, ranges: list[tuple[int, int]]) -> list[tuple[int, int]]:
        return self.ranges + ranges

    def difference(self, range: tuple[int, int]) -> None:
        new_ranges: list[tuple[int, int]] = []
        for range1 in self.ranges:
            # No overlap - keep original
            if range[1] < range1[0] or range[0] > range1[1]:
                new_ranges.append(range1)
            # Complete overlap - drop range1
            elif range[0] <= range1[0] and range[1] >= range1[1]:
                pass
            # Subtraction is strictly inside - split into two
            elif range[0] > range1[0] and range[1] < range1[1]:
                new_ranges.append((range1[0], range[0] - 1))
                new_ranges.append((range[1] + 1, range1[1]))
            # Overlap at start
            elif range[0] <= range1[0]:
                new_ranges.append((range[1] + 1, range1[1]))
            # Overlap at end
            else:
                new_ranges.append((range1[0], range[0] - 1))

        self.ranges = new_ranges

    def count(self) -> int:
        c = 0
        for range in self.ranges:
            c += range[1]-range[0]+1
        return c

def problem_two(input: str) -> int:
    ranges: list[tuple[int, int]] = []
    total = 0
    for line in input.split("\n"):
        if "-" in line:
            dashIndex = line.index("-")
            rng = RangeUnion([(int(line[:dashIndex]), int(line[dashIndex+1:]))])
            for r in ranges:
                rng.difference(r)
            ranges = rng.concat(ranges)
            total += rng.count()
    return total

def problem_two_optimal(input: str):
    points: list[tuple[int,int]] = []
    for line in input.split("\n"):
        if "-" in line:
            dashIndex = line.index("-")
            points.append((int(line[:dashIndex]), 1))
            points.append((int(line[dashIndex+1:]) + 1, -1))

    points.sort()
    
    total: int = 0
    delta: int = 0
    start: tuple[int, int] = (0,0)
    for point in points:
        if point[1] > 0 and delta == 0:
                start = point
        delta += point[1]
        if delta == 0:
            total += point[0] - start[0]

    return total

class TestProblemOne(unittest.TestCase):
    def test_example(self):
        self.assertEqual(problem_one("""3-5
10-14
16-20
12-18

1
5
8
11
17
32"""), 3)


class TestProblemTwo(unittest.TestCase):
    def test_example(self):
        self.assertEqual(problem_two_optimal("""3-5
10-14
16-20
12-18"""), 14)


def main():
    with open("day5.txt", "r") as f:
        data = f.read()
    print(f"Problem 1: {problem_one(data)}")
    print(f"Problem 2: {problem_two_optimal(data)}")


if __name__ == "__main__":
    main()
