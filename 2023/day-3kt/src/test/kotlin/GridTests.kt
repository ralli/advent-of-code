import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class HaseTest {
    val input = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...${'$'}.*....
.664.598.."""

    @Test
    fun part1Works() {
        val result = part1(input)
        val expected = 4361
        assertEquals(expected, result)
    }
}
