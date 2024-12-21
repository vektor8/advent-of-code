package day15

import day10.Grid
import day10.Position
import java.io.File
import java.util.*
import kotlin.system.exitProcess

fun toMove(grid: Grid<Char>, robotPos: Position, direction: Char): List<Position> {
    val s = Stack<Position>()
    val neighbours = mapOf(
        '>' to Position(0, 1),
        '<' to Position(0, -1),
        '^' to Position(-1, 0),
        'v' to Position(1, 0)
    )
    val toMove = mutableListOf<Position>()
    s.push(robotPos)
    val visited = mutableSetOf<Position>()
    while (s.isNotEmpty()) {
        val current = s.pop()
        if (visited.contains(current)) {
            continue
        }
        visited.add(current)
        val move = neighbours[direction]!!
        val next = current.add(move)
        if (grid.get(next) == '#') error("Impossible to move")
        if (grid.get(next) == '[') {
            toMove.add(next)
            s.push(next)
            s.push(Position(next.row, next.col + 1))
        }
        if (grid.get(next) == ']') {
            toMove.add(Position(next.row, next.col - 1))
            s.push(next)
            s.push(Position(next.row, next.col - 1))
        }
    }
    return toMove.toSet().toList()
}

fun move(grid: Grid<Char>, toMove: Collection<Position>, direction: Char) {
    val blocks = when (direction) {
        '>' -> toMove.toList().sortedByDescending { it.col }
        '<' -> toMove.toList().sortedBy { it.col }
        'v' -> toMove.toList().sortedByDescending { it.row }
        '^' -> toMove.toList().sortedBy { it.row }
        else -> {
            error("Unknown direction")
        }
    }
    val neighbours = mapOf(
        '>' to Position(0, 1),
        '<' to Position(0, -1),
        '^' to Position(-1, 0),
        'v' to Position(1, 0)
    )
    for (block in blocks) {
        val new = block.add(neighbours[direction]!!)
        grid.set(block, '.')
        grid.set(block.row, block.col + 1, '.')

        grid.set(new, '[')
        grid.set(new.row, new.col + 1, ']')
    }
}

fun Grid<Char>.resize(): Grid<Char> {
    val newHeight = height
    val newWidth = width * 2
    val newGrid = Array(newHeight) { row ->
        Array(newWidth) { col ->
            val originalCol = col / 2
            when (grid[row][originalCol]) {
                '#' -> '#'
                'O' -> if (col % 2 == 0) '[' else ']'
                '.' -> '.'
                '@' -> if (col % 2 == 0) '@' else '.'
                else -> throw IllegalStateException("Unexpected tile: ${grid[row][originalCol]}")
            }
        }
    }
    return Grid(newGrid)
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val text = File(args[0]).readText()

    val (gridText, movesText) = text.split(Regex("\\n\\s*\\n"))

    val _grid =
        gridText.split("\n").map { it.trim().map { e -> e }.toTypedArray() }
            .toTypedArray()

    val grid = Grid(_grid).resize()

    var robotPos = grid.getStartPoints('@').asSequence().single()

    val moves = movesText.lines().joinToString("")

    val directions = mapOf('^' to Position(-1, 0), '>' to Position(0, 1), 'v' to Position(1, 0), '<' to Position(0, -1))

    val neighbours = mapOf(
        '>' to Position(0, 1),
        '<' to Position(0, -1),
        '^' to Position(-1, 0),
        'v' to Position(1, 0)
    )
    for (move in moves) {
        val nextRobot = robotPos.add(neighbours[move]!!)
        if (grid.get(nextRobot) == '.') {
            grid.set(nextRobot, '@')
            grid.set(robotPos, '.')
            robotPos = nextRobot
        } else {
            try {
                val toMovePos = toMove(grid, robotPos, move)
                move(grid, toMovePos, move)
                grid.set(nextRobot, '@')
                grid.set(robotPos, '.')
                robotPos = nextRobot
            } catch (_: IllegalStateException) {
                continue
            }
        }
    }
    var res = 0
    for (i in 0..<grid.height) {
        for (j in 0..<grid.width) {
            if (grid.get(Position(i, j)) != '[') continue
            res += 100 * i + j
        }
    }
    println(res)
}