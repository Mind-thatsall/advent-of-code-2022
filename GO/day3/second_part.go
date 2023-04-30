package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
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

		splittedFirstLine := createSet(sc.Text())
		sc.Scan()
		splittedSecondLine := createSet(sc.Text())
		sc.Scan()
		splittedThirdLine := createSet(sc.Text())

		for k := range splittedFirstLine {
			if splittedSecondLine[k] && splittedThirdLine[k] {
				scoreLetter := unicode.ToLower(k) - 96
				if unicode.IsUpper(k) {
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

func createSet(items string) (itemsMap map[rune]bool) {
	itemsMap = make(map[rune]bool)

	for _, item := range items {
		itemsMap[item] = true
	}

	return
}
