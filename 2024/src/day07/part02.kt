package day07

import java.io.File
import kotlin.system.exitProcess


fun generateLists(length: Int): List<List<Long>> {
    val numbers = listOf(0, 1, 2)
    val result = mutableListOf<List<Long>>()

    fun helper(currentList: MutableList<Long>) {
        if (currentList.size == length) {
            result.add(currentList.toList())
            return
        }

        for (number in numbers) {
            currentList.add(number.toLong())
            helper(currentList)
            currentList.removeAt(currentList.lastIndex)
        }
    }

    helper(mutableListOf())
    return result
}
fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])

    val equations = file.readLines().map {
        val target = it.split(":")[0].toLong()
        val vals = it.split(":")[1].split(" ").drop(1).map { it.toLong() }.toLongArray()
        target to vals
    }

    val res = equations.sumOf {
        val (target, vals) = it
        var goodPossibilities = 0
        for (possibility in generateLists(vals.size)) {
            var sum = 0L
            for ((num, op) in vals.zip(possibility)) {
                when (op) {
                    0L -> sum += num
                    1L -> sum *= num
                    2L -> sum = (sum.toString() + num.toString()).toLong()
                }
                if (sum > target) {
                    break
                }
            }
            if (sum == target) {
                goodPossibilities++
            }
        }
        if (goodPossibilities > 0L) {
            target
        } else {
            0
        }
    }
    println(res)
}