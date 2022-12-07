package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
)

const input_file string = "input.txt"

type file struct {
	name string
	size int
}

func newFile(name string, size int) *file {
	f := file{name: name, size: size}
	return &f
}

type directory struct {
	name string
	size int
	files map[string]*file
	directories map[string]*directory
}

func newDirectory(name string) *directory {
	d := directory{name: name, files: make(map[string]*file, 0), size: -1, directories: make(map[string]*directory, 0)}
	return &d
}

func addDirectoryToDirectory(dir *directory, d *directory) {
	dir.directories[d.name] = d
}

func addFileToDirectory(dir *directory, f *file) {
	dir.files[f.name] = f
}

func printPath(path []*directory) {
	pstring := ""
	for _, p := range path {
		pstring += " " + p.name
	}
	fmt.Println(pstring)
}

func getDirectorySizeP1(dir *directory, sum *int, sizes *[]int) {
	size := 0
	for _, v := range dir.files {
		size += v.size
	}
	if len(dir.directories) == 0 {
		dir.size = size
		*sizes = append(*sizes, size)
		if size <= 100000 {
			*sum += size
		}
		return
	}
	for _, v := range dir.directories {
		if v.size < 0 {
			getDirectorySizeP1(v, sum, sizes)
		}
		size += v.size
	}
	dir.size = size
	*sizes = append(*sizes, size)
	if size <= 100000 {
		*sum += size
	}
}

func partOne(lines []string, sizes *[]int) *directory {
	root := newDirectory("/")
	path := []*directory{root}
	for _, line := range lines {
		if line[0] == '$' {
			cmd := line[2:4]
			if cmd == "cd" {
				dname := line[5:]
				if dname == "/" {
					path = []*directory{root}
				} else if dname == ".." {
					path = path[:len(path)-1]
				} else {
					dir := path[len(path)-1].directories[dname]
					path = append(path, dir)
				}
			} else if cmd == "ls" {
				continue
			}
		} else {
			entry := strings.Split(line, " ")
			if entry[0] == "dir" {
				dir := newDirectory(entry[1])
				addDirectoryToDirectory(path[len(path)-1], dir)
			} else {
				size, _ := strconv.Atoi(entry[0])
				file := newFile(entry[1], size)
				addFileToDirectory(path[len(path)-1], file)
			} 
		}
	}
	sum := new(int)
	getDirectorySizeP1(root, sum, sizes)
	fmt.Printf("Part 2: %v\n", *sum)
	return root
}

func partTwo(root *directory, sizes []int) {
	availableSpace := 70000000 - root.size
	requiredSpace := 30000000 - availableSpace
	min := 70000000
	for _, size := range sizes {
		if size >= requiredSpace && size < min {
			min = size
		}
	}
	fmt.Printf("Part 2: %v", min)
}

func main() {
    file, _ := os.Open(input_file)
    defer file.Close()

    scanner := bufio.NewScanner(file)
	var lines []string
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

	sizes := make([]int, 0)
	root := partOne(lines, &sizes)
	partTwo(root, sizes)
}