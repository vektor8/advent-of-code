package day02

import java.io.File
import kotlin.system.exitProcess


fun isSafe2(nums: List<Int>): Boolean {
    if (isSafe1(nums)) return true
    for (i in nums.indices) {
        val nums2 = nums.subList(0, i) + nums.subList(i + 1, nums.size)
        if (isSafe1(nums2)) return true
    }
    return false
}

fun Boolean.toInt() = if (this) 1 else 0

fun main(args: Array<String>) {
    if (args.size != 1) {
        print("Input file required")
        exitProcess(0)
    }

    val file = File(args[0]);
    val res = file.readLines().sumOf {
        isSafe2(it.split(" ").map { e -> e.trim().toInt() }).toInt()
    }
    println(res)
}