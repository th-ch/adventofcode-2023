package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strings"
	"sync"
	"time"
)

func gcd(a, b uint64) uint64 {
	if a < b {
		tmp := a
		a = b
		b = tmp
	}
	for b != 0 {
		r := a % b
		a = b
		b = r
	}
	return a
}

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	instructions := lines[0]

	r := regexp.MustCompile(`([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)`)
	nodes := map[string][]string{}
	for _, line := range lines[2:] {
		matches := r.FindStringSubmatch(line)
		if len(matches) == 0 {
			continue
		}
		nodes[matches[1]] = []string{matches[2], matches[3]}
	}

	roots := []string{}

	for k := range nodes {
		if k[2] == 'A' {
			roots = append(roots, k)
		}
	}

	firstInst := instructions[0]
	cycles := make([]uint64, len(roots))
	var wg sync.WaitGroup
	for j, root := range roots {
		wg.Add(1)
		go func(j int, root string) {
			defer wg.Done()
			cur := ""
			length := uint64(1)
			i := 1
			if firstInst == 'L' {
				cur = nodes[root][0]
			} else {
				cur = nodes[root][1]
			}

			for cur[2] != 'Z' {
				if i >= len(instructions) {
					i = 0
				}
				instruction := instructions[i]

				if instruction == 'L' {
					cur = nodes[cur][0]
				} else {
					cur = nodes[cur][1]
				}
				i += 1
				length += 1
			}
			cycles[j] = length
		}(j, root)
	}
	wg.Wait()

	g := cycles[0]
	p := cycles[0]
	for _, cycle := range cycles[1:] {
		g = gcd(g, cycle)
		p *= cycle / g
	}

	return p
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
