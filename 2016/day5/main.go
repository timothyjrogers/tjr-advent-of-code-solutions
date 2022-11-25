package main

import (
	"fmt"
	"crypto/md5"
	"strconv"
	"strings"
)

const input string = "ffykfhsq"

func partOne() {
	password := ""
	for index := 0; len(password) < 8; index++ {
		cur := []byte(input + strconv.Itoa(index))
		hashBytes := md5.Sum(cur)
		hash := fmt.Sprintf("%x", string(hashBytes[:]))
		if hash[0:5] == "00000" {
			password += string(hash[5])
		}
	}
	fmt.Printf("Part 1: %v\n", password)
}

func partTwo() {
	password := [8]string{"","","","","","","",""}
	found := 0
	for index := 0; found < 8; index++ {
		cur := []byte(input + strconv.Itoa(index))
		hashBytes := md5.Sum(cur)
		hash := fmt.Sprintf("%x", string(hashBytes[:]))
		if hash[0:5] == "00000" {
			idx, _ := strconv.ParseInt(string(hash[5]), 16, 16)
			if idx > 7 {
				continue
			}
			if password[idx] == "" {
				password[idx] = string(hash[6])
				found++
			}
		}
	}
	fmt.Printf("Part 2: %v\n", strings.Join(password[:], ""))
}

func main() {
    partOne()
	partTwo()
}