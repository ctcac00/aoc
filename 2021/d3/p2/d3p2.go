package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func remove(slice []string, s int) []string {
	return append(slice[:s], slice[s+1:]...)
}

func removeEntries(entries []string, char string, position int) []string {
	for i := 0; i < len(entries); i++ {
		if entries[i][position] != char[0] {
			entries = remove(entries, i)
			i--
		}
	}
	return entries
}

func main() {
	file, err := os.Open("d3/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var entries []string
	for scanner.Scan() {
		entries = append(entries, scanner.Text())
	}

	oxygen := make([]string, len(entries))
	copy(oxygen, entries)

	co2 := make([]string, len(entries))
	copy(co2, entries)

	for i := 0; i < 12; i++ {
		common := 0
		for j := 0; j < len(oxygen); j++ {
			value, err := strconv.Atoi(string([]byte(oxygen[j])[i]))
			if err != nil {
				log.Fatal(err)
			}
			common += value
		}
		if len(oxygen) > 1 {
			if float64(common) >= float64(len(oxygen))/2 {
				// 1 is more common
				// remove all entries that do not have 1 in position i
				oxygen = removeEntries(oxygen, "1", i)
			} else {
				// 0 is more common
				oxygen = removeEntries(oxygen, "0", i)
			}
		}
	}

	fmt.Printf("oxygen generator rating is %v\n", oxygen[0])

	oxygen_decimal, err := strconv.ParseInt(oxygen[0], 2, 64)
	if err != nil {
		fmt.Println(err)
		return
	}

	for i := 0; i < 12; i++ {
		common := 0
		for j := 0; j < len(co2); j++ {
			value, err := strconv.Atoi(string([]byte(co2[j])[i]))
			if err != nil {
				log.Fatal(err)
			}
			common += value
		}
		if len(co2) > 1 {
			if float64(common) >= float64(len(co2))/2 {
				// 1 is more common
				// remove all entries that do not have 0 in position i
				co2 = removeEntries(co2, "0", i)
			} else {
				// 0 is more common
				co2 = removeEntries(co2, "1", i)
			}
		}
	}

	co2_decimal, err := strconv.ParseInt(co2[0], 2, 64)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("co2 generator rating is %v\n", co2[0])

	fmt.Printf("Total is %v\n", oxygen_decimal*co2_decimal)

}
