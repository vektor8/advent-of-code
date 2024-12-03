package day03

import java.io.File
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0]);

    val mulRegex = Regex("mul\\((\\d+),(\\d+)\\)")

    val res = file.readLines()
        .sumOf { mulRegex.findAll(it).map { m -> m.groupValues[1].toInt() * m.groupValues[2].toInt() }.sum() }

    println(res)
}