package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func create_sliding_window(input [2000]int) []int {
	var result []int

	for i := 0; i < len(input)-2; i++ {
		result = append(result, input[i]+input[i+1]+input[i+2])
	}
	return result
}

func main() {
	file, err := os.Open("d1/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var input [2000]int
	scanner := bufio.NewScanner(file)
	i := 0
	for scanner.Scan() {
		value, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		input[i] = value
		i++
	}

	result := create_sliding_window(input)

	total := 0
	for i := 1; i < len(result); i++ {
		if result[i] > result[i-1] {
			total++
		}
	}
	fmt.Println("Total is ", total)
}
