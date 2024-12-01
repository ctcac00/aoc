package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type path struct {
	S        position
	velocity position
}

type position struct {
	x int
	y int
}

type style struct {
	value int
	vel   position
}

func main() {

	file, err := os.Open("d17/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var x_min, x_max, y_min, y_max int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		entry := scanner.Text()

		// Calling the Sscanf() function which
		// returns the number of elements
		// successfully parsed and error if
		// it persists
		_, err := fmt.Sscanf(entry, "target area: x=%d..%d, y=%d..%d", &x_min, &x_max, &y_min, &y_max)
		// Below statements get
		// executed if there is any error
		if err != nil {
			panic(err)
		}

		fmt.Printf("target area: x=%d..%d, y=%d..%d\n", x_min, x_max, y_min, y_max)
	}

	//build target area grid
	targetMap := make(map[position]struct{})
	for y := y_min; y <= y_max; y++ {
		for x := x_min; x <= x_max; x++ {

			pos := position{
				x: x,
				y: y,
			}
			targetMap[pos] = struct{}{}
		}
	}

	fmt.Printf("Target map\n")
	for k := range targetMap {
		fmt.Printf("%v ", k)
	}

	S := position{
		x: 0,
		y: 0,
	}

	velocity := position{
		x: 0,
		y: 0,
	}

	style := style{
		value: 0,
		vel: position{
			x: 0,
			y: 0,
		},
	}

	winVel := make([]path, 0)
	for y := y_min; y <= x_max; y++ {
		for x := 0; x <= x_max; x++ {

			steps := 0

			S.x = 0
			S.y = 0

			velocity.x = x
			velocity.y = y

			currPath := make([]path, 0)

			for {

				newPath := path{
					S:        S,
					velocity: velocity,
				}
				currPath = append(currPath, newPath)

				//fmt.Printf("velocity is %v\n", velocity)

				// check if it has it the target
				if _, ok := targetMap[S]; ok {
					fmt.Printf("Boom! Hit the target at %v after %v steps\n", S, steps)
					winPath := path{
						S:        S,
						velocity: currPath[0].velocity,
					}
					winVel = append(winVel, winPath)
					v, vel := checkStyle(currPath)
					if v > style.value { // incorrect!! need to keep a trace of all the positions and check the highest only if it hits the targe
						style.value = v
						style.vel = vel
					}

					//fmt.Printf("Style is %v. ", style)
					fmt.Printf("Original velocity was %v\n", currPath[0].velocity)
					break
				} else if S.x > x_max || S.y < y_min {
					//fmt.Printf("Missed the target on %v after %v steps\n", S, steps)
					break
				}

				S.x += velocity.x
				S.y += velocity.y

				if velocity.x > 0 {
					velocity.x--
				} else if velocity.x < 0 {
					velocity.x++
				}

				velocity.y--
				steps++
			}
		}
	}

	fmt.Printf("Highest style was %v\n", style)

	fmt.Printf("Win velocities ")
	for _, v := range winVel {
		fmt.Printf("%v ", v.velocity)
	}

	fmt.Printf("# of Win velocities %v\n", len(winVel))
}

func checkStyle(path []path) (int, position) {

	style := 0
	velocity := position{
		x: 0,
		y: 0,
	}
	for _, v := range path {
		if v.S.y > style {
			style = v.S.y
			velocity = path[0].velocity
		}
	}
	return style, velocity
}
