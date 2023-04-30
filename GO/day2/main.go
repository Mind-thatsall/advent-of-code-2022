package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	f, err := os.Open("day2.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	sc := bufio.NewScanner(f)

	scorePartOne := 0
	scorePartTwo := 0
	gameOutComePartOne := map[string]int{
		"A X": 4,
		"A Y": 8,
		"A Z": 3,
		"B X": 1,
		"B Y": 5,
		"B Z": 9,
		"C X": 7,
		"C Y": 2,
		"C Z": 6,
	}

	gameOutComePartTwo := map[string]int{
		"A X": 3,
		"A Y": 4,
		"A Z": 8,
		"B X": 1,
		"B Y": 5,
		"B Z": 9,
		"C X": 2,
		"C Y": 6,
		"C Z": 7,
	}

	for sc.Scan() {
		scorePartOne += gameOutComePartOne[string(sc.Text())]
		scorePartTwo += gameOutComePartTwo[string(sc.Text())]
	}
	fmt.Println(scorePartOne)
	fmt.Println(scorePartTwo)

	if err := sc.Err(); err != nil {
		log.Fatal(err)
	}
}
