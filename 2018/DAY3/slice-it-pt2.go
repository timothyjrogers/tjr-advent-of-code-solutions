package main

import ("fmt"; "os"; "bufio"; "strings"; "strconv")

//#1 @ 871,327: 16x20
func parseClaimIdxs(claim string) []string {
	var claimedIdxs []string

	claimSplit := strings.Split(claim, " ")
	origin, dim := claimSplit[2], claimSplit[3]

	originx, originy := strings.Split(origin, ",")[0], strings.Split(origin, ",")[1]
	originy = originy[:len(originy)-1]
	dimx, dimy := strings.Split(dim, "x")[0], strings.Split(dim, "x")[1]

	origin_x, _ := strconv.ParseInt(originx, 0, 0)
	origin_y, _ := strconv.ParseInt(originy, 0, 0)

	dim_x, _ := strconv.ParseInt(dimx, 0, 0)
	dim_y, _ := strconv.ParseInt(dimy, 0, 0)

	for i := origin_x; i < origin_x + dim_x; i++ {
		for j := origin_y; j < origin_y + dim_y; j++ {
			claimx := strconv.Itoa(int(i))
			claimy := strconv.Itoa(int(j))
			claimedIdxs = append(claimedIdxs, claimx+","+claimy)
		}
	}
	return claimedIdxs
}

func compareClaims(claim1 string, claim2 string) bool {
	overlap := make(map[string]int)
	claim1Idxs := parseClaimIdxs(claim1)
	claim2Idxs := parseClaimIdxs(claim2)
	for _, claim := range claim1Idxs {
		if _, ok := overlap[claim]; ok { overlap[claim] = overlap[claim] + 1 } else { overlap[claim] = 1 }
	}
	for _, claim := range claim2Idxs {
		if _, ok := overlap[claim]; ok { overlap[claim] = overlap[claim] + 1 } else { overlap[claim] = 1 }
	}
	for _, v := range overlap {
		if v > 1 {return true}
	}
	return false
}

func main() {
	inputPath := "slice-it-pt1.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	var claims []string
	var ans string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		claims = append(claims, scanner.Text())
	}
	for i, claim := range claims {
		overlap := false
		for j, c := range claims {
			if i == j {continue}
			overlap = compareClaims(claim, c)
			if overlap == true {break}
		}
		if overlap == false { ans = claim; break }
	}
	fmt.Println(ans)
}