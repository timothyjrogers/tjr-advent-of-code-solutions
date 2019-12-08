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
		//call populateTree for each child node
		child, childEntries := populateTree(data[entriesUsed:])
		//increment entriesUsed with each result
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

func sumMetaData(root *Node) int {
	metaSum := 0
	for _, v := range root.meta { metaSum += v }
	if len(root.children) == 0 { return metaSum }
	for _, v := range root.children { metaSum += sumMetaData(v) }
	return metaSum
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
	root, _ := populateTree(data)
	fmt.Println(sumMetaData(root))
}