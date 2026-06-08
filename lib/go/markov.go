package main

import (
	"fmt"
	"math/rand"
)

type TransitionTable map[uint8]map[uint8]float64

func BuildTransitions(notes []uint8) TransitionTable {
	table := make(TransitionTable)
	for i := 0; i < len(notes)-1; i++ {
		from := notes[i]
		to := notes[i+1]
		if table[from] == nil {
			table[from] = make(map[uint8]float64)
		}
		table[from][to]++
	}
	for _, targets := range table {
		var total float64
		for _, count := range targets {
			total += count
		}
		for note, count := range targets {
			targets[note] = count / total
		}
	}
	return table
}

func Generate(table TransitionTable, start uint8, length int) []uint8 {
	seq := make([]uint8, 0, length)
	seq = append(seq, start)
	current := start
	for i := 0; i < length-1; i++ {
		if targets, ok := table[current]; ok {
			r := rand.Float64()
			var cumulative float64
			for next, prob := range targets {
				cumulative += prob
				if r <= cumulative {
					seq = append(seq, next)
					current = next
					break
				}
			}
		} else {
			seq = append(seq, current)
		}
	}
	return seq
}

func main() {
	training := []uint8{60, 62, 64, 65, 67, 65, 64, 62}
	table := BuildTransitions(training)
	result := Generate(table, 60, 16)
	fmt.Println("Training:", training)
	fmt.Println("Markov:", result)
}
