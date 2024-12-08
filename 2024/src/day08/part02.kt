package day08

import java.io.File
import kotlin.system.exitProcess

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
                antiNodes.add(charPositions[i])
                antiNodes.add(charPositions[j])
                var node = charPositions[i].add(distance)
                while (node.isInBounds(map)) {
                    antiNodes.add(node)
                    node = node.add(distance)
                }
                node = charPositions[j].subtract(distance)
                while (node.isInBounds(map)) {
                    antiNodes.add(node)
                    node = node.subtract(distance)
                }
            }
        }
    }
    println(antiNodes.size)
}