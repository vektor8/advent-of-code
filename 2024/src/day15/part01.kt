package day15

import day10.Grid
import day10.Position
import java.io.File
import kotlin.system.exitProcess


fun <T> Grid<T>.getColumn(index: Int) = iterator {
    for (i in 0 until height) {
        yield(Position(i, index))
    }
}

fun <T> Grid<T>.getRow(index: Int) = iterator {
    for (i in 0 until width) {
        yield(Position(index, i))
    }
}

fun <T> Grid<T>.isInBounds(pos: Position): Boolean {
    return pos.row in 0 until height && pos.col in 0 until width;
}

fun <T> Grid<T>.set(pos: Position, value: T) {
    if (!isInBounds(pos)) {
        throw IllegalArgumentException("pos $pos out of range")
    }
    grid[pos.row][pos.col] = value
}

fun <T> Grid<T>.set(row: Int, col: Int, value: T) {
    val pos = Position(row, col)
    set(pos, value)
}

fun <T> Grid<T>.get(row: Int, col: Int): T {
    val pos = Position(row, col)
    return get(pos)
}

fun <T> Grid<T>.print() {
    for (line in grid) {
        for (e in line) {
            print(e)
        }
        println()
    }
}

fun <T> Grid<T>.get(rows: IntRange, cols: IntRange) = iterator {
    for (row in rows) {
        for (col in cols) {
            yield(get(Position(row, col)))
        }
    }
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val text = File(args[0]).readText()

    val (gridText, movesText) = text.split(Regex("\\n\\s*\\n"))

    val _grid =
        gridText.split("\n").drop(1).dropLast(1).map { it.trim().drop(1).dropLast(1).map { e -> e }.toTypedArray() }
            .toTypedArray()

    val grid = Grid(_grid)

    var robotPos = grid.getStartPoints('@').asSequence().first()

    val moves = movesText.lines().joinToString("")

    val directions = mapOf('^' to Position(-1, 0), '>' to Position(0, 1), 'v' to Position(1, 0), '<' to Position(0, -1))

    for (move in moves) {
        val nextPos = directions.getValue(move).add(robotPos)
        if (!grid.isInBounds(nextPos)) {
            continue
        }
        var moved = false;
        when (move) {
            'v' -> {
                val firstWall = grid.getColumn(robotPos.col).asSequence()
                    .indexOfFirst { it.row > robotPos.row && grid.get(it) == '#' }
                val firstEmpty = grid.getColumn(robotPos.col).asSequence()
                    .indexOfFirst { it.row > robotPos.row && grid.get(it) == '.' }
                if (firstEmpty != -1 && (firstWall == -1 || firstWall > firstEmpty)) {
                    moved = true
                    for (r in firstEmpty downTo robotPos.row + 1) {
                        grid.set(r, robotPos.col, grid.get(r - 1, robotPos.col))
                    }
                }
            }

            '^' -> {
                val firstWall = grid.getColumn(robotPos.col).asSequence()
                    .indexOfLast { it.row < robotPos.row && grid.get(it) == '#' }
                val firstEmpty = grid.getColumn(robotPos.col).asSequence()
                    .indexOfLast { it.row < robotPos.row && grid.get(it) == '.' }
                if (firstEmpty != -1 && (firstWall == -1 || firstWall < firstEmpty)) {
                    moved = true
                    for (r in firstEmpty until robotPos.row) {
                        grid.set(r, robotPos.col, grid.get(r + 1, robotPos.col))
                    }
                }
            }

            '>' -> {
                val firstWall = grid.getRow(robotPos.row).asSequence()
                    .indexOfFirst { it.col > robotPos.col && grid.get(it) == '#' }
                val firstEmpty = grid.getRow(robotPos.row).asSequence()
                    .indexOfFirst { it.col > robotPos.col && grid.get(it) == '.' }
                if (firstEmpty != -1 && (firstWall == -1 || firstWall > firstEmpty)) {
                    moved = true
                    for (c in firstEmpty downTo robotPos.col + 1) {
                        grid.set(robotPos.row, c, grid.get(robotPos.row, c - 1))
                    }
                }
            }

            '<' -> {
                val firstWall = grid.getRow(robotPos.row).asSequence()
                    .indexOfLast { it.col < robotPos.col && grid.get(it) == '#' }
                val firstEmpty = grid.getRow(robotPos.row).asSequence()
                    .indexOfLast { it.col < robotPos.col && grid.get(it) == '.' }
                if (firstEmpty != -1 && (firstWall == -1 || firstWall < firstEmpty)) {
                    moved = true
                    for (c in firstEmpty until robotPos.col) {
                        grid.set(robotPos.row, c, grid.get(robotPos.row, c + 1))
                    }
                }
            }
        }
        if (moved) {
            grid.set(robotPos, '.')
            grid.set(nextPos, '@')
            robotPos = nextPos
        }
    }

    var res = 0
    for (i in 0..<grid.height) {
        for (j in 0..<grid.width) {
            if (grid.get(Position(i, j)) != 'O') continue
            res += 100 * (i + 1) + j + 1
        }
    }
    println(res)
}