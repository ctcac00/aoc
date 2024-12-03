package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func getInt(str string) int {
	value, err := strconv.Atoi(str)
	if err != nil {
		log.Fatal(err)
	}
	return value
}

func convertToIntArray(str string) [10]int {
	var result [10]int

	for i := 0; i < len(str); i++ {
		result[i] = getInt(string(str[i]))
	}

	return result
}

func main() {
	file, err := os.Open("d9/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var heightmap [5][10]int
	scanner := bufio.NewScanner(file)
	n := 0
	for scanner.Scan() {
		entry := scanner.Text()
		heightmap[n] = convertToIntArray(entry)
		n++
	}

	fmt.Printf("heightmap is %v\n", heightmap)

	risk_level := 0
	for row := 0; row < 5; row++ {
		for column := 0; column < 10; column++ {
			if heightmap[row][column] < 9 {
				if row == 0 {
					if column == 0 {
						// only check right and down
						if heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row+1][column] {
							risk_level += (1 + heightmap[row][column])
						}

					} else if column == 9 {
						// only check left and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row+1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					} else {
						// check left, right and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row+1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					}
				} else if row == 4 {
					if column == 0 {
						// only check right and up
						if heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row-1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					} else if column == 9 {
						// only check left and up
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row-1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					} else {
						// check left, right and up
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row-1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					}
				} else {
					if column == 0 {
						// only check right, up and down
						if heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row+1][column] &&
							heightmap[row][column] < heightmap[row-1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					} else if column == 9 {
						// only check left, up and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row+1][column] &&
							heightmap[row][column] < heightmap[row-1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					} else {
						// check left, right, up and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row-1][column] &&
							heightmap[row][column] < heightmap[row+1][column] {
							risk_level += (1 + heightmap[row][column])
						}
					}
				}
			}
		}
	}

	fmt.Printf("risk level is %v\n", risk_level)

}
