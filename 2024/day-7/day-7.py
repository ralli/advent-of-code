from pathlib import Path

def has_solution(goal: int, current: int, values: list[int]) -> bool:
    if len(values) == 0:
        return current == goal
    if current > goal:
        return False
    rest = values[1:]
    first_value = values[0]
    return has_solution(goal, current+first_value, rest) or has_solution(goal, current*first_value, rest)
    
def has_solution2(goal: int, current: int, values: list[int]) -> bool:
    if len(values) == 0:
        return current == goal
    if current > goal:
        return False
    rest = values[1:]
    first_value = values[0]
    return has_solution2(goal, current+first_value, rest) or has_solution2(goal, current*first_value, rest) or has_solution2(goal, int(f"{current}{first_value}"), rest)

def ends_with(x: int, y: int) -> bool:
    while y > 0:
        if x % 10 != y % 10:
            return False
        x //= 10
        y //= 10
    return True

def truncate(x: int, y: int) -> bool:
    while y > 0:
        x //= 10
        y //= 10
    return x

def has_solution3(goal: int, values: int) -> bool:
    if len(values) == 0:
        return goal == 0
    
    if goal % values[-1] == 0 and has_solution3(goal // values[-1], values[:-1]): return True
    if goal >= values[-1] and has_solution3(goal - values[-1], values[:-1]): return True
    if ends_with(goal, values[-1]) and has_solution3(truncate(goal, values[-1]), values[:-1]): return True

    return False

def solve(filename: str):
    with open(Path(__file__).parent.joinpath("input.txt")) as f:
        content = f.read()
    result = 0
    result2 = 0
    for line in content.splitlines():
        [s1, s2] = line.split(": ")
        (goal, values) = (int(s1), [int(s) for s in s2.split()])
        if has_solution(goal, 0, values):
            result += goal
        if has_solution3(goal, values):
            result2 += goal
    print(result)
    print(result2)

if __name__ == "__main__":
    solve("input.txt")
