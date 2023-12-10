package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

type StringValue struct {
	str   string
	value int
}

func run(s string) interface{} {

	stringDigits := [19]StringValue{
		{"0", 0},
		{"one", 1},
		{"1", 1},
		{"two", 2},
		{"2", 2},
		{"three", 3},
		{"3", 3},
		{"four", 4},
		{"4", 4},
		{"five", 5},
		{"5", 5},
		{"six", 6},
		{"6", 6},
		{"seven", 7},
		{"7", 7},
		{"eight", 8},
		{"8", 8},
		{"nine", 9},
		{"9", 9},
	}

	// Your code goes here
	currentSum := 0
	for _, line := range strings.Split(s, "\n") {
		digitsInLine := [2]int{-1, -1}
		if len(line) > 0 {
			found := false
			for i := 0; i < len(line); i++ {
				for _, stringDigit := range stringDigits {
					//fmt.Printf("Looking for %s in %s\n", stringDigit.str, line[i:])
					if strings.Index(line[i:], stringDigit.str) == 0 {
						//fmt.Printf("  => Found %s in %s\n", stringDigit.str, line[i:])
						digitsInLine[0] = stringDigit.value
						found = true
						break
					}
				}
				if found {
					break
				}
			}
			found = false
			for i := len(line) - 1; i > -1; i-- {
				for _, stringDigit := range stringDigits {
					if strings.Index(line[i:], stringDigit.str) == 0 {
						digitsInLine[1] = stringDigit.value
						found = true
						break
					}
				}
				if found {
					break
				}
			}
			lineNumber := digitsInLine[0]*10 + digitsInLine[1]
			//fmt.Printf("Line nb is %d\n", lineNumber)
			currentSum += lineNumber
		}
	}
	return currentSum
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
