package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("d8/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var outputs [][]string
	scanner := bufio.NewScanner(file)
	n := 0
	for scanner.Scan() {
		entry := scanner.Text()
		output := strings.Split(strings.Split(entry, "|")[1], " ")
		outputs = append(outputs, output)

		fmt.Printf("Entry is: %v\n", entry)
		fmt.Printf("Output is: %v\n", output)
		n++
	}

	total := 0
	for i := 0; i < n; i++ {
		for j := 0; j < len(outputs[i]); j++ {
			fmt.Printf("Output is: %v\n", outputs[i][j])
			if len(outputs[i][j]) == 2 {
				//1
				total++
			}
			if len(outputs[i][j]) == 4 {
				//4
				total++
			}
			if len(outputs[i][j]) == 3 {
				//7
				total++
			}
			if len(outputs[i][j]) == 7 {
				//8
				total++
			}
		}
	}

	fmt.Printf("Total is: %v\n", total)

}
