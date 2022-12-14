package main

import (
	"fmt"
	"os"
	"bufio"
	"encoding/json"
	"reflect"
)

const input_file string = "input.txt"

func compare(left []any, right []any) int {
	for i := 0; i < len(left); i++ {
		if i == len(right) {
			return -1
		}
		leftType := reflect.TypeOf(left[i])
		rightType := reflect.TypeOf(right[i])
		if leftType == rightType && leftType.Name() == "float64" {
			//both items are numbers
			if left[i].(float64) > right[i].(float64) {
				return -1
			} else if left[i].(float64) < right[i].(float64) {
				return 0
			}
		} else if leftType == rightType {
			res := compare(left[i].([]any), right[i].([]any))
			 if  res != 1 {
				 return res
			 }
		} else {
			//one number, one slice
			if leftType.Name() == "float64" {
				res := compare([]any{left[i]}, right[i].([]any))
				if  res != 1 {
					return res
				}
			} else {
				res := compare(left[i].([]any), []any{right[i]})
				if res != 1 {
					return res
				}
			}
		}
	}
	if len(left) < len(right) {
		return 0
	}
	return 1
}

func partOne(lines []string) {
	sum := 0
	pair := 0
	for i := 0; i < len(lines); i += 3 {
		pair++
		var line1Json []any
		json.Unmarshal([]byte(lines[i]), &line1Json)
		var line2Json []any
		json.Unmarshal([]byte(lines[i+1]), &line2Json)
		if compare(line1Json, line2Json) == 0 {
			sum += pair
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func partTwo(lines []string) {
	data := make([][]any, 0)
	for _, line := range lines {
		if line == "" {
			continue
		}
		var lj []any
		json.Unmarshal([]byte(line), &lj)
		data = append(data, lj)
	}
	var divider1 []any
	json.Unmarshal([]byte("[[2]]"), &divider1)
	data = append(data, divider1)
	var divider2 []any
	json.Unmarshal([]byte("[[6]]"), &divider2)
	data = append(data, divider2)
	for i := 0; i < len(data); i++ {
		for j := i+1; j < len(data); j++ {
			if compare(data[i], data[j]) == -1 {
				data[i], data[j] = data[j], data[i]
			}
		}
	}
	res := 1
	for i, v := range data {
		if compare(v, divider1) == 1 || compare(v, divider2) == 1 {
			res = res * (i+1)
		}
	}
	fmt.Printf("Part 2: %v\n", res)
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