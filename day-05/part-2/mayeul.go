package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
)

type mapping struct {
	srcStart  int
	destStart int
	length    int
}

type Range struct {
	id     int
	length int
}

type RangeList []Range

func (r RangeList) Len() int {
	return len(r)
}

func (r RangeList) Less(i, j int) bool {
	return r[i].id < r[j].id
}

func (r RangeList) Swap(i, j int) {
	tmp := r[j]
	r[j] = r[i]
	r[i] = tmp
}

func getNewRanges(mapping *map[int]mapping, r Range) []Range {
	if r.length == 0 {
		return nil
	}
	res := []Range{}
	match := false
	for k, v := range *mapping {
		if r.id < k+v.length {
			if r.id >= k {
				if r.id+r.length > k+v.length {
					res = append(res, Range{id: r.id - k + v.destStart, length: k - r.id + v.length})
					res = append(res, getNewRanges(mapping, Range{id: k + v.length, length: r.length - k - v.length + r.id})...)
				} else {
					res = append(res, Range{id: r.id - k + v.destStart, length: r.length})
				}
				match = true
				break
			}

			if r.id+r.length-1 >= k {
				if r.id+r.length-1 < k+v.length {
					res = append(res, getNewRanges(mapping, Range{id: r.id, length: k - r.id})...)
					res = append(res, Range{id: v.destStart, length: r.length - k + r.id})
				} else {
					res = append(res, getNewRanges(mapping, Range{id: r.id, length: k - r.id})...)
					res = append(res, Range{id: v.destStart, length: v.length})
					res = append(res, getNewRanges(mapping, Range{id: k + v.length, length: r.length - v.length - k + r.id})...)
				}
				match = true
				break
			}
		}
	}
	if !match {
		return []Range{r}
	}
	return res
}

func compactRange(ranges RangeList) []Range {
	res := []Range{}

	sort.Sort(ranges)

	currentRange := Range{id: ranges[0].id, length: ranges[0].length}
	for _, r := range ranges[1:] {
		if r.id == (currentRange.id + currentRange.length) {
			currentRange.length += r.length
		} else {
			res = append(res, currentRange)
			currentRange = Range{id: r.id, length: r.length}
		}
	}
	res = append(res, currentRange)
	return res
}

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	seeds := strings.Split(strings.Trim(strings.Split(lines[0], ":")[1], " "), " ")
	parse := regexp.MustCompile("^([a-z]+)-to-([a-z]+) map:$")

	relations := map[string]map[string]map[int]mapping{}

	seedIds := []Range{}
	for i := 0; i < len(seeds)-1; i += 2 {
		id, _ := strconv.Atoi(seeds[i])
		length, _ := strconv.Atoi(seeds[i+1])
		seedIds = append(seedIds, Range{id: id, length: length})
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
	for _, seedId := range seedIds {
		src := "seed"
		dst := ""
		currentRanges := []Range{seedId}
		for k := range relations["seed"] {
			dst = k
		}
		for src != "location" {
			newRanges := []Range{}

			for _, r := range currentRanges {
				m := relations[src][dst]
				newRanges = append(newRanges, getNewRanges(&m, r)...)
			}
			src = dst
			newDst := ""
			for k := range relations[dst] {
				newDst = k
			}
			dst = newDst
			currentRanges = compactRange(newRanges)
		}

		for _, r := range currentRanges {
			if res == -1 || res > r.id {
				res = r.id
			}
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
