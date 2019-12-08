package main

import ("fmt"; "os"; "bufio"; "strings"; "strconv")

type marble struct {
	score int
	next *marble
	prev *marble
}

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
	numMarbles *= 100
	
	curMarble := &marble{score: 0, next: nil, prev: nil}
	curMarble.next = curMarble
	curMarble.prev = curMarble
	//zeroMarble = curMarble

	for i := 1; i <= numMarbles; i++ {
		curPlayer := i % numPlayers
		if i % 23 == 0 {
			players[curPlayer] += i
			for i := 0; i < 7; i++ { curMarble = curMarble.prev }
			players[curPlayer] += curMarble.score
			curMarble.prev.next = curMarble.next
			curMarble.next.prev = curMarble.prev
			curMarble = curMarble.next
			continue
		}
		curMarble = curMarble.next
		nextMarble := &marble{score: i, next: curMarble.next, prev: curMarble}
		curMarble.next.prev = nextMarble
		curMarble.next = nextMarble
		curMarble = curMarble.next
	}

	maxScore := 0
	for _, v := range players {
		if v > maxScore { maxScore = v }
	}
	fmt.Println(maxScore)
}