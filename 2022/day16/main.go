package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
)

const input_file string = "input.txt"


type node struct {
	name string
	rate int
	connections []string
}

/*
type state strut {
	openValves []string
	position string
	steps int
	totalRate int
}
*/

func valveOpen(cur *state, valve string) bool {
	for _, v := cur.openValves {
		if v == valve {
			return true
		}
	}
	return false
}

func printNode(n *node) {
	fmt.Printf("%v | %v | %v\n", n.name, n.rate, n.connections)
}

func parseLine(line string) *node {
	vt := strings.Split(line, "; ")
	valveString := strings.Split(vt[0], " ")
	name := valveString[1]
	rate, _ := strconv.Atoi(strings.Split(valveString[len(valveString)-1], "=")[1])
	connections := strings.Split(strings.Join(strings.Split(vt[1], " ")[4:], " "), ", ")
	n := node{name: name, rate: rate, connections: connections}
	return &n
}

func partOne(lines []string) {
	cave := make(map[string]*node)
	for _, line := range lines {
		n := parseLine(line)
		cave[n.name] = n
	}

	dist := make(map[string]map[string]int)
	for k, v := range cave {
		
	}
	/*
	stack := []*node{cave["AA"]}
	//paths := make([][]*node, 0)
	rate := 0
	cache := make(map[state]int)
	currentState := state{openValves: make([]string, 0), position: "AA", steps: 0, totalRate: 0}
	for {
		if len(stack) == 0 {
			break
		}
		if hours == 30 {

		}
		cur := stack[len(stack)-1]
		stack := stack[:len(stack)-1]
		hours++
		if cur.rate > 0 {
			//Two options: Stay and open valve; Go to neighbor
		} else {
			//Do not open 0 value valves, only go to neighbor

		}
	}
	*/
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
	//partTwo(lines)
}