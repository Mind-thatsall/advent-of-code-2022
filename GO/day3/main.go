package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {

	f, err := os.Open("day3.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	sc := bufio.NewScanner(f)

	for sc.Scan() {
		lineFirstHalf := string(sc.Text())
		splittedLineFirstHalf := strings.Split(lineFirstHalf[:len(lineFirstHalf)/2], "")
		fmt.Println(splittedLineFirstHalf)
	}

}
