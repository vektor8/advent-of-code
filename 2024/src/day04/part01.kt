package day04

import java.io.File
import kotlin.system.exitProcess

private const val XMAS = "XMAS"
private val DIRECTIONS =
    listOf(Pair(-1, -1), Pair(-1, 0), Pair(-1, 1), Pair(0, -1), Pair(0, 1), Pair(1, -1), Pair(1, 0), Pair(1, 1))


fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])

    val matrix = file.readLines().map { it.toCharArray() }.toTypedArray()
    val height = matrix.size
    val width = matrix[0].size
    var res = 0
    for ((i, row) in matrix.withIndex()) {
        for ((j, _) in row.withIndex()) {
            for (dir in DIRECTIONS) {
                var pos = Pair(i, j)
                var isXmas = true
                for (letter in XMAS) {
                    if (pos.first < 0 || pos.second < 0 || pos.first >= height || pos.second >= width) {
                        isXmas = false
                        break
                    }
                    if (matrix[pos.first][pos.second] != letter) {
                        isXmas = false
                        break
                    }
                    pos = Pair(pos.first + dir.first, pos.second + dir.second)
                }
                if (isXmas) {
                    res++
                }
            }
        }
    }
    println(res)
}