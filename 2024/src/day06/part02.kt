package day06

import java.io.File
import kotlin.system.exitProcess

fun Array<CharArray>.println() {
    for (line in this) {
        println(line)
    }
}

fun createsCycle(map: Array<CharArray>, obstaclePos: Position): Boolean {
    var pos = findStart(map)
    var char = map[pos.row][pos.column]
    val orig = map[obstaclePos.row][obstaclePos.column]
    map[obstaclePos.row][obstaclePos.column] = 'O'
    val visited = mutableSetOf<Pair<Position, Char>>()
    while (true) {
        val nextPos = pos.add(DIRECTIONS[char]!!)
        if (nextPos.row < 0 || nextPos.column < 0 || nextPos.row >= map.size || nextPos.column >= map[0].size) {
            map[obstaclePos.row][obstaclePos.column] = orig
            return false
        }
        if (map[nextPos.row][nextPos.column] == '#' || map[nextPos.row][nextPos.column] == 'O') {
            char = TURNS[char]!!
            continue
        }
        if (visited.contains(Pair(nextPos, char))) {
            map[obstaclePos.row][obstaclePos.column] = orig
            return true
        }
        pos = nextPos
        visited.add(Pair(pos, char))
    }
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])

    val map = file.readLines().map { line -> line.toCharArray() }.toTypedArray()
    var pos = findStart(map)
    var char = map[pos.row][pos.column]
    val visited = mutableSetOf<Position>()
    while (true) {
        visited.add(pos)
        val nextPos = pos.add(DIRECTIONS[char]!!)
        if (nextPos.row < 0 || nextPos.column < 0 || nextPos.row >= map.size || nextPos.column >= map[0].size)
            break
        if (map[nextPos.row][nextPos.column] == '#') {
            char = TURNS.getOrDefault(char, '^')
            continue
        }
        pos = nextPos
    }
    var res = 0
    for (p in visited) {
        if (createsCycle(map, p)) {
            res++
        }
    }
    println(res)
}