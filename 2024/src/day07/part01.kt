package day07

import java.io.File
import kotlin.math.pow
import kotlin.system.exitProcess

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
        val possibilities = 2.0.pow(vals.size.toDouble()).toLong()
        for (i in 0..<possibilities) {
            val useMultiply = i.toString(2).padStart(vals.size, '0').map { it.digitToInt().toLong() }.toLongArray()
            var sum = 0L
            for ((num, op) in vals.zip(useMultiply)) {
                when (op) {
                    0L -> sum += num
                    1L -> sum *= num
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