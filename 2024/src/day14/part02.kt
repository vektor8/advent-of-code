package day14

import java.io.File
import kotlin.system.exitProcess

fun findLongestHorizontalLine(positions: Collection<Position>): List<Position> {
    val positionSet = positions.toSet()

    var longestLine = emptyList<Position>()

    for (position in positionSet) {
        if (Position(position.x, position.y - 1) !in positionSet) {
            val line = mutableListOf<Position>()
            var current = position

            while (current in positionSet) {
                line.add(current)
                current = Position(current.x, current.y + 1)
            }

            if (line.size > longestLine.size) {
                longestLine = line
            }
        }
    }

    return longestLine
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }

    val robots = File(args[0]).readLines().map { line ->
        val (p, v) = line.split(" ")
        val (posX, posY) = p.split("=")[1].split(",").map { it.toInt() }
        val (velX, velY) = v.split("=")[1].split(",").map { it.toInt() }
        Robot(Position(posX, posY), Position(velX, velY))
    }

    val ITER = 100
    val WIDTH = 101
    val HEIGHT = 103
    val directions = listOf(Position(-1, 0), (Position(0, 1)), Position(0, -1), Position(1, 0))

    var iter = 0

    val outFile = File("output.txt")

    while (true) {
        val positions = robots.map { r ->
            Position(myMod(r.pos.x + r.velocity.x * iter, WIDTH), myMod(r.pos.y + r.velocity.y * iter, HEIGHT))
        }
        iter++
        if (findLongestHorizontalLine(positions).size > 10) {
            break
        }
    }

    iter--
    println(iter)
}