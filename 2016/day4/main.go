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

type room struct {
	name string
	sector int
	checksum string
}

func newRoom(encoded string) *room {
	parts := strings.Split(encoded, "-")
	name := strings.Join(parts[0:len(parts)-1], " ")
	sector, _ := strconv.Atoi(strings.Split(parts[len(parts)-1], "[")[0])
	checksum := strings.Split(parts[len(parts)-1], "[")[1]
	checksum = checksum[0:len(checksum)-1]
	room := room{name: name, sector: sector, checksum: checksum}
	return &room
}

func calculateRoomChecksum(r *room) string {
	counter := make(map[string]int)
	for _, c := range r.name {
		if string(c) != " " {
			counter[string(c)]++
		}
	}
	letters := make([]string, 0, len(counter))
	for c := range counter {
		letters = append(letters, c)
	}
	sort.Slice(letters, func(i, j int) bool {
		if counter[letters[i]] > counter[letters[j]] {
			return true
		} else if counter[letters[i]] == counter[letters[j]] {
			return strings.Compare(letters[i], letters[j]) < 0
		} else {
			return false
		}
	})
	return strings.Join(letters[0:5], "")
}

func isValidRoom(r *room) bool {
	return calculateRoomChecksum(r) == r.checksum
}

func partOne(lines []string) []*room {
	sum := 0
	valid := make([]*room, 0)
	for i := 0; i < len(lines); i++ {
		r := newRoom(lines[i])
		if isValidRoom(r) {
			sum += r.sector
			valid = append(valid, r)
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
	return valid
}

func decodeCharacter(in string, count int) string {
	letters := [26]string{"a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"}
	index := 0
	for i, c := range letters {
		if c == in {
			index = i
			break
		}
	}
	return letters[(index + count) % 26]
}

func decodeRoomName(r *room) string {
	name := ""
	for _, c := range r.name {
		if string(c) == " " {
			name += " "
		} else {
			name += decodeCharacter(string(c), r.sector)
		}
	}
	return name
}

func partTwo(rooms []*room) {
	for _, r := range rooms {
		name := decodeRoomName(r)
		//Magic string determined by sorting and printing full list of decoded rooms; unclear from prompt
		if name == "northpole object storage" {
			fmt.Printf("Part 2: %v\n", r.sector)
		}
	}
}

func main() {
    file, _ := os.Open(input_file)
    defer file.Close()

    scanner := bufio.NewScanner(file)
	var lines []string
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

	valid_rooms := partOne(lines)
	partTwo(valid_rooms)
}