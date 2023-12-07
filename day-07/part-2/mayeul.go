package main

import (
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Value int

const (
	HIGH Value = iota
	PAIR
	TWOPAIRS
	THREE
	FULL
	FOUR
	FIVE
)

func toValue(l byte) int {
	switch string(l) {
	case "A":
		return 13
	case "K":
		return 12
	case "Q":
		return 11
	case "J":
		return 1
	case "T":
		return 10
	default:
		v, _ := strconv.Atoi(string(l))
		return v
	}
}

type hand struct {
	cards string
	bid   int
	value Value
}

type handList []hand

func (h handList) Len() int {
	return len(h)
}

func (h handList) Less(i, j int) bool {
	if h[i].value != h[j].value {
		return h[i].value < h[j].value
	}

	for k := 0; k < 5; k++ {
		if h[i].cards[k] != h[j].cards[k] {
			return toValue(h[i].cards[k]) < toValue(h[j].cards[k])
		}
	}
	return true
}

func (h handList) Swap(i, j int) {
	tmp := h[i]
	h[i] = h[j]
	h[j] = tmp
}

func run(s string) interface{} {
	lines := strings.Split(s, "\n")
	hands := handList{}
	res := 0
	for _, line := range lines {
		handS := strings.Split(line, " ")
		cards := handS[0]
		bid, _ := strconv.Atoi(handS[1])
		hand := hand{cards: cards, bid: bid}

		count := map[rune]int{}
		for _, l := range cards {
			if _, ok := count[l]; !ok {
				count[l] = 1
			} else {
				count[l] += 1
			}
		}
		if c, ok := count[rune('J')]; ok && len(count) != 1 {
			max := 0
			var maxRune rune
			for r, v := range count {
				if v > max && r != rune('J') {
					max = v
					maxRune = r
				}
			}
			count[maxRune] += c
			delete(count, rune('J'))
		}
		switch len(count) {
		case 5:
			hand.value = HIGH
		case 4:
			hand.value = PAIR
		case 3:
			max := 0
			for _, v := range count {
				if v > max {
					max = v
				}
			}
			if max == 3 {
				hand.value = THREE
			} else {
				hand.value = TWOPAIRS
			}
		case 2:
			max := 0
			for _, v := range count {
				if v > max {
					max = v
				}
			}
			if max == 4 {
				hand.value = FOUR
			} else {
				hand.value = FULL
			}
		case 1:
			hand.value = FIVE

		default:
			hand.value = HIGH
		}
		hands = append(hands, hand)
	}
	sort.Sort(hands)

	for i, hand := range hands {
		res += (i + 1) * hand.bid
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
