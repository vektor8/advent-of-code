package day09

import java.io.File
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(1)
    }
    val input = File(args[0]).readText()
    val blocks = mutableListOf<Pair<Int, Int>>()
    var fileId = 0;
    for ((idx, c) in input.withIndex()) {
        val count = c.digitToInt()
        if (idx % 2 == 0) {
            blocks.add(Pair(fileId, count))
            fileId++
        } else {
            blocks.add(Pair(-1, count))
        }

    }
    val files = blocks.filter { it.first != -1 }.reversed().toList()

    for (file in files) {
        val freePos = blocks.indexOfFirst { it.first == -1 && it.second >= file.second }
        val fileIdx = blocks.indexOf(file)
        if (freePos >= fileIdx) {
            continue
        }
        if (freePos == -1) {
            continue
        }
        blocks[fileIdx] = Pair(-1, file.second)
        blocks.add(freePos, file)
        if (blocks[freePos + 1].second == file.second) {
            blocks.removeAt(freePos + 1)
            continue
        }
        blocks[freePos + 1] = Pair(-1, blocks[freePos + 1].second - file.second)
    }
    val l = mutableListOf<Int>()
    for (block in blocks) {
        for (i in 0 until block.second) {
            l.add(block.first)
        }
    }
    val res: Long = l.withIndex()
        .sumOf { if (it.value != -1) (it.index * it.value).toLong() else 0L };
    println(res)
}