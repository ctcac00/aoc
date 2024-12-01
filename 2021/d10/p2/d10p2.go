package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
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

func checkIfOpenOnly(entry string) bool {
	for i := 0; i < len(entry); i++ {
		if getClosingTag(string(entry[i])) == "" {
			return false
		}
	}
	return true
}

func completeEntry(result string, entry string, pos int, closingTag string) (string, bool, string, int, string) {
	var index int
	var next_tag string
	for index = pos + 1; index < len(entry); index++ {
		next_tag = string(entry[index])
		if next_tag == closingTag {
			//found a matching closing tag
			//remove matching tags
			entry = entry[:pos] + entry[pos+1:]
			entry = entry[:index-1] + entry[index:]

			return result, true, closingTag, index, entry
		} else if getClosingTag(next_tag) != "" {
			// new opening tag
			lookingFor := getClosingTag(next_tag)
			var what string
			var found bool
			var where int
			result, found, what, where, entry = completeEntry(result, entry, index, lookingFor)
			if !found {
				return result, false, what, where, entry
			} else {
				closed := checkIfOpenOnly(entry)
				if closed {
					index = len(entry)
				} else {
					index--
				}
			}
		}
	}
	return result, true, next_tag, index, entry
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
			//fmt.Printf("Expected %v, but found %v\n", closingTag, next_tag)
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
	var total []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		entry := scanner.Text()
		//fmt.Printf("Entry %v\n", entry)
		result := ""
		result, _, _, _ = findMatch(result, entry, 0, getClosingTag(string(entry[0])))

		if result != "" {
			//fmt.Printf("result is %v\n", result)
			results = append(results, result)
		} else {
			subtotal := 0
			//fmt.Printf("Possible imcomplete line\n")
			result, _, _, _, entry = completeEntry(result, entry, 0, getClosingTag(string(entry[0])))

			closed := checkIfOpenOnly(entry)
			if !closed {
				result, _, _, _, entry = completeEntry(result, entry, 0, getClosingTag(string(entry[0])))
			}

			fmt.Printf("entry %v\n", entry)

			for i := len(entry) - 1; i >= 0; i-- {
				switch string(entry[i]) {
				case "<":
					subtotal = subtotal*5 + 4
				case "{":
					subtotal = subtotal*5 + 3
				case "(":
					subtotal = subtotal*5 + 1
				case "[":
					subtotal = subtotal*5 + 2
				}
			}

			fmt.Printf("subtotal is %v\n", subtotal)
			total = append(total, subtotal)
		}

	}
	fmt.Printf("total is %v\n", total)
	sort.Ints(total)
	fmt.Printf("total is %v\n", total[(len(total)-1)/2])

}
