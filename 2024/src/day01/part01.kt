package day01

import java.io.File
import kotlin.math.abs
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val inputFile = File(args[0])

    val a = mutableListOf<Int>()
    val b = mutableListOf<Int>()
    for (line in inputFile.readLines()) {
        val (first, second) = line.split("   ").map { it.trim().toInt() }
        a.add(first)
        b.add(second)
    }

    a.sort()
    b.sort()

    val res = a.zip(b).sumOf { abs(it.first - it.second) }
    println(res)
}
