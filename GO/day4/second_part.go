package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {

	f, err := os.Open("day4.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	sc := bufio.NewScanner(f)

	var nbPairs int

	for sc.Scan() {
		var startFirstPair, endFirstPair, startSecondPair, endSecondPair int

		fmt.Sscanf(sc.Text(), "%d-%d,%d-%d", &startFirstPair, &endFirstPair, &startSecondPair, &endSecondPair)

		if startSecondPair <= endFirstPair && endSecondPair >= startFirstPair || startFirstPair <= endSecondPair && endFirstPair >= startSecondPair {
			nbPairs++
		}
	}

	fmt.Println(nbPairs)

}
