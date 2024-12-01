package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("d1/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var a [2000]int
	scanner := bufio.NewScanner(file)
	index := 0
	total := 0
	for scanner.Scan() {
		value, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		a[index] = value
		if index != 0 && a[index] > a[index-1] {
			total += 1
		}
		index += 1
	}

	fmt.Println("Total is ", total)
}
