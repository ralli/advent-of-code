package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const input = `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

func Test_Part1(t *testing.T) {
	result, err := Part1(input)
	assert.NoError(t, err)
	expected := 8
	assert.Equal(t, result, expected)
}

func Test_Part2(t *testing.T) {
	result, err := Part2(input)
	assert.NoError(t, err)
	expected := 2286
	assert.Equal(t, result, expected)
}
