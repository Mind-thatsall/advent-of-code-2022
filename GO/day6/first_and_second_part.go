package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"time"
)

func main() {
	start := time.Now()
	data, err := ioutil.ReadFile("day6")
	if err != nil {
		log.Fatal(err)
	}

	k := 14
	// k := 4
	window := make([]int, 256)
	distinctCount := 0
	lastDistinctIndex := -1
	for i := 0; i < len(data); i++ {
		c := data[i]
		if window[c] == 0 {
			distinctCount++
		}

		window[c]++
		if i >= k-1 {
			if distinctCount == k {
				lastDistinctIndex = i
				break // once we found the last distinct index we can break out of the loop
			}
			window[data[i-k+1]]--
			if window[data[i-k+1]] == 0 {
				distinctCount--
			}
		}
	}

	if lastDistinctIndex == k-1 {
		fmt.Println("No substring found")
	} else {
		fmt.Println(lastDistinctIndex + 1)
	}

	elapsed := time.Since(start)
	fmt.Println(elapsed)
}
