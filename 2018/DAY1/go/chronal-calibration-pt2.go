package main

import ("fmt"; "os"; "bufio"; "strconv")

func main() {
	inputPath := "../chronal-calibration-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	freq := int64(0)
	freqSeen := make(map[int64]bool)
	done := false

	var changes []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		changes = append(changes, scanner.Text())
	}
	for !done {	
		for _, val := range changes {
			freqChange, _ := strconv.ParseInt(val, 0, 0)
			freq = freq + freqChange
			if _, ok := freqSeen[freq]; ok {
				done = true
				break
			} else {
				freqSeen[freq] = true
			}
		}
	}
	fmt.Println(freq)
}