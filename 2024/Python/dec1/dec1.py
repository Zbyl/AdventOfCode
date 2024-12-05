import itertools
import logging
import argparse
from collections import Counter


def main():
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser(description='Advent of Code.')
    parser.add_argument('-i', '--input', default='input.txt', help='Input file.')
    args = parser.parse_args()

    with open(args.input, 'rt') as f:
        lines = f.readlines()

    list0 = []
    list1 = []
    for line in lines:
        a, b = [int(x) for x in line.split()]
        list0.append(a)
        list1.append(b)

    list0.sort()
    list1.sort()

    diff = 0
    for a, b in zip(list0, list1):
        diff += abs(a - b)

    print(diff)


def main2():
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser(description='Advent of Code.')
    parser.add_argument('-i', '--input', default='input.txt', help='Input file.')
    args = parser.parse_args()

    with open(args.input, 'rt') as f:
        lines = f.readlines()

    list0 = []
    list1 = []
    for line in lines:
        a, b = [int(x) for x in line.split()]
        list0.append(a)
        list1.append(b)

    count1 = Counter(list1)

    diff = 0
    for a in list0:
        diff += a * count1[a]

    print(diff)


if __name__ == '__main__':
    #main()
    main2()
