package main

import ("fmt"; "os"; "bufio"; "strings"; "strconv")

func main() {
	//initialize file access for input
	inputPath := "marble-mania-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	data := strings.Fields(scanner.Text())

	numPlayers, _ := strconv.Atoi(data[0])
	players := make(map[int]int)
	for i := 1; i <= numPlayers; i++ { players[i] = 0 }
	
	numMarbles, _ := strconv.Atoi(data[6])
	marbles := make([]int, 1)
	marbles[0] = 0

	curMarble := 0
	for i := 1; i <= numMarbles; i++ {
		curPlayer := i % numPlayers
		if i % 23 == 0 {
			players[curPlayer] += i
			nextMarble := curMarble - 7
			if nextMarble < 0 { nextMarble += len(marbles) }
			players[curPlayer] += marbles[nextMarble]
			if nextMarble == 0 { 
				marbles = marbles[1:]
			} else if nextMarble == len(marbles) - 1 {
				marbles = marbles[:len(marbles) - 1]
			} else {
				marbles = append(marbles[:nextMarble], marbles[nextMarble+1:]...)
			}
			//nextMarble += 1
			//if nextMarble >= len(marbles) { nextMarble = nextMarble - len(marbles) }
			curMarble = nextMarble
			continue
		}
		newMarbles := make([]int, len(marbles)+1)
		nextMarble := curMarble + 2
		if nextMarble > len(marbles) { nextMarble = nextMarble - len(marbles) }
		if nextMarble == 0 { nextMarble = len(marbles) }
		if nextMarble == len(marbles) {
			newMarbles = append(marbles, i)
		} else {
			newMarbles = make([]int, len(marbles)+1)
			copy(newMarbles[:nextMarble], marbles[:nextMarble])
			newMarbles[nextMarble] = i
			copy(newMarbles[nextMarble+1:], marbles[nextMarble:])
		}
		marbles = newMarbles
		curMarble = nextMarble
	}

	maxScore := 0
	for _, v := range players {
		if v > maxScore { maxScore = v }
	}
	fmt.Println(maxScore)
}