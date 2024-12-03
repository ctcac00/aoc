package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"log"
	"os"

	"github.com/fatih/color"
)

type position struct {
	x, y, risk int
}

// An Item is something we manage in a priority queue.
type Item struct {
	value    position // The value of the item; arbitrary.
	priority int      // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) update(item *Item, value position, priority int) {
	item.value = value
	item.priority = priority
	heap.Fix(pq, item.index)
}

var (
	moves = []position{
		{x: 0, y: 1},  // down
		{x: 1, y: 0},  // right
		{x: -1, y: 0}, // left
		{x: 0, y: -1}, // up
	}
)

func main() {
	file, err := os.Open("d15/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	cavern := make([][]int, 0)
	y := 0
	for scanner.Scan() {
		entry := scanner.Text()

		row := make([]int, 0)
		for _, v := range entry {
			c := v - '0'
			row = append(row, int(c))
		}
		cavern = append(cavern, row)

		y++
	}

	//duplicate to right 4 times
	original_size := len(cavern)
	for i := 0; i < 4; i++ {
		for y := 0; y < original_size; y++ {
			row := cavern[y]
			for x := i * original_size; x < i*original_size+original_size; x++ {
				// append new values to row
				new_value := cavern[y][x] + 1
				if new_value > 9 {
					new_value = 1
				}
				row = append(row, new_value)
			}
			cavern[y] = row
		}
	}

	//duplicate to bottom 4 times
	for i := 0; i < 4; i++ {
		for y := i * original_size; y < i*original_size+original_size; y++ {
			row := make([]int, 0)
			for x := 0; x < len(cavern[y]); x++ {
				// append new values to row
				new_value := cavern[y][x] + 1
				if new_value > 9 {
					new_value = 1
				}
				row = append(row, new_value)
			}
			cavern = append(cavern, row)
		}
	}

	//printCavern(cavern)

	start := position{
		x: 0, y: 0, risk: cavern[0][0],
	}
	end := position{
		x: len(cavern[0]) - 1, y: len(cavern[0]) - 1, risk: cavern[len(cavern[0])-1][len(cavern[0])-1],
	}

	var neighbours func(p position) []position
	neighbours = func(p position) []position {
		var pos []position

		for _, m := range moves {
			newPos := position{x: p.x + m.x, y: p.y + m.y}

			if newPos.x >= 0 && newPos.x < len(cavern[0]) && newPos.y >= 0 && newPos.y < len(cavern) {
				// found neighbour
				newPos.risk = cavern[newPos.y][newPos.x]
				pos = append(pos, newPos)
			}
		}

		return pos
	}

	frontier := make(PriorityQueue, 0)
	heap.Init(&frontier)

	// Insert a new item and then modify its priority.
	item := &Item{
		value:    start,
		priority: start.risk,
	}
	heap.Push(&frontier, item)
	came_from := make(map[position]position, 0)
	cost_so_far := make(map[position]int, 0)

	came_from[start] = position{}
	cost_so_far[start] = start.risk

	for frontier.Len() > 0 {
		current := heap.Pop(&frontier).(*Item)

		if current.value == end {
			break
		}

		n := neighbours(current.value)
		for _, next := range n {
			new_cost := cost_so_far[current.value] + next.risk

			if v, ok := cost_so_far[next]; !ok || new_cost < v {
				cost_so_far[next] = new_cost
				priority := new_cost

				item := &Item{
					value:    next,
					priority: priority,
				}
				heap.Push(&frontier, item)
				came_from[next] = current.value
			}
		}
	}

	current := end
	path := make([]position, 0)

	totalRisk := 0
	for current != start {
		totalRisk += cavern[current.y][current.x]
		path = append(path, current)
		current = came_from[current]
	}

	//printPath(path, cavern)
	fmt.Printf("Total risk is %v\n", totalRisk)
}

func printCavern(cavern [][]int) {
	fmt.Printf("\n")
	for y := 0; y < len(cavern); y++ {
		for x := 0; x < len(cavern[y]); x++ {
			fmt.Printf("%v", cavern[y][x])
		}
		fmt.Printf("\n")
	}
	fmt.Printf("\n")
}

func printPath(path []position, cavern [][]int) {
	fmt.Printf("\n")
	blue := color.New(color.FgBlue).PrintfFunc()

	for y := 0; y < len(cavern); y++ {
		for x := 0; x < len(cavern[y]); x++ {
			color := false
			for _, v := range path {
				if v.x == x && v.y == y {
					blue("%v", cavern[y][x])
					color = true
				}
			}
			if !color {
				fmt.Printf("%v", cavern[y][x])
			}
		}
		fmt.Printf("\n")
	}

	fmt.Printf("\n")
}
