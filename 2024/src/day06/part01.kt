package day06

import java.io.File
import kotlin.system.exitProcess

data class Position(val row: Int, val column: Int) {
    fun add(position: Position) = Position(row + position.row, column + position.column)
}


fun findStart(map: Array<CharArray>): Position {
    for ((i, line) in map.withIndex()) {
        for ((j, c) in line.withIndex()) {
            if (c == '^')
                return Position(i, j)
        }
    }
    throw IllegalArgumentException("No start position found")
}

val DIRECTIONS = mapOf('^' to Position(-1, 0), '>' to Position(0, 1), 'v' to Position(1, 0), '<' to Position(0, -1))

val TURNS = mapOf('^' to '>', '>' to 'v', 'v' to '<', '<' to '^')
fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])

    val map = file.readLines().map { line -> line.toCharArray() }.toTypedArray()
    var pos = findStart(map)
    var char = map[pos.row][pos.column]
    val visited = mutableSetOf<Position>()
    while (true) {
        visited.add(pos)
        var nextPos = pos.add(DIRECTIONS[char]!!)
        if (nextPos.row < 0 || nextPos.column < 0 || nextPos.row >= map.size || nextPos.column >= map[0].size)
            break
        if (map[nextPos.row][nextPos.column] == '#') {
            char = TURNS.getOrDefault(char, '^')
            nextPos = pos.add(DIRECTIONS[char]!!)
        }
        pos = nextPos
    }
    println(visited.size)
}