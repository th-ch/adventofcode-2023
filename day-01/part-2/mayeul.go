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

func reverseString(s string) string {
	res := ""
	for _, l := range s {
		res = string(l) + res
	}
	return res
}

func convertToInt(s string) int {
	if n, err := strconv.Atoi(s); err == nil {
		return n
	} else {
		switch s {
		case "zero":
			return 0
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
		}
	}
	return 0
}

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	res := 0

	regexp := "zero|one|two|three|four|five|six|seven|eight|nine"
	lToR := re.MustCompile(regexp + "|[0-9]")
	rToL := re.MustCompile(reverseString(regexp) + "|[0-9]")
	for _, line := range lines {
		first := lToR.FindString(line)
		last := rToL.FindString(reverseString(line))

		n := convertToInt(first)
		m := convertToInt(reverseString(last))
		res += n*10 + m
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
