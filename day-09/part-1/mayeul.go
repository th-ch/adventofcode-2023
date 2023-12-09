package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
	"time"
)

func isZero(values []int) bool {
	for _, v := range values {
		if v != 0 {
			return false
		}
	}
	return true
}

func diff(values []int) int {
	if isZero(values) {
		return 0
	}
	newValues := []int{}
	for i := 0; i < len(values)-1; i++ {
		newValues = append(newValues, values[i+1]-values[i])
	}
	return values[len(values)-1] + diff(newValues)
}

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	res := 0
	for _, line := range lines {
		values := []int{}
		for _, num := range strings.Split(line, " ") {
			value, _ := strconv.Atoi(num)
			values = append(values, value)
		}
		res += diff(values)
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
