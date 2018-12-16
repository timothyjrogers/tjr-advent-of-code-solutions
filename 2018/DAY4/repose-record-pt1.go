package main

import ("fmt"; "os"; "bufio"; "sort"; "strings"; "strconv")

func main() {
	inputPath := "repose-record-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	var records []string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		records = append(records, scanner.Text())
	}
	sort.Strings(records)

	guardMap := make(map[string]map[int]int)
	curGuard := ""
	curStart := 0
	curStop := 0

	for _, record := range records {
		rec := strings.Split(record, " ")
		if rec[2] == "Guard" {
			curGuard = rec[3]
			if _, ok := guardMap[curGuard]; !ok {
				guardMap[curGuard] = make(map[int]int)
			}
			curStart, curStop = 0, 0
		} else if rec[2] == "falls" {
			minutes := strings.Split(rec[1], ":")[1]
			minutes = minutes[:len(minutes)-1]
			minInt, _ := strconv.Atoi(minutes)
			curStart = minInt
		} else {
			minutes := strings.Split(rec[1], ":")[1]
			minutes = minutes[:len(minutes)-1]
			minInt, _ := strconv.Atoi(minutes)
			curStop = minInt
			for i := curStart; i < curStop; i++ {
				if _, ok := guardMap[curGuard][i]; ok {
					guardMap[curGuard][i] += 1
				} else {
					guardMap[curGuard][i] = 1
				}
			}
		}
	}

	maxGuard := ""
	maxGuardMins := 0
	for k, v := range guardMap {
		guardMins := 0
		for _, val := range v {
			guardMins = guardMins + val
		}
		if guardMins > maxGuardMins {
			maxGuardMins = guardMins
			maxGuard = k
		}
	}

	maxMin := 0
	maxMinKey := 0
	for k, v := range guardMap[maxGuard] {
		if v > maxMin {
			maxMin = v
			maxMinKey = k
		}
	}
	maxGuardInt, _ := strconv.Atoi(maxGuard[1:])
	fmt.Println(maxGuardInt*maxMinKey)

}