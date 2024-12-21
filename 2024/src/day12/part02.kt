package day12

import day10.Grid
import day10.Position
import java.io.File
import java.util.ArrayDeque
import kotlin.system.exitProcess


private fun <T> countEdges(grid: Grid<T>, position: Position): Int {
    val up = position.add(Position(-1, 0))
    val down = position.add(Position(1, 0))
    val left = position.add(Position(0, -1))
    val right = position.add(Position(0, 1))
    val topLeft = position.add(Position(-1, -1))
    val topRight = position.add(Position(-1, 1))
    val downLeft = position.add(Position(1, -1))
    val posVal = grid.get(position)
    fun isInBounds(pos: Position): Boolean {
        return pos.row in 0 until grid.height && pos.col in 0 until grid.width
    }

    var edges = 0;
    fun inRegion(pos: Position): Boolean {
        return isInBounds(pos) && grid.get(pos) == posVal
    }
    if (!inRegion(up)) {
        val sameEdge = inRegion(left) && !inRegion(topLeft);
        if (!sameEdge) {
            edges++
        }
    }

    if (!inRegion(down)) {
        val sameEdge = inRegion(left) && !inRegion(downLeft)
        if (!sameEdge) {
            edges++
        }
    }

    if (!inRegion(left)) {
        val sameEdge = inRegion(up) and !inRegion(topLeft)
        if (!sameEdge) {
            edges++
        }
    }

    if (!inRegion(right)) {
        val sameEdge = inRegion(up) && !inRegion(topRight)
        if (!sameEdge) {
            edges++
        }
    }
    return edges
}

private fun <T> Grid<T>.bfs2(startPos: Position, visited: Array<Boolean>): Pair<Int, Int> {
    val queue = ArrayDeque<Position>();
    queue.offer(startPos);
    var area = 0;
    val region = mutableSetOf<Position>();
    visited[startPos.row * width + startPos.col] = true;
    while (!queue.isEmpty()) {
        val current = queue.remove()
        region.add(current);
        area++
        for (n in getNeighbors(current)) {
            if (get(n) == get(startPos) && !visited[n.row * width + n.col]) {
                queue.offer(n)
                visited[n.row * width + n.col] = true;
            }
        }
    }
    val perimeter = region.sumOf { countEdges(this, it) };
    return Pair(area, perimeter);
}

fun <T> Grid<T>.getIslands2() = iterator {
    val visited = Array(size = height * width, init = { false })
    var pos = Position(0, 0);
    while (!visited.all { it }) {
        val island = bfs2(pos, visited);
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

    val res = grid.getIslands2().asSequence().sumOf { it.first * it.second }
    println(res)
}