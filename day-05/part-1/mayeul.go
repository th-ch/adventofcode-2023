package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

type mapping struct {
	srcStart  int
	destStart int
	length    int
}

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	seeds := strings.Split(strings.Trim(strings.Split(lines[0], ":")[1], " "), " ")
	parse := regexp.MustCompile("^([a-z]+)-to-([a-z]+) map:$")

	relations := map[string]map[string]map[int]mapping{}

	seedIds := []int{}
	for _, seed := range seeds {
		id, _ := strconv.Atoi(seed)
		seedIds = append(seedIds, id)
	}

	currentSrc := ""
	currentDest := ""
	for _, line := range lines[1:] {
		if match := parse.FindAllStringSubmatch(line, -1); len(match) != 0 {
			currentSrc = match[0][1]
			currentDest = match[0][2]
			relations[currentSrc] = map[string]map[int]mapping{}
			relations[currentSrc][currentDest] = map[int]mapping{}
			continue
		}

		if nums := strings.Split(line, " "); len(nums) == 3 {
			dst, _ := strconv.Atoi(nums[0])
			src, _ := strconv.Atoi(nums[1])
			length, _ := strconv.Atoi(nums[2])
			relations[currentSrc][currentDest][src] = mapping{srcStart: src, destStart: dst, length: length}
		}
	}

	res := -1
	for _, id := range seedIds {
		src := "seed"
		dst := ""
		currentId := id
		for k := range relations["seed"] {
			dst = k
		}
		for src != "location" {
			for k, v := range relations[src][dst] {
				if currentId >= k && currentId < k+v.length {
					currentId = currentId - v.srcStart + v.destStart
					break
				}
			}
			src = dst
			newDst := ""
			for k := range relations[dst] {
				newDst = k
			}
			dst = newDst
		}

		if res == -1 || res > currentId {
			res = currentId
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
