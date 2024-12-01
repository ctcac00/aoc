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

func main() {
	file, err := os.Open("d5/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var diagram [1000][1000]int
	//fmt.Printf("diagram is %v\n", diagram)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		//fmt.Printf("Entry is %v\n", line)

		coordinates := strings.Split(line, " -> ")
		//fmt.Printf("Coordinates are %v\n", coordinates)

		x1 := getInt(strings.Split(coordinates[0], ",")[0])
		y1 := getInt(strings.Split(coordinates[0], ",")[1])

		x2 := getInt(strings.Split(coordinates[1], ",")[0])
		y2 := getInt(strings.Split(coordinates[1], ",")[1])

		fmt.Printf("x1 is %v, y1 is %v, x2 is %v, y2 is %v\n", x1, y1, x2, y2)

		if x1 == x2 {
			//fmt.Printf("Horizontal line!\n")

			if y2 >= y1 {
				for i := y1; i <= y2; i++ {
					diagram[i][x1] += 1
				}
			}
			if y2 < y1 {
				for i := y2; i <= y1; i++ {
					diagram[i][x1] += 1
				}
			}

		} else if y1 == y2 {
			//fmt.Printf("Vertical line!\n")

			if x2 >= x1 {
				for i := x1; i <= x2; i++ {
					diagram[y1][i] += 1
				}
			}

			if x2 < x1 {
				for i := x2; i <= x1; i++ {
					diagram[y1][i] += 1
				}
			}

		}

	}

	total := 0
	for i := 0; i < 1000; i++ {
		//fmt.Printf("%v\n", diagram[i])
		for j := 0; j < 1000; j++ {
			if diagram[i][j] >= 2 {
				total++
			}
		}
	}

	fmt.Printf("Total is %v\n", total)

}
