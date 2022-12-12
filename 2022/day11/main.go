package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
	"sort"
)

const input_file string = "input.txt"

type monkey struct {
	items []int
	operation string
	operationValue int
	testValue int
	targetIndices [2]int
	inspectionCount int
}

func newMonkey(items []int, operation string, operationValue int, testValue int, targetIndices [2]int) *monkey {
	m := monkey{items: items, operation: operation, operationValue: operationValue, testValue: testValue, targetIndices: targetIndices, inspectionCount: 0}
	return &m
}

func partOne(lines []string) {
	monkeys := make([]*monkey, 0)
	for i := 0; i < len(lines); i += 7 {
		itemStrings := strings.Split(strings.Split(lines[i+1], ": ")[1], ", ")
		items := make([]int, 0)
		for _, item := range itemStrings {
			i, _ := strconv.Atoi(item)
			items = append(items, i)
		}
		operationParts := strings.Split(strings.Split(lines[i+2], "= ")[1], " ")
		operation := operationParts[1]
		operationValue, err := strconv.Atoi(operationParts[2])
		if err != nil {
			if operation == "+" {
				operation = "double"
			} else {
				operation = "square"
			}
		}
		testValue, _ := strconv.Atoi(strings.Split(lines[i+3], "by ")[1])
		targetIndex1, _ :=  strconv.Atoi(strings.Split(lines[i+4], "monkey ")[1])
		targetIndex2, _ :=  strconv.Atoi(strings.Split(lines[i+5], "monkey ")[1])
		m := newMonkey(items, operation, operationValue, testValue, [2]int{targetIndex1, targetIndex2})
		monkeys = append(monkeys, m)
	}
	
	for i := 0; i < 20; i++ {
		for _, m := range monkeys {
			for _, item := range m.items {
				m.inspectionCount++
				worry := item
				if m.operation == "*" {
					worry = worry * m.operationValue
				} else if m.operation == "+" {
					worry = worry + m.operationValue
				} else if m.operation == "double" {
					worry = worry * 2
				} else if m.operation == "square" {
					worry = worry * worry
				}
				worry = worry / 3
				if worry % m.testValue == 0 {
					monkeys[m.targetIndices[0]].items = append(monkeys[m.targetIndices[0]].items, worry)
				} else {
					monkeys[m.targetIndices[1]].items = append(monkeys[m.targetIndices[1]].items, worry)
				}
			}
			m.items = make([]int, 0)
		}
	}
	sort.Slice(monkeys, func(i, j int) bool {
		return monkeys[i].inspectionCount > monkeys[j].inspectionCount
	})
	fmt.Printf("Part 1: %v\n", monkeys[0].inspectionCount * monkeys[1].inspectionCount)
}

func partTwo(lines []string) {
	monkeys := make([]*monkey, 0)
	modConstant := 1
	for i := 0; i < len(lines); i += 7 {
		itemStrings := strings.Split(strings.Split(lines[i+1], ": ")[1], ", ")
		items := make([]int, 0)
		for _, item := range itemStrings {
			i, _ := strconv.Atoi(item)
			items = append(items, i)
		}
		operationParts := strings.Split(strings.Split(lines[i+2], "= ")[1], " ")
		operation := operationParts[1]
		operationValue, err := strconv.Atoi(operationParts[2])
		if err != nil {
			if operation == "+" {
				operation = "double"
			} else {
				operation = "square"
			}
		}
		testValue, _ := strconv.Atoi(strings.Split(lines[i+3], "by ")[1])
		modConstant *= testValue
		targetIndex1, _ :=  strconv.Atoi(strings.Split(lines[i+4], "monkey ")[1])
		targetIndex2, _ :=  strconv.Atoi(strings.Split(lines[i+5], "monkey ")[1])
		m := newMonkey(items, operation, operationValue, testValue, [2]int{targetIndex1, targetIndex2})
		monkeys = append(monkeys, m)
	}
	
	for i := 0; i < 10000; i++ {
		for _, m := range monkeys {
			for _, item := range m.items {
				m.inspectionCount++
				worry := item
				if m.operation == "*" {
					worry = worry * m.operationValue
				} else if m.operation == "+" {
					worry = worry + m.operationValue
				} else if m.operation == "double" {
					worry = worry * 2
				} else if m.operation == "square" {
					worry = worry * worry
				}
				if worry % m.testValue == 0 {
					monkeys[m.targetIndices[0]].items = append(monkeys[m.targetIndices[0]].items, worry % modConstant)
				} else {
					monkeys[m.targetIndices[1]].items = append(monkeys[m.targetIndices[1]].items, worry % modConstant)
				}
			}
			m.items = make([]int, 0)
		}
	}
	sort.Slice(monkeys, func(i, j int) bool {
		return monkeys[i].inspectionCount > monkeys[j].inspectionCount
	})
	fmt.Printf("Part 2: %v * %v =  %v\n", monkeys[0].inspectionCount, monkeys[1].inspectionCount, monkeys[0].inspectionCount * monkeys[1].inspectionCount)
}


func main() {
    file, _ := os.Open(input_file)
    defer file.Close()

    scanner := bufio.NewScanner(file)
	var lines []string
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

	partOne(lines)
	partTwo(lines)
}