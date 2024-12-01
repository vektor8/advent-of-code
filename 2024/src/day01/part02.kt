package day01

import java.io.File
import kotlin.system.exitProcess


fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val inputFile = File(args[0])

    val freqA = mutableMapOf<Int, Int>()
    val freqB = mutableMapOf<Int, Int>()
    for (line in inputFile.readLines()) {
        val (first, second) = line.split("   ").map { it.trim().toInt() }
        freqA[first] = freqA.getOrDefault(first, 0) + 1
        freqB[second] = freqB.getOrDefault(second, 0) + 1
    }

    val res = freqA.map { it.key * it.value * freqB.getOrDefault(it.key, 0) }.sum()
    println(res);
}
