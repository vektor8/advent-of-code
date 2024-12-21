package day14

import java.io.File
import kotlin.system.exitProcess

data class Position(val x: Int, val y: Int);
data class Robot(val pos: Position, val velocity: Position);


fun myMod(x: Int, y: Int): Int {
    if (x >= 0) {
        return x % y;
    }
    var tempX = x;
    while (tempX < 0) {
        tempX += y
    }
    return tempX % y;
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }

    val robots = File(args[0]).readLines().map { line ->
        val (p, v) = line.split(" ")
        val (posX, posY) = p.split("=")[1].split(",").map { it.toInt() };
        val (velX, velY) = v.split("=")[1].split(",").map { it.toInt() };
        Robot(Position(posX, posY), Position(velX, velY))
    }

    val ITER = 100
    val WIDTH = 101
    val HEIGHT = 103
    val finalPositions = robots.map { r ->
        Position(myMod(r.pos.x + r.velocity.x * ITER, WIDTH), myMod(r.pos.y + r.velocity.y * ITER, HEIGHT))
    }

    val quadrants =
        mutableMapOf(Pair(false, false) to 0, Pair(true, false) to 0, Pair(false, true) to 0, Pair(true, true) to 0)
    for (robot in finalPositions) {
        if (robot.x == WIDTH / 2 || robot.y == HEIGHT / 2) continue
        quadrants[Pair(robot.x < WIDTH / 2, robot.y < HEIGHT / 2)] =
            quadrants[Pair(robot.x < WIDTH / 2, robot.y < HEIGHT / 2)]!! + 1
    }

    val res = quadrants.values.reduce { a, b -> a * b };
    println(res)
}