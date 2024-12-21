package day10

import java.io.File
import java.util.*
import kotlin.system.exitProcess


data class Position(val row: Int, val col: Int) {
    fun add(position: Position) = Position(row + position.row, col + position.col)
}

class Grid<T>(
    val grid: Array<Array<T>>,
    private val directions: List<Position> = listOf(Position(-1, 0), Position(1, 0), Position(0, -1), Position(0, 1))
) {
    val height = grid.size
    val width = grid[0].size
    fun getNeighbors(pos: Position) = iterator {
        for (direction in directions) {
            val newPos = pos.add(direction)
            if (isInBounds(newPos)) {
                yield(newPos)
            }
        }
    }

    private fun isInBounds(pos: Position): Boolean {
        return pos.row in 0..<height && pos.col in 0..<width
    }

    fun getStartPoints(startValue: T) = iterator {
        for (i in 0..<height) {
            for (j in 0..<width) {
                if (grid[i][j] == startValue)
                    yield(Position(i, j))
            }
        }
    }

    fun get(pos: Position) = grid[pos.row][pos.col]
}

private fun <T> dfs(
    grid: Grid<T>,
    startPoint: Position,
    neighCondition: (T, T) -> Boolean,
    endCondition: (T) -> Boolean
): Int {
    val stack = Stack<Position>()
    stack.push(startPoint)
    val ends = mutableSetOf<Position>();
    while (stack.isNotEmpty()) {
        val currentPoint = stack.pop()
        if (endCondition(grid.get(currentPoint))) {
            ends.add(currentPoint)
            continue
        }
        for (n in grid.getNeighbors(currentPoint)) {
            if (neighCondition(grid.get(currentPoint), grid.get(n))) {
                stack.add(n)
            }
        }
    }
    return ends.size
}

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        println("Input file required")
        exitProcess(1)
    }
    val file = File(args[0])
    val mat: Array<Array<Int>> = file.readLines().map { it.map { e -> e.digitToInt() }.toTypedArray() }.toTypedArray()

    val grid = Grid(mat)

    val neighCondition: (Int, Int) -> Boolean = { a, b -> b == a + 1 }
    val endCondition = { a: Int -> a == 9 }
    val res = grid.getStartPoints(0).asSequence().sumOf { dfs(grid, it, neighCondition, endCondition) }
    println(res)
}