package day13

import java.io.File
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])

    val res = file.readText().split(Regex("\\n\\s*\\n")).map {
        val(a, b, target) = it.split("\n")
        val xa = a.split("X")[1].split(",")[0].toLong();
        val ya = a.split("Y")[1].trim().toLong()

        val xb = b.split("X")[1].split(",")[0].toLong();
        val yb = b.split("Y")[1].trim().toLong();

        val targetX = target.split(regex = "X=".toRegex()).last().split(",")[0].toLong() + 10000000000000
        val targetY = target.split(regex = "Y=".toRegex()).last().trim().toLong() + 10000000000000
        Pair(Equation(xa, xb, targetX), Equation(ya, yb, targetY))
    }.sumOf {
        val result = solveLinear(it.first, it.second).getOrDefault(Pair(0L, 0L))
        result.first * 3L + result.second
    }

    println(res)
}