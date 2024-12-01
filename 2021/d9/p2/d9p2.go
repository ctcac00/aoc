package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func getInt(str string) int {
	value, err := strconv.Atoi(str)
	if err != nil {
		log.Fatal(err)
	}
	return value
}

func convertToIntArray(str string) [100]int {
	var result [100]int

	for i := 0; i < len(str); i++ {
		result[i] = getInt(string(str[i]))
	}

	return result
}

func printBasin(heightmap [100][100]int) {
	fmt.Printf("Basin\n")
	for i := 0; i < 100; i++ {
		fmt.Printf("%v\n", heightmap[i])
	}
}

func calculateBasin(heightmap [100][100]int) int {
	sum := 0
	for i := 0; i < 100; i++ {
		for j := 0; j < 100; j++ {
			if heightmap[i][j] == -1 {
				sum++
			}
		}
	}
	return sum
}

func getBasin(heightmap [100][100]int, x int, y int) [100][100]int {
	//check left
	heightmap[x][y] = -1
	if y > 0 && heightmap[x][y-1] < 9 && heightmap[x][y-1] != -1 {
		heightmap = getBasin(heightmap, x, y-1)
	}
	//check right
	if y < 98 && heightmap[x][y+1] < 9 && heightmap[x][y+1] != -1 {
		heightmap = getBasin(heightmap, x, y+1)
	}
	//check up
	if x > 0 && heightmap[x-1][y] < 9 && heightmap[x-1][y] != -1 {
		heightmap = getBasin(heightmap, x-1, y)
	}
	//check down
	if x < 98 && heightmap[x+1][y] < 9 && heightmap[x+1][y] != -1 {
		heightmap = getBasin(heightmap, x+1, y)
	}

	return heightmap
}

func main() {
	file, err := os.Open("d9/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var heightmap [100][100]int
	scanner := bufio.NewScanner(file)
	n := 0
	for scanner.Scan() {
		entry := scanner.Text()
		heightmap[n] = convertToIntArray(entry)
		n++
	}

	fmt.Printf("heightmap is %v\n", heightmap)

	var basin_sizes []int
	for row := 0; row < 100; row++ {
		for column := 0; column < 100; column++ {
			if heightmap[row][column] < 9 {
				if row == 0 {
					if column == 0 {
						// only check right and down
						if heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row+1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}

					} else if column == 99 {
						// only check left and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row+1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					} else {
						// check left, right and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row+1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					}
				} else if row == 99 {
					if column == 0 {
						// only check right and up
						if heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row-1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					} else if column == 99 {
						// only check left and up
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row-1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					} else {
						// check left, right and up
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row-1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					}
				} else {
					if column == 0 {
						// only check right, up and down
						if heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row+1][column] &&
							heightmap[row][column] < heightmap[row-1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					} else if column == 99 {
						// only check left, up and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row+1][column] &&
							heightmap[row][column] < heightmap[row-1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					} else {
						// check left, right, up and down
						if heightmap[row][column] < heightmap[row][column-1] &&
							heightmap[row][column] < heightmap[row][column+1] &&
							heightmap[row][column] < heightmap[row-1][column] &&
							heightmap[row][column] < heightmap[row+1][column] {
							//check basin size
							printBasin(getBasin(heightmap, row, column))
							basin_sizes = append(basin_sizes, calculateBasin(getBasin(heightmap, row, column)))
						}
					}
				}
			}
		}
	}

	sort.Sort(sort.Reverse(sort.IntSlice(basin_sizes)))
	fmt.Printf("basin_sizes is %v\n", basin_sizes)
	total := 1
	for i := 0; i < 3; i++ {
		total *= basin_sizes[i]
	}

	fmt.Printf("Result is %v\n", total)
}
