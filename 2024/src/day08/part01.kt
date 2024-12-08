package day08

import java.io.File
import kotlin.system.exitProcess


fun getUniqueCharacters(map: Array<CharArray>): Set<Char> {
    return map.fold(mutableSetOf()) { acc, charArray ->
        acc.apply { addAll(charArray.toSet()) }
    }
}


fun getAllPositionsOf(map: Array<CharArray>, char: Char): List<Position> {
    val charPositions = mutableListOf<Position>()
    for ((row, line) in map.withIndex()) {
        for ((col, c) in line.withIndex()) {
            if (c == char) {
                charPositions.add(Position(row, col))
            }
        }
    }
    return charPositions
}

data class Position(val row: Int, val col: Int) {
    fun distance(other: Position): Position {
        return Position(row - other.row, col - other.col)
    }

    fun add(other: Position): Position {
        return Position(row + other.row, col + other.col)
    }
    private fun invert(): Position{
        return Position(-row, -col)
    }

    fun subtract(other: Position): Position{
        return this.add(other.invert())
    }
    fun isInBounds(map: Array<CharArray>): Boolean {
        return row >= 0 && row < map.size && col >= 0 && col < map[0].size
    }
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(1)
    }
    val file = File(args[0])

    val map = file.readLines().map { line -> line.toCharArray() }.toTypedArray()
    val antiNodes = mutableSetOf<Position>()
    for (char in getUniqueCharacters(map)) {
        if (char == '.') {
            continue
        }
        val charPositions = getAllPositionsOf(map, char)
        for (i in 0 until charPositions.size - 1) {
            for (j in i + 1 until charPositions.size) {
                val distance = charPositions[i].distance(charPositions[j])
                val firstNode = charPositions[i].add(distance)
                val secondNode = charPositions[j].subtract(distance)
                if(firstNode.isInBounds(map)){
                    antiNodes.add(firstNode)
                }
                if(secondNode.isInBounds(map)){
                    antiNodes.add(secondNode)
                }
            }
        }
    }
    println(antiNodes.size)
}