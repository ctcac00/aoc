package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

type cave struct {
	name         string
	bigCave      bool
	timesVisited int
}

type path struct {
	start cave
	end   cave
}

func main() {
	file, err := os.Open("d12/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	paths := make([]path, 0)
	for scanner.Scan() {
		entry := scanner.Text()
		caves := strings.Split(entry, "-")
		p := path{
			start: cave{
				name:    caves[0],
				bigCave: !unicode.IsLower(rune(caves[0][0])),
			},
			end: cave{
				name:    caves[1],
				bigCave: !unicode.IsLower(rune(caves[1][0])),
			},
		}
		paths = append(paths, p)
	}

	//fmt.Printf("Paths\n")
	//printPaths(paths)

	var (
		currentPath []cave
		allPaths    [][]cave
		visited     map[cave]int
	)

	smallCavesTwiceVisited := func() bool { //return true if there are small caves that have been visited more than once
		for _, v := range visited {
			if v == 2 {
				return true
			}
		}
		return false
	}

	var dfs func(start cave, end cave, paths []path)

	dfs = func(start cave, end cave, paths []path) {
		if start.name == "start" || start.name == "end" {
			if visited[start] == 0 {
				visited[start]++
			} else {
				return
			}
		} else if !start.bigCave {
			if visited[start] == 0 {
				visited[start]++
			} else if visited[start] == 1 && !smallCavesTwiceVisited() { // check if there are other caves that have been visited more thance once
				visited[start]++
			} else {
				return
			}
		}

		currentPath = append(currentPath, start)
		if start.name == end.name {
			//displayPath(currentPath)
			allPaths = append(allPaths, currentPath)
			visited[start]--
			currentPath = currentPath[:len(currentPath)-1]
			return
		}
		n := neighbors(start, paths)
		for _, next := range n {
			dfs(next, end, paths)
		}
		visited[start]--
		currentPath = currentPath[:len(currentPath)-1]
	}

	start, _ := findCave("start", paths)
	end, _ := findCave("end", paths)
	visited = make(map[cave]int)
	dfs(start, end, paths)
	fmt.Printf("Result: %v", len(allPaths))

}

func displayPath(p []cave) {
	fmt.Println()
	for _, v := range p {
		fmt.Printf("%v,", v.name)
	}

}

func printPaths(p []path) {
	for _, v := range p {
		fmt.Printf("%v-%v\n", v.start, v.end)
	}
}

func neighbors(c cave, paths []path) []cave {
	var caves []cave

	for _, v := range paths {
		if c.name == v.start.name {
			// found neighbour
			caves = append(caves, v.end)
		}
		if c.name == v.end.name {
			// found neighbour
			caves = append(caves, v.start)
		}
	}
	return caves

}

func findCave(name string, paths []path) (cave, error) {
	var c cave
	for _, v := range paths {
		if name == v.start.name {
			return v.start, nil
		}
		if name == v.end.name {
			return v.end, nil
		}
	}
	return c, errors.New("Cave not found")
}
