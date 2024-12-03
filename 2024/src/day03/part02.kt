package day03

import java.io.File
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0]);

    val mulRegex = Regex("mul\\((\\d+),(\\d+)\\)")

    val doRegex = Regex("do\\(\\)")
    val dontRegex = Regex("don't\\(\\)")
    val text = file.readText().replace("\n", "")
    val dos = listOf(-1) + doRegex.findAll(text).map { matchResult -> matchResult.range.first }.toList()
    val donts = listOf(-2) + dontRegex.findAll(text).map { matchResult -> matchResult.range.first }.toList()

    var res = 0
    for (match in mulRegex.findAll(text)) {
        val start = match.range.first
        val closestDo = dos.findLast { e -> e < start }
        val closestDont = donts.findLast { e -> e < start }
        if (closestDo!! > closestDont!!){
            res += match.groupValues[1].toInt() * match.groupValues[2].toInt()
        }
    }
    println(res)
}