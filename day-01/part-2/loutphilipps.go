package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strings"
	"time"

	// "time"
	"unicode"
)

func digitNameToInt(s string) int {
	switch s {
	case "one":
		return 1
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	default:
		fmt.Println(fmt.Errorf("string %s is not a valid number", s))
		return -1
	}
}

func getCalibration(line string) int {
	re := regexp.MustCompile(".*(one|two|three|four|five|six|seven|eight|nine).*")

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
			matches := re.FindAllStringSubmatch(candidate, -1)
			if len(matches) > 0 {
				digit := digitNameToInt(matches[len(matches)-1][len(matches[len(matches)-1])-1])
				if firstDigit == -1 {
					firstDigit = digit
				}
				lastDigit = digit
			} else if len(matches) > 2 {
				fmt.Println("More than one digit name found, this is unexpected")
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
