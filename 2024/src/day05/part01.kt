package day05

import java.io.File
import kotlin.system.exitProcess


fun checkSafe(nums: IntArray, dependencies: HashMap<Int, MutableList<Int>>): Boolean {
    for ((idx, num) in nums.withIndex()) {
        if (!dependencies.containsKey(num)) {
            continue
        }
        for (dep in dependencies[num]!!) {
            if (nums.indexOf(dep) != -1 && nums.indexOf(dep) > idx) {
                return false
            }
        }
    }
    return true
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])
    val text = file.readText()

    val (firstPart, secondPart) = text.split("\n\n")

    val dependencies = HashMap<Int, MutableList<Int>>()

    for (line in firstPart.split("\n")) {
        val (a, b) = line.split("|")
        if (dependencies.containsKey(b.toInt())) {
            dependencies[b.toInt()]?.add(a.toInt())
        } else {
            dependencies[b.toInt()] = mutableListOf(a.toInt())
        }
    }

    var res = 0
    for (line in secondPart.split("\n")) {
        val nums = line.split(",").map { it.toInt() }.toIntArray()
        if (checkSafe(nums, dependencies)) {
            res += nums[nums.size / 2]
        }
    }
    println(res)
}