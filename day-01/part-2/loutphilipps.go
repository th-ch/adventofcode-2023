package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"

	// "time"
	"unicode"
)

func getCalibration(line string) int {
	nameToInt := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}

	candidate := ""
	firstDigit := -1
	lastDigit := -1
	for _, char := range line {
		if unicode.IsDigit(char) {
			candidate = ""
			if firstDigit == -1 {
				firstDigit = int(char - '0')
			}
			lastDigit = int(char - '0')
		} else if unicode.IsLetter(char) {
			candidate += string(char)
			for name, value := range nameToInt {
				if strings.HasSuffix(candidate, name) {
					if firstDigit == -1 {
						firstDigit = value
					}
					lastDigit = value
				}
			}
		}
	}
	return 10*firstDigit + lastDigit
}

func run(s string) int {
	lines := strings.Split(s, "\n")
	sum := 0

	for _, line := range lines {
		sum += getCalibration(line)
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
