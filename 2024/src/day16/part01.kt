package day16

import day10.Grid
import day10.Position
import day15.isInBounds
import java.io.File
import java.util.*
import kotlin.system.exitProcess

enum class Direction {
    NORTH, SOUTH, EAST, WEST
}

fun <T> Grid<T>.getNeighborsWithDirection(pos: Position) = iterator {
    val directions = listOf(
        Pair(Position(-1, 0), Direction.NORTH),
        Pair(Position(1, 0), Direction.SOUTH),
        Pair(Position(0, -1), Direction.WEST),
        Pair(Position(0, 1), Direction.EAST)
    )
    for (direction in directions) {
        val newPos = pos.add(direction.first)
        if (isInBounds(newPos)) {
            yield(Pair(newPos, direction.second))
        }
    }
}

private data class State(val direction: Direction, val pos: Position, val score: Int)

private fun <T> dijkstra(grid: Grid<T>, startPoint: Position, validPlaces: Collection<T>): Map<Position, Int> {
    val queue =
        PriorityQueue<State>(compareBy { it.score }).apply { add(State(Direction.EAST, startPoint, 0)) }
    val scores = mutableMapOf<Pair<Position, Direction>, Int>().withDefault { Int.MAX_VALUE }
    while (queue.isNotEmpty()) {
        val (direction, pos, score) = queue.remove()
        if (score > scores.getValue(pos to direction)) continue
        scores[pos to direction] = score

        for ((neigh, dir) in grid.getNeighborsWithDirection(pos)) {
            if (direction == Direction.NORTH && dir == Direction.SOUTH) continue
            if (direction == Direction.SOUTH && dir == Direction.NORTH) continue
            if (direction == Direction.EAST && dir == Direction.WEST) continue
            if (direction == Direction.WEST && dir == Direction.EAST) continue

            val cost = if (direction == dir) 1 else 1001
            if (validPlaces.contains(grid.get(neigh))) {
                queue.add(State(pos = neigh, direction = dir, score = score + cost))
            }
        }
    }
    return scores.entries.groupBy { it.key.first }.mapValues { (_, entries) -> entries.minOf { it.value } }
}

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])

    val map = file.readLines().map { line -> line.map { e -> e }.toTypedArray() }.toTypedArray()

    val grid = Grid(map)
    val startPoint = grid.getStartPoints('S').asSequence().single()
    val endPoint = grid.getStartPoints('E').asSequence().single()
    val distances = dijkstra(grid, startPoint, listOf('S', 'E', '.'))
    val res = distances[endPoint]
    println(res)
}