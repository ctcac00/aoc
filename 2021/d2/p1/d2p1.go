package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("d2/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	horizontal := 0
	depth := 0
	for scanner.Scan() {
		entry := scanner.Text()
		position := strings.Fields(entry)[0]
		value, err := strconv.Atoi(strings.Fields(entry)[1])
		if err != nil {
			log.Fatal(err)
		}

		if position == "forward" {
			horizontal = horizontal + value
		} else if position == "up" {
			depth = depth - value
		} else if position == "down" {
			depth = depth + value
		}
	}
	fmt.Printf("Position is %d, %d\n", horizontal, depth)
	fmt.Println("Total is ", horizontal*depth)
}
