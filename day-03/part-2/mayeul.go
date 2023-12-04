package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
	"unicode"
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

func expandNumber(line string, x, nbCols int) (res int, start int, end int) {
	buf := ""
	start, end = x, x+1
	for i := x; i >= 0; i-- {
		if unicode.IsDigit(rune(line[i])) {
			buf = string(line[i]) + buf
			start--
		} else {
			break
		}
	}
	for i := x + 1; i < nbCols; i++ {
		if unicode.IsDigit(rune(line[i])) {
			buf += string(line[i])
			end++
		} else {
			break
		}
	}
	res, _ = strconv.Atoi(buf)
	return
}

func run(s string) interface{} {
	engine := strings.Split(s, "\n")
	gears := regexp.MustCompile(`\*`)
	nbRows, nbCols := len(engine), len(engine[0])
	res := 0
	for i, line := range engine {
		indexes := gears.FindAllStringIndex(line, -1)
		for _, index := range indexes {
			neighbors := adjacent(i, index[0], index[1], nbRows, nbCols)
			adjGears := []int{}

			explored := map[int]map[int]bool{}

			for _, coord := range neighbors {
				if visited, ok := explored[coord.x][coord.y]; ok && visited {
					continue
				}
				if unicode.IsDigit(rune(engine[coord.x][coord.y])) {
					gear, start, end := expandNumber(engine[coord.x], coord.y, nbCols)
					adjGears = append(adjGears, gear)
					for i := start; i < end; i++ {
						if _, ok := explored[coord.x]; !ok {
							explored[coord.x] = map[int]bool{}
						}
						explored[coord.x][i] = true
					}
				}
			}

			sort.Ints(adjGears)
			if len(adjGears) == 2 {
				res += adjGears[0] * adjGears[1]
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
