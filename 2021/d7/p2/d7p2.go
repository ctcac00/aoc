package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
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

func calculateMedian(data []int) int {
	sort.Ints(data)

	length := len(data)
	var median int
	if length%2 == 0 {
		// Even number"
		median = data[(length-1)/2]
	} else {
		// Odd number"
		median = (data[length/2] + data[(length/2)-1]) / 2
	}
	return median
}

func calculateDistance(data []int, point int) int {
	distance := 0
	for i := 0; i < len(data); i++ {
		steps := int(math.Abs(float64(data[i]) - float64(point)))
		cost := 0
		for i := 1; i <= steps; i++ {
			cost += i
		}
		distance += cost
	}
	return distance
}

func main() {

	file, err := os.Open("d7/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var temp []string
	var initial_position_x []int
	var line string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line = scanner.Text()
	}

	temp = strings.Split(line, ",")
	for i := 0; i < len(temp); i++ {
		initial_position_x = append(initial_position_x, getInt(temp[i]))
	}

	//fmt.Printf("Initial positions: %v\n", initial_position_x)

	median := calculateMedian(initial_position_x)

	//fmt.Printf("median is %v\n", median)
	distance := calculateDistance(initial_position_x, median)

	min_distance := distance
	for i := 0; i < len(initial_position_x)/2; i++ {
		median++
		//fmt.Printf("median is %v\n", median)
		distance := calculateDistance(initial_position_x, median)
		//fmt.Printf("distance is %v\n", distance)
		if distance < min_distance {
			min_distance = distance
		} else {
			i = len(initial_position_x) / 2
		}

	}
	fmt.Printf("Min distance is %v\n", min_distance)
}
