package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
	"unicode"
)

func run(s string) int {
	lines := strings.Split(s, "\n")
	sum := 0

	var firstDigit, lastDigit int
	for _, line := range lines {
		firstDigit = -1
		for _, char := range line {
			if unicode.IsDigit(char) {
				if firstDigit == -1 {
					firstDigit = int(char - '0')
				}
				lastDigit = int(char - '0')
			}
		}
		sum += 10*firstDigit + lastDigit
	}
	return sum
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
