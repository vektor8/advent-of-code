package day12

import day10.Grid
import day10.Position
import java.io.File
import java.util.*
import kotlin.system.exitProcess

private fun <T> Grid<T>.bfs(startPos: Position, visited: Array<Boolean>): Pair<Int, Int> {
    val queue = ArrayDeque<Position>();
    queue.offer(startPos);
    var area = 0;
    var perimeter = 0;
    visited[startPos.row * width + startPos.col] = true;
    while (!queue.isEmpty()) {
        val current = queue.remove()
        area++
        var posPerimeter = 4;
        for (n in getNeighbors(current)) {
            if (get(n) == get(startPos)) {
                if (!visited[n.row * width + n.col]) {
                    queue.offer(n)
                    visited[n.row * width + n.col] = true;
                }
                posPerimeter--;
            }
        }
        perimeter += posPerimeter
    }
    return Pair(area, perimeter);
}

fun <T> Grid<T>.getIslands() = iterator {
    val visited = Array(size = height * width, init = { false })
    var pos = Position(0, 0);
    while (!visited.all { it }) {
        val island = bfs(pos, visited);
        yield(island)
        for (i in 0 until height) {
            for (j in 0 until width) {
                if (!visited[i * width + j]) {
                    pos = Position(i, j)
                    break
                }
            }
        }
    }
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0);
    }
    val file = File(args[0]);

    val mat = file.readLines().map { it.map { c -> c }.toTypedArray() }.toTypedArray()

    val grid = Grid(mat);

    val res = grid.getIslands().asSequence().sumOf { it.first * it.second }
    println(res)
}