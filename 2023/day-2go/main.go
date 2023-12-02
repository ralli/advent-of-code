package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	filename := "day-2.txt"
	b, err := os.ReadFile(filename)
	if err != nil {
		log.Fatalf("cannot read %s: %v", filename, err)
	}
	input := string(b)

	result, err := Part1(input)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%d\n", result)

	result, err = Part2(input)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%d\n", result)
}

func Part1(input string) (int, error) {
	gameList, err := ParseInput(input)
	if err != nil {
		return 0, err
	}

	result := 0

	for _, game := range gameList.Games {
		if game.IsPossible() {
			result += game.ID
		}
	}

	return result, nil
}

func Part2(input string) (int, error) {
	gameList, err := ParseInput(input)
	if err != nil {
		return 0, err
	}

	result := 0

	for _, game := range gameList.Games {
		counts := game.MinCounts()
		power := counts.Power()
		result += power
	}

	return result, nil
}

type GameList struct {
	Games []Game
}

type Game struct {
	ID    int
	Moves []Move
}

type Move struct {
	Cubes []Cube
}

type Cube struct {
	Count int
	Color string
}

func (game *Game) IsPossible() bool {
	for _, move := range game.Moves {
		if !move.IsPossible() {
			return false
		}
	}
	return true
}

func (game *Game) MinCounts() Counts {
	red := 0
	green := 0
	blue := 0

	for _, move := range game.Moves {
		counts := move.CubeCounts()
		red = max(red, counts.Red)
		green = max(green, counts.Green)
		blue = max(blue, counts.Blue)
	}

	return Counts{Red: red, Green: green, Blue: blue}
}

func (move *Move) IsPossible() bool {
	counts := move.CubeCounts()
	return counts.Red <= 12 && counts.Green <= 13 && counts.Blue <= 14
}

func (move *Move) CubeCounts() Counts {
	var red int
	var green int
	var blue int

	for _, cube := range move.Cubes {
		if cube.Color == "red" {
			red += cube.Count
		} else if cube.Color == "green" {
			green += cube.Count
		} else {
			blue += cube.Count
		}
	}

	return Counts{Red: red, Green: green, Blue: blue}
}

type Counts struct {
	Red   int
	Green int
	Blue  int
}

func (counts *Counts) Power() int {
	return counts.Red * counts.Green * counts.Blue
}

func ToJson(x interface{}) string {
	b, err := json.MarshalIndent(x, " ", " ")
	if err != nil {
		log.Fatal(err)
	}
	return string(b)
}

func ParseInput(input string) (*GameList, error) {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var games []Game
	for scanner.Scan() {
		game, err := ParseGame(scanner.Text())
		if err != nil {
			return nil, err
		}
		games = append(games, *game)
	}
	return &GameList{Games: games}, nil
}

func ParseGame(line string) (*Game, error) {
	parts := strings.Split(line, ": ")
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid game line: %s", line)
	}
	gamePart := parts[0]
	movesPart := parts[1]

	id, err := ParseGameCount(gamePart)
	if err != nil {
		return nil, fmt.Errorf("cannot parse game id: %w", err)
	}

	moves, err := ParseMoves(movesPart)
	if err != nil {
		return nil, fmt.Errorf("cannot parse cubes: %w", err)
	}
	return &Game{ID: id, Moves: moves}, nil
}

func ParseGameCount(input string) (int, error) {
	parts := strings.Split(input, " ")
	id, err := strconv.Atoi(parts[1])
	if err != nil {
		return 0, fmt.Errorf("cannot parse game id %s: %w", parts[1], err)
	}
	return id, nil
}

func ParseMoves(input string) ([]Move, error) {
	parts := strings.Split(input, "; ")
	var moves []Move

	for _, part := range parts {
		move, err := ParseMove(part)
		if err != nil {
			return nil, fmt.Errorf("cannot parse move %s: %w", part, err)
		}
		moves = append(moves, *move)
	}

	return moves, nil
}

func ParseMove(input string) (*Move, error) {
	parts := strings.Split(input, ", ")
	var cubes []Cube

	for _, part := range parts {
		cube, err := ParseCube(part)
		if err != nil {
			return nil, fmt.Errorf("cannot parse cube %s: %w", part, err)
		}
		cubes = append(cubes, *cube)
	}

	return &Move{Cubes: cubes}, nil
}

func ParseCube(input string) (*Cube, error) {
	parts := strings.Split(input, " ")

	count, err := strconv.Atoi(parts[0])
	if err != nil {
		return nil, fmt.Errorf("invalid count %s: %w", parts[0], err)
	}

	color := parts[1]
	if color != "red" && color != "green" && color != "blue" {
		return nil, fmt.Errorf("invalid color %s", color)
	}

	return &Cube{Count: count, Color: color}, nil
}
