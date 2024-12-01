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
	aim := 0
	for scanner.Scan() {
		entry := scanner.Text()
		position := strings.Fields(entry)[0]
		value, err := strconv.Atoi(strings.Fields(entry)[1])
		if err != nil {
			log.Fatal(err)
		}

		if position == "forward" {
			depth = depth + (aim * value)
			horizontal = horizontal + value
		} else if position == "up" {

			aim = aim - value
		} else if position == "down" {

			aim = aim + value
		}
	}
	fmt.Printf("horizontal is %d, depth is %d, aim is %d \n", horizontal, depth, aim)
	fmt.Println("Total is ", horizontal*depth)
}
