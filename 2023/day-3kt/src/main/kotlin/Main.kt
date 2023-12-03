import java.io.File

fun main(args: Array<String>) {
    val filename = "day-3.txt"
    val input = File(filename).readText()
    val result1 = part1(input)
    println("$result1")
    val result2 = part2(input)
    println("$result2")
}

fun part1(input: String): Int {
    val lines = input.lines().filter { it.isNotBlank() }.map { it.toCharArray() }
    val grid = Grid(lines)
    val numbers = mutableListOf<Int>()
    for (row in 0..<grid.height()) {
        for (col in 0..<grid.width()) {
            val n = grid.scanNumber(row, col)
            n?.let { numbers.add(it) }
        }
    }
    return numbers.sum()
}

fun part2(input: String): Int {
    val lines = input.lines().filter { it.isNotBlank() }.map { it.toCharArray() }
    val grid = Grid(lines)
    val numbers = mutableListOf<NumberAndPositions>()
    for (row in 0..<grid.height()) {
        for (col in 0..<grid.width()) {
            val n = grid.scanNumberAndPositions(row, col)
            n?.let { numbers.add(it) }
        }
    }
    val starsAndNumbers = mutableMapOf<Position, MutableList<Int>>()
    for (np in numbers) {
        for (pos in np.positions) {
            starsAndNumbers.putIfAbsent(pos, mutableListOf<Int>(np.number))?.add(np.number)
        }
    }
    return starsAndNumbers.filter { it.value.size == 2 }.map { it.value[0] * it.value[1] }.sum()
}


val deltas =
    listOf(
        Pair(-1, -1), Pair(-1, 0), Pair(-1, 1),
        Pair(0, -1), Pair(0, 1),
        Pair(1, -1), Pair(1, 0), Pair(1, 1)
    )

data class Grid(val fields: List<CharArray>) {

    fun scanNumber(row: Int, col: Int): Int? {
        if (!isStartOfNumber(row, col)) {
            return null
        }
        var c = col
        var result = 0
        var symbolFound = false

        while (isDigit(row, c)) {
            result *= 10
            result += get(row, c).code - '0'.code
            symbolFound = symbolFound || hasAdjacentSymbols(row, c)
            ++c
        }

        return if (symbolFound) result else null
    }

    fun scanNumberAndPositions(row: Int, col: Int): NumberAndPositions? {
        if (!isStartOfNumber(row, col)) {
            return null
        }
        var c = col
        var result = 0
        val positions = mutableSetOf<Position>()

        while (isDigit(row, c)) {
            result *= 10
            result += get(row, c).code - '0'.code
            positions.addAll(adjacentStars(row, c))
            ++c
        }

        return if (positions.isEmpty()) null else NumberAndPositions(result, positions)
    }


    private fun hasAdjacentSymbols(row: Int, col: Int): Boolean {
        return deltas.any { isSymbol(row + it.first, col + it.second) }
    }

    private fun adjacentStars(row: Int, col: Int): Set<Position> {
        return deltas.map { Position(it.first + row, it.second + col) }.filter { isStar(it.row, it.col) }.toSet()
    }

    private fun isStartOfNumber(row: Int, col: Int): Boolean {
        return isDigit(row, col) && !isDigit(row, col - 1)
    }

    private fun isDigit(row: Int, col: Int): Boolean {
        return get(row, col).isDigit()
    }

    private fun isSymbol(row: Int, col: Int): Boolean {
        val c = get(row, col)
        return !(c == '.' || c.isDigit())
    }

    private fun isStar(row: Int, col: Int): Boolean {
        val c = get(row, col)
        return !(c == '.' || c.isDigit())
    }

    fun width(): Int {
        return fields.firstOrNull()?.size ?: 0
    }

    fun height(): Int {
        return fields.size
    }

    fun get(row: Int, col: Int): Char {
        if (row < 0 || row >= height() || col < 0 || col >= width()) {
            return '.'
        }
        return fields[row][col]
    }
}

data class Position(val row: Int, val col: Int)
data class NumberAndPositions(val number: Int, val positions: Set<Position>)
