package main

import ("fmt"; "os"; "bufio"; "strings"; "strconv")

type Node struct {
	children []*Node
	meta []int
}
func (n *Node) appendChild(child *Node) {
	n.children = append(n.children, child)
}
func (n *Node) appendMeta(meta int) {
	n.meta = append(n.meta, meta)
}

func populateTree(data []string) (*Node, int) {
	var children []*Node
	var meta []int
	root := &Node{children: children, meta: meta}

	numChildren, _ := strconv.Atoi(data[0])
	numMeta, _ := strconv.Atoi(data[1])
	entriesUsed := 2
	for i := 0; i < numChildren; i++ {
		child, childEntries := populateTree(data[entriesUsed:])
		root.appendChild(child)
		entriesUsed += childEntries
	}

	for i := 0; i < numMeta; i++ { 
		metadata, _ := strconv.Atoi(data[entriesUsed + i])
		root.appendMeta(metadata)
	}
	entriesUsed += numMeta
	return root, entriesUsed
}

func nodeValue(root *Node) int {
	value := 0
	if len(root.children) == 0 {
		for _, v := range root.meta { value += v }
		return value
	}
	for _, v := range root.meta {
		if v <= len(root.children) && v > 0 { value += nodeValue(root.children[v - 1]) }
	}
	return value
}

func main() {
	//initialize file access for input
	inputPath := "memory-maneuver-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	//read file data into data object
	var data []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		license := scanner.Text()
		data = strings.Fields(license)
	}

	var root *Node
	root, dataConsumed := populateTree(data)
	//Sanity check for tree construction
	fmt.Println(dataConsumed, "/", len(data))
	fmt.Println(nodeValue(root))
}