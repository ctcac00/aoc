package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("d3/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var entry []string
	for scanner.Scan() {
		entry = append(entry, scanner.Text())
	}

	var gamma string
	var epsilon string
	for i := 0; i < 12; i++ {
		common := 0
		for j := 0; j < len(entry); j++ {
			value, err := strconv.Atoi(string([]byte(entry[j])[i]))
			if err != nil {
				log.Fatal(err)
			}
			common += value
		}
		if common > len(entry)/2 {
			gamma = gamma + "1"
			epsilon = epsilon + "0"
		} else {
			gamma = gamma + "0"
			epsilon = epsilon + "1"
		}
	}

	fmt.Printf("gamma is %v\n", gamma)
	fmt.Printf("epsilon is %v\n", epsilon)

	gamma_decimal, err := strconv.ParseInt(gamma, 2, 64)
	if err != nil {
		fmt.Println(err)
		return
	}

	epsilon_decimal, err := strconv.ParseInt(epsilon, 2, 64)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("gamma decimal is %v\n", gamma_decimal)
	fmt.Printf("epsilon decimal is %v\n", epsilon_decimal)

	fmt.Printf("Result is %v\n", gamma_decimal*epsilon_decimal)

}
