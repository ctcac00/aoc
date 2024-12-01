package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
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

func getMax(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func getMin(a int, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	file, err := os.Open("d5/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var diagram [1000][1000]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		coordinates := strings.Split(line, " -> ")

		x1 := getInt(strings.Split(coordinates[0], ",")[0])
		y1 := getInt(strings.Split(coordinates[0], ",")[1])

		x2 := getInt(strings.Split(coordinates[1], ",")[0])
		y2 := getInt(strings.Split(coordinates[1], ",")[1])

		//fmt.Printf("%v,%v, -> %v,%v\n", x1, y1, x2, y2)

		if x1 == x2 {
			//fmt.Printf("Vertical line\n")
			for y := getMin(y1, y2); y <= getMax(y1, y2); y++ {
				diagram[y][x1] += 1
			}

		} else if y1 == y2 {
			//fmt.Printf("Horizontal line\n")
			for x := getMin(x1, x2); x <= getMax(x1, x2); x++ {
				diagram[y1][x] += 1
			}
		} else if math.Abs(float64(x1-x2)) == math.Abs(float64(y1-y2)) {
			//fmt.Printf("Diagonal line\n")
			if x2 >= x1 {
				y := y1
				for x := x1; x <= x2; x++ {
					diagram[y][x] += 1
					if y1 <= y2 {
						y++
					} else {
						y--
					}
				}
			} else {
				y := y1
				for x := x1; x >= x2; x-- {
					diagram[y][x] += 1
					if y1 <= y2 {
						y++
					} else {
						y--
					}
				}
			}
		}
	}

	total := 0
	for x := 0; x < 1000; x++ {
		//fmt.Printf("%v\n", diagram[x])
		for y := 0; y < 1000; y++ {
			if diagram[y][x] >= 2 {
				total++
			}
		}
	}

	fmt.Printf("Total is %v\n", total)

}
