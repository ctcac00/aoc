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

func matches(input string, digits string) bool {
	if len(input) == len(digits) {
		total := 0
		for i := 0; i < len(digits); i++ {
			if strings.Contains(input, string(digits[i])) {
				total++
			}
			if total == len(digits) {
				return true
			}
		}
	}
	return false
}

func findDigit(output string, digits [10]string) int {
	for i := 0; i < len(digits); i++ {
		if matches(output, digits[i]) {
			return i
		}
	}
	return -1
}

func is2(input string, digits string) int {

	total := 0
	for i := 0; i < len(digits); i++ {
		if strings.Contains(input, string(digits[i])) {
			total++
		}
	}
	return total
}

func contains(input string, digits string) bool {

	total := 0
	for i := 0; i < len(digits); i++ {
		if strings.Contains(input, string(digits[i])) {
			total++
		}
	}
	if total == len(digits) {
		return true
	}
	return false
}

func main() {
	file, err := os.Open("d8/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var inputs [][]string
	var outputs [][]string
	scanner := bufio.NewScanner(file)
	n := 0
	total := 0

	for scanner.Scan() {
		entry := scanner.Text()

		input := strings.Split(strings.Split(entry, "|")[0], " ")
		inputs = append(inputs, input)

		output := strings.Split(strings.Split(entry, "|")[1], " ")
		outputs = append(outputs, output)

		//fmt.Printf("Entry is: %v\n", entry)
		//fmt.Printf("Input is: %v\n", input)
		//fmt.Printf("Output is: %v\n", output)
		n++
	}

	for i := 0; i < n; i++ {
		var digits [10]string
		for j := 0; j < len(inputs[i]); j++ {
			//fmt.Printf("Output is: %v\n", inputs[i][j])
			if len(inputs[i][j]) == 2 {
				//1
				//fmt.Printf("1 is %v\n", inputs[i][j])
				digits[1] = inputs[i][j]
			}
			if len(inputs[i][j]) == 4 {
				//4
				//fmt.Printf("4 is %v\n", inputs[i][j])
				digits[4] = inputs[i][j]
			}
			if len(inputs[i][j]) == 3 {
				//7
				//fmt.Printf("7 is %v\n", inputs[i][j])
				digits[7] = inputs[i][j]

			}
			if len(inputs[i][j]) == 7 {
				//8
				//fmt.Printf("8 is %v\n", inputs[i][j])
				digits[8] = inputs[i][j]
			}
		}

		for j := 0; j < len(inputs[i]); j++ {
			if len(inputs[i][j]) == 5 {
				//5, 2 or 3
				if contains(inputs[i][j], digits[1]) {
					//3
					digits[3] = inputs[i][j]
				} else {
					//2 or 5
					if is2(inputs[i][j], digits[4]) > 2 {
						//5
						digits[5] = inputs[i][j]
					} else {
						//2
						digits[2] = inputs[i][j]
					}

				}
			}
			if len(inputs[i][j]) == 6 {
				//9, 6 or 0
				if contains(inputs[i][j], digits[1]) {
					//9 or 0
					if contains(inputs[i][j], digits[4]) {
						//9
						digits[9] = inputs[i][j]
					} else {
						//0
						digits[0] = inputs[i][j]
					}
				} else {
					//6
					digits[6] = inputs[i][j]
				}
			}

		}

		fmt.Printf("Digits %v\n", digits)
		number := ""
		for a := 0; a < len(outputs[i]); a++ {
			if len(outputs[i][a]) > 1 {
				number += strconv.Itoa(findDigit(outputs[i][a], digits))
			}
		}
		fmt.Printf("Output number is %v\n", number)
		total += getInt(number)
	}

	fmt.Printf("Total is %v\n", total)

}
