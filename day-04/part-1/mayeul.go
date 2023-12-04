package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"sort"
	"strings"
	"time"
)

func run(s string) interface{} {
	r := regexp.MustCompile(`Card(?: )+([0-9]+): ([0-9 ]+) \| ([0-9 ]+)`)
	res := 0
	for _, line := range strings.Split(s, "\n") {
		matches := r.FindAllStringSubmatch(line, -1)
		// id := matches[0][1]
		card := strings.Split(matches[0][2], " ")
		num := strings.Split(matches[0][3], " ")

		sort.Strings(card)
		sort.Strings(num)

		i, j, score := 0, 0, 0
		for i < len(card) && j < len(num) {
			if card[i] < num[j] || len(card[i]) == 0 {
				i++
				continue
			}
			if card[i] > num[j] || len(num[j]) == 0 {
				j++
				continue
			}

			if card[i] == num[j] {
				if score == 0 {
					score = 1
				} else {
					score *= 2
				}

				i += 1
				j += 1
				continue
			}
		}
		res += score
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
