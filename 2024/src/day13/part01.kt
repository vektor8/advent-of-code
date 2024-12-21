package day13

import java.io.File
import kotlin.system.exitProcess


data class Equation(val a: Long, val b: Long, val c: Long)

fun solveLinear(eq1: Equation, eq2: Equation): Result<Pair<Long, Long>> {
    val y = (eq2.c * eq1.a - eq1.c * eq2.a) / (eq2.b * eq1.a - eq1.b * eq2.a)
    val x = (eq1.c - eq1.b * y) / eq1.a

    if (eq1.a * x + eq1.b * y != eq1.c || eq2.a * x + eq2.b * y != eq2.c) {
        return Result.failure(IllegalStateException("No solution for system"))
    }
    return Result.success(Pair(x, y))
}

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

        val targetX = target.split(regex = "X=".toRegex()).last().split(",")[0].toLong()
        val targetY = target.split(regex = "Y=".toRegex()).last().trim().toLong();
        Pair(Equation(xa, xb, targetX), Equation(ya, yb, targetY))
    }.sumOf {
        val result = solveLinear(it.first, it.second).getOrDefault(Pair(0L, 0L))
        result.first * 3L + result.second
    }

    println(res)
}
