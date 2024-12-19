package day11

import java.io.File
import kotlin.system.exitProcess


fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required");
        exitProcess(0);
    }
    val file = File(args[0]);
    var stones = file.readText().split(' ').map { it.toLong() }.associateWith { 1L };
    for (i in 0 until 75) {
        val newStones = mutableMapOf<Long, Long>();
        stones.forEach { stone ->
            run {
                mapNumber(stone.key).forEach { newStone ->
                    newStones[newStone] = newStones.getOrDefault(newStone, 0) + stone.value;
                }
            }
        }
        stones = newStones;
    }
    val res = stones.values.sum();
    println(res)
}