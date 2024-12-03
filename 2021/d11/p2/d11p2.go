package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"

	"github.com/fatih/color"
)

const (
	maxSteps = 100
)

type position struct {
	x int
	y int
}

type octopus struct {
	energy            int
	flash             bool
	flashedNeighbours bool
	pos               position
	flashCount        int
	stepNumber        int
}

func (o *octopus) enableFlash() {
	o.flash = true
	o.flashCount++
	o.energy = 0
}

func (o *octopus) step() {
	o.energy++
	o.stepNumber++
	o.flashedNeighbours = false

	if o.flash {
		o.flash = false
	}
	if o.energy > 9 {
		o.enableFlash()
	}
}

func increaseEnergyNeighbours(o *octopus, octopuses []*octopus) {
	for _, neighbour := range octopuses {
		if math.Abs(float64(o.pos.x)-float64(neighbour.pos.x)) < 2 &&
			math.Abs(float64(o.pos.y)-float64(neighbour.pos.y)) < 2 &&
			!neighbour.flash {
			neighbour.energy++
			if neighbour.energy > 9 {
				neighbour.enableFlash()
				increaseEnergyNeighbours(neighbour, octopuses)
			}
		}
	}
	o.flashedNeighbours = true
}

func main() {
	file, err := os.Open("d11/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	octopuses := make([]*octopus, 0)
	y := 0
	for scanner.Scan() {
		entry := scanner.Text()
		for x, energy := range entry {
			o := octopus{
				energy: int(energy - '0'),
				pos: position{
					x: x,
					y: y,
				},
				flash: false,
			}
			octopuses = append(octopuses, &o)
		}
		y++
	}

	totalEnergy := 1
	step := 0
	for ok := true; ok; ok = totalEnergy != 0 {
		step++
		//fmt.Printf("Step %v\n", step)
		for _, o := range octopuses {
			o.step()
		}

		for _, o := range octopuses {
			if o.flash && !o.flashedNeighbours {
				increaseEnergyNeighbours(o, octopuses)
			}
		}

		//printMatrix(octopuses, y)
		totalEnergy = 0
		for _, o := range octopuses {
			totalEnergy += o.energy
		}
	}
	fmt.Printf("Total steps %v\n", step)

}

func printMatrix(octopuses []*octopus, y int) {

	fmt.Printf("\n")
	for i, o := range octopuses {
		blue := color.New(color.FgBlue).PrintfFunc()
		if o.energy == 0 {
			blue("%v", o.energy)
		} else {
			fmt.Printf("%v", o.energy)
		}

		if (i+1)%y == 0 {
			fmt.Printf("\n")
		}
	}
	fmt.Printf("\n")
}
