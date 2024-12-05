package day05

import java.io.File
import kotlin.system.exitProcess


class Graph(private val edges: ArrayList<Edge>) {
    fun removeEdge(from: Int, to: Int): Graph {
        edges.removeIf { it.from == from && it.to == to }
        return this
    }

    fun getNeighbors(node: Int): List<Edge> {
        return edges.filter { it.from == node }
    }

    fun getDependencies(node: Int): List<Edge> {
        return edges.filter { it.to == node }
    }

    fun removeIrrelevantEdges(nodes: IntArray): Graph {
        val newEdges = edges.filter { nodes.contains(it.from) && nodes.contains(it.to) }.toList()
        return Graph(newEdges as ArrayList<Edge>)
    }
}

data class Edge(val from: Int, val to: Int)

fun topologicalSort(
    nums: IntArray,
    graph: Graph
): IntArray {
    val s = ArrayDeque(nums.filter { graph.getDependencies(it).isEmpty() })
    val sorted = ArrayList<Int>()
    while (s.isNotEmpty()) {
        val current = s.removeFirst()
        sorted.add(current)
        for (neigh in graph.getNeighbors(current)) {
            graph.removeEdge(current, neigh.to)
            if (graph.getDependencies(neigh.to).isEmpty()) {
                s.addLast(neigh.to)
            }
        }
    }
    return sorted.toIntArray()
}


fun main(args: Array<String>) {
    if (args.size != 1) {
        println("Input file required")
        exitProcess(0)
    }
    val file = File(args[0])
    val text = file.readText()

    val (firstPart, secondPart) = text.split("\n\n")

    val depsOut = HashMap<Int, MutableList<Int>>()
    val depsIn = HashMap<Int, MutableList<Int>>()
    for (line in firstPart.split("\n")) {

    }
    val graph = Graph(firstPart.split("\n").map {
        val (a, b) = it.split("|")
        Edge(a.toInt(), b.toInt())
    } as ArrayList<Edge>)

    var res = 0
    for (line in secondPart.split("\n")) {
        val nums = line.split(",").map { it.toInt() }.toIntArray()
        val tempGraph = graph.removeIrrelevantEdges(nums)
        val a = topologicalSort(nums, tempGraph)
        if (nums.contentEquals(a)) {
            continue
        }
        res += a[a.size / 2]
    }
    println(res)
}