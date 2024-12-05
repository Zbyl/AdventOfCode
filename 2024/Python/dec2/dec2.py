import itertools
import logging
import argparse


def is_safe_ex(report: list[int], diffs: set[int]):
    for a, b in itertools.pairwise(report):
        if (b - a) not in diffs:
            return False
    return True


def is_safe(report: list[int]):
    return is_safe_ex(report, {1, 2, 3}) or is_safe_ex(report, {-1, -2, -3})

def is_safe_2(report2: list[int]):
    for i in range(len(report2)):
        report = report2[0:i] + report2[i+1:]
        res = is_safe(report)
        if res:
            print(report, res)
            return True

    print(report, res)
    return False



def main():
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser(description='Advent of Code.')
    parser.add_argument('-i', '--input', default='input.txt', help='Input file.')
    args = parser.parse_args()

    with open(args.input, 'rt') as f:
        lines = f.readlines()

    reports = []
    for line in lines:
        reports.append([int(x) for x in line.split()])

    #print(reports)
    reportsX = [
        [7, 6, 4, 2, 1],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ]

    count = 0
    for report in reports:
        if is_safe_2(report):
            count += 1

    print(count)


if __name__ == '__main__':
    main()
