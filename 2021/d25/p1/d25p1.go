package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type position struct {
	y int
	x int
}

type seacumber struct {
	pos  position
	east bool
	step int
}

func main() {
	file, err := os.Open("d25/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	seacumbers := make(map[position]*seacumber, 0)
	scanner := bufio.NewScanner(file)
	y := 0
	columns := 0
	for scanner.Scan() {
		entry := scanner.Text()

		for x := 0; x < len(entry); x++ {
			pos := position{y, x}

			switch entry[x] {
			case 'v':
				s := seacumber{
					pos:  position{y, x},
					east: false,
				}
				seacumbers[pos] = &s
			case '>':
				s := seacumber{
					pos:  position{y, x},
					east: true,
				}
				seacumbers[pos] = &s
			case '.':
				seacumbers[pos] = nil
			}
		}
		y++
		columns = len(entry)
	}
	rows := y
	//printSeafloor(seacumbers, rows, columns)

	step := 1

	for {
		moved := false
		//check east sea cucumbers first
		tempSeacumbers := make(map[position]*seacumber, 0)
		for y := 0; y < rows; y++ {
			for k, v := range seacumbers {
				tempSeacumbers[k] = v
			}
			for x := 0; x < columns; x++ {
				pos := position{y, x}
				if seacumbers[pos] == nil {
					// empty space
				} else if seacumbers[pos].east {
					// east sea cucumber
					var newPos position
					// we're at the end - so try the beginning of the row x = 0
					if pos.x+1 == columns {
						newPos = position{
							x: 0,
							y: pos.y,
						}
					} else {
						newPos = position{
							x: pos.x + 1,
							y: pos.y,
						}
					}
					if tempSeacumbers[newPos] == nil && seacumbers[pos].step != step {
						//can move to this position
						s := seacumbers[pos]
						s.step = step
						s.pos = newPos
						seacumbers[newPos] = s
						seacumbers[pos] = nil
						moved = true
					}
				}
			}

		}
		// check south facing sea cucumbers next
		for x := 0; x < columns; x++ {
			for k, v := range seacumbers {
				tempSeacumbers[k] = v
			}
			for y := 0; y < rows; y++ {
				pos := position{y, x}
				if seacumbers[pos] == nil {
					// empty space
				} else if !seacumbers[pos].east {
					// south sea cucumber
					var newPos position
					// we're at the end - so try the beginning of the columns y = 0
					if pos.y+1 == rows {
						newPos = position{
							x: pos.x,
							y: 0,
						}
					} else {
						newPos = position{
							x: pos.x,
							y: pos.y + 1,
						}
					}
					if tempSeacumbers[newPos] == nil && seacumbers[pos].step != step {
						//can move to this position
						s := seacumbers[pos]
						s.step = step
						s.pos = newPos
						seacumbers[newPos] = s
						seacumbers[pos] = nil
						moved = true
					}
				}
			}

		}

		if !moved {
			fmt.Printf("After %v steps\n", step)
			//printSeafloor(seacumbers, rows, columns)
			break
		}
		step++
	}

}

func printSeafloor(seacumbers map[position]*seacumber, rows int, columns int) {
	for y := 0; y < rows; y++ {
		for x := 0; x < columns; x++ {
			pos := position{y, x}
			if seacumbers[pos] == nil {
				fmt.Printf(".")
			} else if seacumbers[pos].east {
				fmt.Printf(">")
			} else {
				fmt.Printf("v")
			}
		}
		fmt.Printf("\n")
	}

}
