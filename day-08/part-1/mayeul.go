package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strings"
	"time"
)

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	res := 0
	instructions := lines[0]

	r := regexp.MustCompile(`([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)`)
	nodes := map[string][]string{}
	for _, line := range lines[2:] {
		matches := r.FindStringSubmatch(line)
		if len(matches) == 0 {
			continue
		}
		nodes[matches[1]] = []string{matches[2], matches[3]}
	}

	root := "AAA"
	i := 0
	for root != "ZZZ" {
		if i >= len(instructions) {
			i = 0
		}
		instruction := instructions[i]

		if instruction == 'L' {
			root = nodes[root][0]
		} else {
			root = nodes[root][1]
		}
		i += 1
		res += 1
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
