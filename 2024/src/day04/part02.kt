package day04

import java.io.File
import kotlin.system.exitProcess


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
        for ((j, c) in row.withIndex()) {
            if (c != 'A') {
                continue
            }
            if (i == 0 || j == 0 || i == height - 1 || j == width - 1) {
                continue
            }
            val leftUp = matrix[i - 1][j - 1]
            val rightUp = matrix[i - 1][j + 1]
            val leftDown = matrix[i + 1][j - 1]
            val rightDown = matrix[i + 1][j + 1]
            if (leftUp == 'M' && rightDown == 'S' || leftUp == 'S' && rightDown == 'M') {
                if (rightUp == 'M' && leftDown == 'S' || rightUp == 'S' && leftDown == 'M') {
                    res += 1
                }
            }
        }
    }
    println(res)
}