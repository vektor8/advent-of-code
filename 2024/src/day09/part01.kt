package day09

import java.io.File
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(1)
    }
    val input = File(args[0]).readText()
    val blocks = mutableListOf<Int>()
    var fileId = 0;
    for ((idx, c) in input.withIndex()) {
        val count = c.digitToInt()
        for (i in 0 until count) {
            blocks.add(if (idx % 2 == 0) fileId else -1)
        }
        if (idx % 2 == 0) fileId++

    }

    val freeIndexes = blocks.withIndex().filter { it.value == -1 }.map { it.index };

    for (free in freeIndexes) {
        val last = blocks.indexOfLast { it != -1 }
        if (last <= free) {
            break
        }
        blocks[free] = blocks[last]
        blocks[last] = -1
    }

    val res: Long = blocks.withIndex()
        .sumOf { if (it.value != -1) (it.index * it.value).toLong() else 0L };
    println(res)
}