package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func getInt(str string) int {
	value, err := strconv.Atoi(str)
	if err != nil {
		log.Fatal(err)
	}
	return value
}

func newDay(state []int) []int {
	for i := 0; i < len(state); i++ {
		if state[i] == 0 {
			state[i] = 6
			state = append(state, 9)
		} else {
			state[i]--
		}
	}
	return state
}

func main() {

	number_of_days := 80

	file, err := os.Open("d6/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var temp []string
	var state []int
	var line string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line = scanner.Text()
	}

	temp = strings.Split(line, ",")
	for i := 0; i < len(temp); i++ {
		state = append(state, getInt(temp[i]))
	}

	fmt.Printf("Initial state: %v\n", state)

	for i := 0; i < number_of_days; i++ {
		state = newDay(state)
		//fmt.Printf("After %v days: %v\n", i+1, state)
	}

	fmt.Printf("Number of fish is %v\n", len(state))
}
