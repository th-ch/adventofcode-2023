package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

type coord struct {
	x int
	y int
}

func adjacent(row, start, end, nbRows, nbCols int) []coord {
	res := []coord{}
	for i := start; i < end; i++ {
		if row < nbRows-1 {
			res = append(res, coord{x: row + 1, y: i})
		}
		if row > 0 {
			res = append(res, coord{x: row - 1, y: i})
		}
	}

	if start > 0 {
		res = append(res, coord{x: row, y: start - 1})
		if row > 0 {
			res = append(res, coord{x: row - 1, y: start - 1})
		}
		if row < nbRows-1 {
			res = append(res, coord{x: row + 1, y: start - 1})

		}
	}

	if end < nbCols-1 {
		res = append(res, coord{x: row, y: end})
		if row > 0 {
			res = append(res, coord{x: row - 1, y: end})
		}
		if row < nbRows-1 {
			res = append(res, coord{x: row + 1, y: end})

		}
	}
	return res
}

func run(s string) interface{} {
	engine := strings.Split(s, "\n")
	digits := regexp.MustCompile("([0-9]+)")
	symbols := regexp.MustCompile("[^0-9.]")
	nbRows, nbCols := len(engine), len(engine[0])
	res := 0
	for i, line := range engine {
		indexes := digits.FindAllStringIndex(line, -1)
		for _, index := range indexes {
			partId, _ := strconv.Atoi(line[index[0]:index[1]])
			neighbors := adjacent(i, index[0], index[1], nbRows, nbCols)

			for _, coord := range neighbors {
				if symbols.MatchString(string(engine[coord.x][coord.y])) {
					res += partId
					break
				}
			}
		}
	}
	return res
}

func main() {
	// Uncomment this line to disable garbage collection
	// debug.SetGCPercent(-1)

	var input []byte
	var err error
	if len(os.Args) > 1 {
		// Read input from file for local debugging
		input, err = os.ReadFile(os.Args[1])
		if err != nil {
			panic(err)
		}
		// Remove extra newline
		input = input[:len(input)-1]
	} else {
		// Read input from stdin
		input, err = io.ReadAll(os.Stdin)
		if err != nil {
			panic(err)
		}
	}

	// Start resolution
	start := time.Now()
	result := run(string(input))

	// Print result
	fmt.Printf("_duration:%f\n", time.Since(start).Seconds()*1000)
	fmt.Println(result)
}
