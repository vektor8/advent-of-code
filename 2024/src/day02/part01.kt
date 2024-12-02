package day02

import java.io.File
import kotlin.math.abs
import kotlin.system.exitProcess


fun isSafe1(nums: List<Int>): Boolean {
    var diff = nums[0] - nums[1]
    if (abs(diff) < 1 || abs(diff) > 3) {
        return false
    }
    for (i in 1 until nums.size - 1) {
        val diff2 = nums[i] - nums[i + 1]
        if (abs(diff2) < 1 || abs(diff2) > 3) {
            return false
        }
        if (diff < 0 && diff2 > 0) return false
        if (diff > 0 && diff2 < 0) return false
        diff = diff2
    }
    return true
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        print("Input file required")
        exitProcess(0)
    }

    val file = File(args[0]);
    val res = file.readLines().sumOf {
        isSafe1(it.split(" ").map { e -> e.trim().toInt() }).toInt()
    }
    println(res)
}