package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strings"
)

const (
	steps = 10
)

type rule struct {
	match  []rune
	insert rune
}

func main() {
	file, err := os.Open("d14/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var template []rune
	var rules []rule
	for scanner.Scan() {
		entry := scanner.Text()

		if strings.Contains(entry, "->") {
			s := strings.Split(entry, " -> ")
			rule := rule{
				match:  []rune(s[0]),
				insert: []rune(s[1])[0],
			}
			rules = append(rules, rule)

		} else if len(entry) > 1 {
			template = []rune(entry)
		}
	}

	//fmt.Printf("Template: %v", string(template))
	for i := 0; i < steps; i++ {
		insertions := make(map[int]rune, 0)
		for _, rule := range rules {
			//fmt.Printf("Insertion rule: %v\n", rule)
			for j := 1; j < len(template); j++ {
				if template[j] == rule.match[1] && template[j-1] == rule.match[0] {
					insertions[j] = rule.insert
				}
			}
		}

		keys := make([]int, 0, len(insertions))
		for k := range insertions {
			keys = append(keys, k)
		}
		sort.Ints(keys)

		count := 0
		for _, k := range keys {
			if k > len(template) {
				template = append(template, insertions[k])
			}
			template = insert(template, k+count, insertions[k])
			count++
		}
		//fmt.Printf("After step %v: %v\n", i+1, (string(template)))
	}

	groupByRune := make(map[rune]int, 0)
	for _, v := range template {
		if _, ok := groupByRune[v]; ok {
			groupByRune[v]++
		} else {
			groupByRune[v] = 1
		}
	}

	max, min := 0, math.MaxInt
	for _, v := range groupByRune {
		if v > max {
			max = v
		} else if v < min {
			min = v
		}
	}

	result := max - min
	fmt.Printf("result %v\n", result)

}

// 0 <= index <= len(a)
func insert(a []rune, index int, value rune) []rune {
	if len(a) == index { // nil or empty slice or after last element
		return append(a, value)
	}
	a = append(a[:index+1], a[index:]...) // index < len(a)
	a[index] = value
	return a
}
