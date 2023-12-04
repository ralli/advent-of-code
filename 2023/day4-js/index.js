const fs = require("fs")

const filename = "day4.txt"
const input = fs.readFileSync(filename, "utf-8")
const lines = input.split("\n").map(line => line.trim())
const cards = lines.map(line => {
    [cardPart, rest] = line.split(":")
    const re = /Card\s+(\d+)/
    const id = cardPart.match(re)[1]
    const [numberPart, wantedPart] = rest.split('|').map(s => s.trim())
    const numbers = numberPart.split(/\s+/).map(s => +s)
    const wanted = wantedPart.split(/\s+/).map(s => +s)
    const matching = numbers.filter(number => wanted.includes(number))
    const points = matching.length ? Math.pow(2, matching.length - 1) : 0
    return { id: +id, numbers, wanted, matching, numMatches: matching.length, points }
})

// Part 1
const result = cards.map(card => card.points).reduce((a, c) => a + c, 0)
console.log(result)

// Part 2
const memo = cards.map(_ => 1)
cards.forEach((card, idx) => {
    [...Array(card.numMatches).keys()].map(i => idx + i + 1).forEach(i => {
        if (i < memo.length) {
            memo[i] += memo[idx]
        }
    })
})
const result2 = memo.reduce((a, c) => a + c, 0)
console.log(result2)