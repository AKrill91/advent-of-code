package com.stexnistudios.adventofcode.year2017.day09;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

public class Day09SolverTest {

    @Test
    public void solveA() {
        Assert.assertEquals(1,(int) new Day09Solver("{}").solveA());
        Assert.assertEquals(6,(int) new Day09Solver("{{{}}}").solveA());
        Assert.assertEquals(5,(int) new Day09Solver("{{},{}}").solveA());
        Assert.assertEquals(16,(int) new Day09Solver("{{{},{},{{}}}}").solveA());
        Assert.assertEquals(1,(int) new Day09Solver("{<a>,<a>,<a>,<a>}").solveA());
        Assert.assertEquals(9,(int) new Day09Solver("{{<ab>},{<ab>},{<ab>},{<ab>}}").solveA());
        Assert.assertEquals(9,(int) new Day09Solver("{{<!!>},{<!!>},{<!!>},{<!!>}}").solveA());
        Assert.assertEquals(3,(int) new Day09Solver("{{<a!>},{<a!>},{<a!>},{<ab>}}").solveA());
    }

    @Test
    public void solveB() {
        Assert.assertEquals(0,(int) new Day09Solver("{<>}").solveB());
        Assert.assertEquals(17,(int) new Day09Solver("{<random characters>}").solveB());
        Assert.assertEquals(3,(int) new Day09Solver("{<<<<>}").solveB());
        Assert.assertEquals(2,(int) new Day09Solver("{<{!>}>}").solveB());
        Assert.assertEquals(0,(int) new Day09Solver("{<!!>}").solveB());
        Assert.assertEquals(0,(int) new Day09Solver("{<!!!>>}").solveB());
        Assert.assertEquals(10,(int) new Day09Solver("{<{o\"i!a,<{i<a>}").solveB());
    }
}