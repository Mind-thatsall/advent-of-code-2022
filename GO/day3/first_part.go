package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

func main() {

	f, err := os.Open("day3.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	sc := bufio.NewScanner(f)
	var score rune

	for sc.Scan() {
		line := string(sc.Text())
		splittedLineFirstHalf := strings.Split(line[:len(line)/2], "")
		secondHalf := line[len(line)/2:]

		for _, v := range splittedLineFirstHalf {
			foundLetter := strings.Contains(secondHalf, v)
			if foundLetter {
				scoreLetter := unicode.ToLower([]rune(v)[0]) - 96
				if unicode.IsUpper([]rune(v)[0]) {
					score += scoreLetter + 26
				} else {
					score += scoreLetter
				}
				break
			}

		}

	}

	fmt.Println(score)
}
