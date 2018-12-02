#!/usr/bin/env python

"""Solution to Advent of Code 2018 Day 2, Challenge 1: Inventory Management System"""
__author__ = "Timothy J. Rogers"

def letter_count(s):
    letters = {}
    two = 0
    three = 0
    for l in s:
        if l in letters:
            letters[l] += 1
        else:
            letters[l] = 1
    if 2 in letters.values():
        two = 1
    if 3 in letters.values():
        three = 1
    return (two, three)

def main():
    twos = 0
    threes = 0
    with open('inventory-management-system-input.txt', 'r') as f:
        box_data = f.readlines()
    for box in box_data:
        twos_and_threes = letter_count(box)
        twos += twos_and_threes[0]
        threes += twos_and_threes[1]
    checksum = twos * threes
    print("Part 1 Answer: " + str(twos) + " * " + str(threes) + " = " + str(checksum))

if __name__ == "__main__":
    main()