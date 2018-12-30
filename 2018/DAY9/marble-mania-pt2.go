package main

import ("fmt"; "os"; "bufio"; "strings"; "strconv")

type Marble struct {
	score int
	next *Marble
	prev *Marble
}
func (m *Marble) getScore() int {
	return m.score
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
	numMarbles = numMarbles

	curMarble := &Marble{score: 0, next: nil, prev: nil}
	curMarble.next = curMarble
	curMarble.prev = curMarble

	for i := 1; i <= numMarbles; i++ {
		curPlayer := i % numPlayers
		if i % 23 == 0 {
			players[curPlayer] += i
			for i := 0; i < 6; i++ { curMarble = curMarble.prev }
			fmt.Println(curMarble.getScore())
			players[curPlayer] += curMarble.getScore()
			curMarble.prev.next = curMarble.next
			curMarble.next.prev = curMarble.prev
			curMarble = curMarble.next
			continue
		}
		newMarble := &Marble{score: i, next: nil, prev: nil}
		curMarble = curMarble.next
		tempMarble := curMarble.next
		curMarble.next = newMarble
		newMarble.prev = curMarble
		newMarble.next = tempMarble
		curMarble = curMarble.next
	}

	maxScore := 0
	for _, v := range players {
		if v > maxScore { maxScore = v }
	}
	fmt.Println(maxScore)
}