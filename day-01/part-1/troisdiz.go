package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

func run(s string) interface{} {
	// Your code goes here
	currentSum := 0
	for _, line := range strings.Split(s, "\n") {
		digitsInLine := [2]int{-1, -1}
		if len(line) > 0 {
			for i := 0; i < len(line); i++ {
				char := line[i]
				if char >= '0' && char <= '9' {
					digitsInLine[0] = int(char - '0')
					break
				}
			}
			for i := len(line) - 1; i > -1; i-- {
				char := line[i]
				if char >= '0' && char <= '9' {
					digitsInLine[1] = int(char - '0')
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
