package main

import ("fmt"; "os"; "bufio"; "strings"; "sort")

type Node struct {
	name string
	dependsOn []string
	dependedOn []string
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
			nodeA := &Node{name: instructionA, dependsOn: depends, dependedOn: depended}
			nodes[instructionA] = nodeA
		}
		if _, ok := nodes[instructionB]; ok {
			n := nodes[instructionB]
			n.AddDependsOn(instructionA)
		} else {
			var depended []string
			var depends []string
			depends = append(depends, instructionA)
			nodeB := &Node{name: instructionB, dependsOn: depends, dependedOn: depended}
			nodes[instructionB] = nodeB
		}
	}
	
	var searchStack []string
	outString := ""

	for _, v := range nodes {
		if len(v.dependsOn) == 0 { searchStack = append(searchStack, v.name) }
	}
	sort.Sort(sort.Reverse(sort.StringSlice(searchStack)))

	for len(searchStack) > 0 {
		searchNode := nodes[searchStack[len(searchStack) - 1]]
		searchStack = searchStack[:len(searchStack) - 1]
		if len(searchNode.dependsOn) == 0 {
			outString += searchNode.name
			nextNodes := searchNode.dependedOn
			sort.Sort(sort.Reverse(sort.StringSlice(nextNodes)))
			for _, v := range nextNodes {
				nodes[v].RemoveDependsOn(searchNode.name) 
				if len(nodes[v].dependsOn) == 0 { searchStack = append(searchStack, nodes[v].name) }
			}
			sort.Sort(sort.Reverse(sort.StringSlice(searchStack)))
		}
	}

	fmt.Println(outString)

}