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

func addFishes(fishes [9]int) int {
	result := 0
	for i := 0; i < len(fishes); i++ {
		result += fishes[i]
	}
	return result
}

func main() {

	number_of_days := 256

	file, err := os.Open("d6/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var temp []string
	var line string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line = scanner.Text()
	}

	var fishes [9]int
	temp = strings.Split(line, ",")
	for i := 0; i < len(temp); i++ {
		fish_timer := getInt(temp[i])
		fishes[fish_timer]++
	}

	for i := 0; i < number_of_days; i++ {

		fishes_in_spawn := fishes[0]

		fishes[0] = fishes[1]
		fishes[1] = fishes[2]
		fishes[2] = fishes[3]
		fishes[3] = fishes[4]
		fishes[4] = fishes[5]
		fishes[5] = fishes[6]
		fishes[6] = fishes[7] + fishes_in_spawn
		fishes[7] = fishes[8]
		fishes[8] = fishes_in_spawn
	}

	fmt.Printf("Result is %v\n", addFishes(fishes))

}
