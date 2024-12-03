package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func getClosingTag(char string) string {
	switch char {
	case "<":
		return ">"
	case "{":
		return "}"
	case "(":
		return ")"
	case "[":
		return "]"
	}
	return ""

}

func findMatch(result string, entry string, pos int, closingTag string) (string, bool, string, int) {
	var index int
	var next_tag string
	for index = pos + 1; index < len(entry); index++ {
		next_tag = string(entry[index])
		if next_tag == closingTag {

			//found a matching closing tag
			return result, true, closingTag, index

		} else if getClosingTag(next_tag) != "" {

			// new opening tag
			lookingFor := getClosingTag(next_tag)
			result, found, what, where := findMatch(result, entry, index, lookingFor)
			if !found {
				return result, false, what, where
			} else {
				entry = entry[:where] + "_" + entry[where+1:]
			}
		} else if getClosingTag(next_tag) == "" && next_tag != "_" {
			// non matching closing tag
			fmt.Printf("Expected %v, but found %v\n", closingTag, next_tag)
			result = next_tag
			return result, false, next_tag, index
		}
	}
	// non matching closing tag
	return result, false, next_tag, index
}

func main() {
	file, err := os.Open("d10/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var results []string
	total := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		entry := scanner.Text()
		fmt.Printf("Entry %v\n", entry)
		result := ""
		result, _, _, _ = findMatch(result, entry, 0, getClosingTag(string(entry[0])))

		fmt.Printf("\n")
		fmt.Printf("result is %v\n", result)

		if result != "" {
			results = append(results, result)
		}

	}
	fmt.Printf("results are %v\n", results)

	for i := 0; i < len(results); i++ {
		switch string(results[i]) {
		case ">":
			total += 25137
		case "}":
			total += 1197
		case ")":
			total += 3
		case "]":
			total += 57
		}
	}

	fmt.Printf("total is %v\n", total)

}
