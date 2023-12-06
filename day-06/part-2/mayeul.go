package main

import (
	"fmt"
	"io"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	space := regexp.MustCompile(`\s`)
	timeS := space.ReplaceAllString(strings.Split(lines[0], ":")[1], "")
	distS := space.ReplaceAllString(strings.Split(lines[1], ":")[1], "")

	time, _ := strconv.ParseFloat(timeS, 64)
	dist, _ := strconv.ParseFloat(distS, 64)
	delta := math.Pow(time, 2.0) - 4*(dist+1)

	inf := math.Ceil(0.5 * (time - math.Sqrt(delta)))
	sup := math.Floor(0.5 * (time + math.Sqrt(delta)))

	if inf < 0 {
		inf = 0
	}
	if sup > time {
		sup = time
	}

	return int(sup-inf) + 1
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
