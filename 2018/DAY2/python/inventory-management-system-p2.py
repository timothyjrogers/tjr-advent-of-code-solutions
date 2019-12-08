#!/usr/bin/env python

"""Solution to Advent of Code 2018 Day 2, Challenge 2: Inventory Management System"""
__author__ = "Timothy J. Rogers"

def strdif(s1, s2):
    diffs = 0
    for x in range(len(s1)):
        if s1[x] != s2[x]:
            diffs += 1
    return diffs

def print_common_chars(s1, s2):
    chars = []
    for c in range(len(s1)):
        if s1[c] == s2[c]:
            chars.append(s1[c])
    print(''.join(chars))

def main():
    box1 = None
    box2 = None
    with open('../inventory-management-system-input.txt', 'r') as f:
        box_data = f.readlines()
    box_data = [x.strip('\n') for x in box_data]
    data_length = len(box_data)
    itr = 0
    for x in range(itr, data_length - 1):
        for y in range(itr + 1, data_length):
            diff = strdif(box_data[x], box_data[y])
            if diff == 1:
                box1, box2 = box_data[x], box_data[y]
                break
        if box1 and box2:
            break
        itr += 1
    print("Box1: " + box1)
    print("Box2: " + box2)
    print_common_chars(box1, box2)

if __name__ == "__main__":
    main()