package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type stack struct {
	elements []rune
}

func (s *stack) addToBottomOfStack(r rune) {
	s.elements = append([]rune{r}, s.elements...)
}

func (s *stack) pop(n int) (r []rune) {
	r = s.elements[len(s.elements)-n : len(s.elements)]
	s.elements = s.elements[:len(s.elements)-n]
	return
}

func (s *stack) push(r []rune) {
	s.elements = append(s.elements, r...)
}

func main() {
	f, err := os.Open("day5.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	sc := bufio.NewScanner(f)

	stacks := make([]stack, 9)

	sc.Scan()
	for sc.Text() != " 1   2   3   4   5   6   7   8   9 " {
		for i, r := range sc.Text() {
			if r != ' ' && r != '[' && r != ']' {
				stacks[i/4].addToBottomOfStack(r)
			}
		}

		sc.Scan()
	}

	sc.Scan()

	for sc.Scan() {
		var toMove, from, to int

		fmt.Sscanf(sc.Text(), "move %d from %d to %d", &toMove, &from, &to)

		stacks[to-1].push(stacks[from-1].pop(toMove))
	}

	for _, s := range stacks {
		fmt.Print(string(s.pop(1)))
	}
}
