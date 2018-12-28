package main

import ("fmt"; "os"; "bufio"; "strings"; "sort")

type Node struct {
	name string
	dependsOn []string
	dependedOn []string
	time int
}
func (n *Node) AddDependsOn(s string) {
	n.dependsOn = append(n.dependsOn, s)
}
func (n *Node) RemoveDependsOn(s string) {
	idx := -1
	for i, v := range n.dependsOn {
		if v == s { idx = i }
	}
	n.dependsOn = append(n.dependsOn[:idx], n.dependsOn[idx+1:]...)
}
func (n *Node) AddDependedOn(s string) {
	n.dependedOn = append(n.dependedOn, s)
}
func(n *Node) reverseSortDependsOn() {
	sort.Sort(sort.Reverse(sort.StringSlice(n.dependsOn)))
}
func(n *Node) decrementTime() {
	n.time -= 1
}

func letterToNodeTime(l string) int {
	base := 60
	letters := []string{"A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"}
	for i, v := range letters {
		if v == l { return base + i + 1 }
	}
	return 0
}

func main() {
	inputPath := "sum-of-parts-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	nodes := make(map[string]*Node)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		instruction := scanner.Text()
		instructionA, instructionB := strings.Split(instruction, " ")[1], strings.Split(instruction, " ")[7]
		if _, ok := nodes[instructionA]; ok {
			n := nodes[instructionA]
			n.AddDependedOn(instructionB)
		} else {
			var depended []string
			depended = append(depended, instructionB)
			var depends []string
			nodeA := &Node{name: instructionA, dependsOn: depends, dependedOn: depended, time: letterToNodeTime(instructionA)}
			nodes[instructionA] = nodeA
		}
		if _, ok := nodes[instructionB]; ok {
			n := nodes[instructionB]
			n.AddDependsOn(instructionA)
		} else {
			var depended []string
			var depends []string
			depends = append(depends, instructionA)
			nodeB := &Node{name: instructionB, dependsOn: depends, dependedOn: depended, time: letterToNodeTime(instructionB)}
			nodes[instructionB] = nodeB
		}
	}
	
	workers := 5
	var activeNodes []string
	var availNodes []string
	outSeconds := 0

	for _, v := range nodes {
		if len(v.dependsOn) == 0 { availNodes = append(availNodes, v.name) }
	}
	sort.Sort(sort.Reverse(sort.StringSlice(availNodes)))

	for len(activeNodes) > 0 || len(availNodes) > 0 {
		if len(activeNodes) < workers {
			for len(activeNodes) < workers && len(availNodes) > 0 {
				node := nodes[availNodes[len(availNodes) - 1]]
				availNodes = availNodes[:len(availNodes) - 1]
				activeNodes = append(activeNodes, node.name)
			}
		}
		for i, v := range activeNodes {
			node := nodes[v]
			node.decrementTime()
			if node.time == 0 { 
				if len(activeNodes) > 1 { activeNodes = append(activeNodes[:i], activeNodes[i+1:]...) } else { activeNodes = activeNodes[:0] }
				nextNodes := node.dependedOn
				for _, val := range nextNodes {
					nodes[val].RemoveDependsOn(node.name) 
					if len(nodes[val].dependsOn) == 0 { availNodes = append(availNodes, nodes[val].name) }
				}
				sort.Sort(sort.Reverse(sort.StringSlice(availNodes)))
			}
		}
		outSeconds += 1
	}

	fmt.Println(outSeconds)
}