package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strings"
)

const (
	steps = 40
)

func main() {
	file, err := os.Open("d14/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	pairs := make(map[string]int, 0)
	letters := make(map[string]int, 0)
	rules := make(map[string][]string, 0)

	for scanner.Scan() {
		entry := scanner.Text()

		if strings.Contains(entry, "->") {
			s := strings.Split(entry, " -> ")

			pair := s[0]
			var out []string

			out = append(out, string(pair[0])+s[1])
			out = append(out, s[1]+string(pair[1]))

			rules[pair] = out

		} else if len(entry) > 1 {
			for i := 1; i < len(entry); i++ {
				pair := string(entry[i-1]) + string(entry[i])
				pairs[pair]++
				letters[string(entry[i-1])]++
				if i == len(entry)-1 {
					letters[string(entry[i])]++
				}
			}
		}
	}

	fmt.Printf("pairs:%v\n", pairs)
	fmt.Printf("letters:%v\n", letters)
	fmt.Printf("rules:%v\n", rules)

	for i := 0; i < steps; i++ {
		tempPairs := make(map[string]int, 0)
		for k, v := range pairs {
			tempPairs[k] = v
		}
		for k, v := range tempPairs {
			if _, ok := rules[k]; ok {
				pairs[k] -= v
				letters[string(k[0])] -= v
				letters[string(k[1])] -= v

				pairs[rules[k][0]] += v
				pairs[rules[k][1]] += v

				letters[string(rules[k][0][0])] += v
				letters[string(rules[k][0][1])] += v
				letters[string(rules[k][1][1])] += v

			}
		}
		fmt.Printf("After step %v\n", i+1)
		fmt.Printf("pairs:%v\n", pairs)
		fmt.Printf("letters:%v\n", letters)
	}

	fmt.Printf("final letters:%v\n", letters)

	max, min := 0, math.MaxInt
	for _, v := range letters {
		if v > max {
			max = v
		} else if v < min {
			min = v
		}
	}

	result := max - min
	fmt.Printf("result %v\n", result)

}
