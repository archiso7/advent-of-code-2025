import unittest

def problem_one(input: str) -> int:
    array: list[list[str]] = [[] for _ in range(len(input[:input.index("\n")].split()))]
    for line in input.split("\n"):
        for j, value in enumerate(line.split()):
            array[j].append(value)

    total: int = 0
    for a in array:
        symbol = a[-1]
        total += int(eval(symbol.join(a[:-1])))
    return total

def problem_two(input: str) -> int:
    return 0

class TestProblemOne(unittest.TestCase):
    def test_example(self):
        self.assertEqual(problem_one("""123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"""), 4277556)


# class TestProblemTwo(unittest.TestCase):
#     def test_example(self):
#         self.assertEqual(problem_two("""
# """), 14)
#
def main():
    with open("day6.txt", "r") as f:
        data = f.read()
    print(f"Problem 1: {problem_one(data)}")
    print(f"Problem 2: {problem_two(data)}")


if __name__ == "__main__":
    main()
