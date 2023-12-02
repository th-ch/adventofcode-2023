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
		id, _ := strconv.Atoi(parsedGame[1])
		content := parsedGame[2]
		valid := true
		for _, grab := range strings.Split(content, ";") {
			red, blue, green := 0, 0, 0
			for _, cube := range regex2.FindAllStringSubmatch(grab, -1) {
				color := cube[2]
				n, _ := strconv.Atoi(cube[1])

				switch color {
				case "red":
					red += n
				case "blue":
					blue += n
				case "green":
					green += n
				default:
					continue
				}
			}
			if red > 12 || green > 13 || blue > 14 {
				valid = false
				break
			}
		}
		if valid {
			res += id
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
