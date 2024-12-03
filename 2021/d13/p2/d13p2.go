package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type position struct {
	x int
	y int
}

type fold struct {
	axis string
	pos  int
}

func main() {
	file, err := os.Open("d13/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var folds []fold
	grid := make(map[position]struct{}, 0)
	maxX, maxY := 0, 0
	for scanner.Scan() {
		entry := scanner.Text()
		if strings.HasPrefix(entry, "fold") {
			t := strings.SplitAfter(entry, "fold along ")
			t = strings.Split(t[1], "=")
			pos, _ := strconv.Atoi(t[1])
			newFold := fold{
				axis: t[0],
				pos:  pos,
			}
			folds = append(folds, newFold)
		} else if len(entry) > 1 {
			t := strings.Split(entry, ",")
			x, _ := strconv.Atoi(t[0])
			y, _ := strconv.Atoi(t[1])

			p := position{
				x: x,
				y: y,
			}
			grid[p] = struct{}{}
		}
	}

	//fmt.Printf("Folds %v\n", folds)
	//fmt.Printf("Grid %v\n", grid)

	for _, fold := range folds {
		//fmt.Printf("Fold along %v,%v\n", fold.axis, fold.pos)
		result := make(map[position]struct{}, 0)
		for p := range grid {
			//fmt.Printf("Original pos: %v,%v\n", p.x, p.y)

			if fold.axis == "y" {
				//vertical fold - along y
				if p.y > fold.pos {
					distanceToFold := p.y - fold.pos
					newPoint := position{
						x: p.x,
						y: fold.pos - distanceToFold,
					}
					result[newPoint] = struct{}{}
				} else {
					result[p] = struct{}{}
				}
			} else {
				//horizontal fold - along x
				if p.x > fold.pos {
					distanceToFold := p.x - fold.pos
					newPoint := position{
						x: fold.pos - distanceToFold,
						y: p.y,
					}
					result[newPoint] = struct{}{}
				} else {
					result[p] = struct{}{}
				}
			}
			//fmt.Printf("New pos: %v,%v\n", p.x, p.y)
		}

		//update grid
		maxX = 0
		maxY = 0
		grid = make(map[position]struct{}, 0)
		for key, value := range result {
			grid[key] = value

			if key.x > maxX {
				maxX = key.x
			}
			if key.y > maxY {
				maxY = key.y
			}
		}

	}
	//fmt.Printf("points: %v\n", grid)

	for y := 0; y < maxY; y++ {
		for x := 0; x < maxX; x++ {
			key := position{
				x, y,
			}
			if _, ok := grid[key]; ok {
				fmt.Printf("#")
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Printf("\n")
	}

}
