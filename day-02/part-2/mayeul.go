package main

import (
	"fmt"
	"io"
	"os"
	re "regexp"
	"strconv"
	"strings"
	"time"
)

func run(s string) interface{} {
	regex1 := re.MustCompile("^Game ([0-9]+): (.+)$")
	regex2 := re.MustCompile("([0-9]+) (red|blue|green)")
	lines := strings.Split(s, "\n")

	res := 0

	for _, line := range lines {
		parsedGame := regex1.FindStringSubmatch(line)
		content := parsedGame[2]
		minSet := map[string]int{"red": 0, "blue": 0, "green": 0}

		for _, grab := range strings.Split(content, ";") {
			for _, cube := range regex2.FindAllStringSubmatch(grab, -1) {
				color := cube[2]
				n, _ := strconv.Atoi(cube[1])

				if v := minSet[color]; v < n {
					minSet[color] = n
				}

			}
		}
		res += minSet["red"] * minSet["blue"] * minSet["green"]
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
