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

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	res := 0
	r := re.MustCompile(`[0-9]`)
	for _, line := range lines {
		first := r.FindString(line)
		last := r.FindString(reverseString(line))

		n, err := strconv.Atoi(first)
		if err != nil {
			continue
		}
		m, err := strconv.Atoi(last)
		if err != nil {
			continue
		}

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
