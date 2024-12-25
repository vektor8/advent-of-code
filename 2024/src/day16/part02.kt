package day16

import day10.Grid
import day10.Position
import day15.print
import day15.set
import java.io.File
import java.util.*
import kotlin.system.exitProcess


private data class State2(val direction: Direction, val pos: Position, val score: Int, val path: List<Position>)

private fun <T> dijkstra(grid: Grid<T>, startPoint: Position, validPlaces: Collection<T>, endValue: T): Int {
    val queue =
        PriorityQueue<State2>(compareBy { it.score }).apply { add(State2(Direction.EAST, startPoint, 0, listOf())) }

    val scores = mutableMapOf<Pair<Position, Direction>, Int>().withDefault { Int.MAX_VALUE }
    var shortest = Int.MAX_VALUE
    var bestPaths = mutableListOf<List<Position>>()
    while (queue.isNotEmpty()) {
        val (direction, pos, score, path) = queue.remove()
        if (score > scores.getValue(pos to direction)) continue
        scores[pos to direction] = score

        if (grid.get(pos) == endValue) {
            if (score < shortest) {
                shortest = score
                bestPaths = listOf(path).toMutableList()
            }
            if (score == shortest) {
                bestPaths.add(path)
            }
            continue
        }
        for ((neigh, dir) in grid.getNeighborsWithDirection(pos)) {
            if (direction == Direction.NORTH && dir == Direction.SOUTH) continue
            if (direction == Direction.SOUTH && dir == Direction.NORTH) continue
            if (direction == Direction.EAST && dir == Direction.WEST) continue
            if (direction == Direction.WEST && dir == Direction.EAST) continue

            val cost = if (direction == dir) 1 else 1001
            if (validPlaces.contains(grid.get(neigh))) {
                queue.add(State2(pos = neigh, direction = dir, score = score + cost, path = path + neigh))
            }
        }
    }
    return bestPaths.flatten().toSet().size + 1
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
    val res = dijkstra(grid, startPoint, listOf('S', 'E', '.'), 'E')
    println(res)
}