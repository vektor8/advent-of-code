package day10

import java.io.File
import java.util.*
import kotlin.system.exitProcess


private fun <T> dfs(
    grid: Grid<T>,
    startPoint: Position,
    neighCondition: (T, T) -> Boolean,
    endCondition: (T) -> Boolean
): Int {
    val stack = Stack<Position>()
    stack.push(startPoint)
    val parents = mutableMapOf<Position, Position>()
    val differentPaths = mutableSetOf<List<Position>>()
    while (stack.isNotEmpty()) {
        val currentPoint = stack.pop()
        if (endCondition(grid.get(currentPoint))) {
            var current = currentPoint
            val path = mutableListOf<Position>(current)
            while (current != startPoint) {
                val next = parents[current]
                if (next != null) {
                    path.add(next)
                }
                current = next
            }
            differentPaths.add(path)
        }
        for (n in grid.getNeighbors(currentPoint)) {
            if (neighCondition(grid.get(currentPoint), grid.get(n))) {
                stack.add(n)
                parents[n] = currentPoint
            }
        }
    }
    return differentPaths.size
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