package day11

import java.io.File
import kotlin.system.exitProcess

fun hasEvenNumberOfDigits(num: Long): Boolean {
    return num.toString().length % 2 == 0;
}


fun mapNumber(num: Long): List<Long> {
    if(num == 0L) return listOf(1);
    if(hasEvenNumberOfDigits(num)){
        val numString = num.toString();
        val num1 = numString.slice(0 until numString.length / 2);
        val num2 = numString.slice(numString.length/2 until numString.length);
        return listOf(num1.toLong(), num2.toLong());
    }
    return listOf(2024 * num);
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required");
        exitProcess(0);
    }
    val file = File(args[0]);
    var vals = file.readText().split(' ').map{it.toLong()};
    for(i in 0..<25){
        vals = vals.map { mapNumber(it) }.flatten();
    }
    println(vals.size)
}