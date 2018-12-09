package main

import ("fmt"; "os"; "bufio"; "strconv")

func main() {
	inputPath := "../chronal-calibration-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	freq := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		freqChange, _ := strconv.Atoi(scanner.Text())
		freq += freqChange
	}
	fmt.Println(freq)
}