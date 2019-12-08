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

func main() {
	inputPath := "slice-it-pt1.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	var claims []string
	claimedSquares := make(map[string]int)
	ans := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		claims = append(claims, scanner.Text())
	}
	for _, claim := range claims {
		claimIdxs := parseClaimIdxs(claim)
		for _, idx := range claimIdxs {
			if _, ok := claimedSquares[idx]; ok {
				claimedSquares[idx] = claimedSquares[idx] + 1
			} else {
				claimedSquares[idx] = 1
			}
		}
	}

	for _, v := range claimedSquares {
		if v > 1 {
			ans = ans + 1
		}
	}
	fmt.Println(ans)
}