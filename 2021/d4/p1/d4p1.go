package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func addRemaining(array [5][5]string) int {
	sum := 0
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {

			if array[j][i] != "-1" {
				value, err := strconv.Atoi(array[j][i])
				if err != nil {
					log.Fatal(err)
				}
				sum += value
			}
		}
	}
	return sum
}

func findWinningRow(array [5][5]string) int {

	for i := 0; i < 5; i++ {
		sum := 0
		for j := 0; j < 5; j++ {
			value, err := strconv.Atoi(array[i][j])
			if err != nil {
				log.Fatal(err)
			}
			sum += value
		}
		if sum == -5 {
			fmt.Printf("Found winning row %v\n", i)
			return i
		}
	}
	return -1
}

func findWinningColumn(array [5][5]string) int {

	for i := 0; i < 5; i++ {
		sum := 0
		for j := 0; j < 5; j++ {
			value, err := strconv.Atoi(array[j][i])
			if err != nil {
				log.Fatal(err)
			}
			sum += value
		}
		if sum == -5 {
			fmt.Printf("Found winning column %v\n", i)
			return i
		}
	}
	return -1
}

func contains(array [5]string, value string) int {

	for i := 0; i < len(array); i++ {
		if array[i] == value {
			return i
		}
	}
	return -1
}

func main() {
	file, err := os.Open("d4/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	line_n := 0
	board_number := 0
	board_row := 0
	var boards [100][5][5]string
	var draw []string
	for scanner.Scan() {
		line := scanner.Text()

		if line_n == 0 {
			draw = strings.Split(line, ",")
		} else if len(line) > 1 {
			content := strings.Split(line, " ")
			copy(boards[board_number][board_row][:], content)
			board_row++
		} else {
			board_row = 0
			if line_n > 2 {
				board_number++
			}
		}
		line_n++
	}

	winning_number := 0
	winning_board := 0
	board_index := 0
	for i := 0; i < len(draw); i++ {
		for board_index = 0; board_index < 100; board_index++ {
			for j := 0; j < 5; j++ {
				fmt.Printf("checking if value %v exists in row %v of the board number %v with content %v\n", draw[i], j, board_index, boards[board_index][j])
				index := contains(boards[board_index][j], draw[i])
				if index > -1 {
					boards[board_index][j][index] = "-1"
				}

				win_row := findWinningRow(boards[board_index])
				win_col := findWinningColumn(boards[board_index])

				if win_row > -1 {
					// BINGO
					fmt.Printf("Bingo! Winning row is %v\n", win_row)
					fmt.Printf("Bingo! Remaining numbers are %v\n", boards[board_index])

					winning_number, err = strconv.Atoi(draw[i])
					if err != nil {
						log.Fatal(err)
					}

					fmt.Printf("Winning number is %v\n", winning_number)

					j = 5
					i = len(draw)
					winning_board = board_index
					board_index = 100

				} else if win_col > -1 {
					// BINGO
					fmt.Printf("Bingo! Winning column is %v\n", win_col)
					fmt.Printf("Bingo! Remaining numbers are %v\n", boards[board_index])

					winning_number, err = strconv.Atoi(draw[i])
					if err != nil {
						log.Fatal(err)
					}

					fmt.Printf("Winning number is %v\n", winning_number)

					j = 5
					i = len(draw)
					winning_board = board_index
					board_index = 100

				}
			}
		}
	}

	remaining := addRemaining(boards[winning_board])
	fmt.Printf("Remaining total is is %v\n", remaining)
	fmt.Printf("Result is %v\n", remaining*winning_number)

}
