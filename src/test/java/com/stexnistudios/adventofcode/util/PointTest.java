package com.stexnistudios.adventofcode.util;

import org.junit.Assert;
import org.junit.Test;

public class PointTest {

    @Test
    public void distance() {
        Assert.assertEquals(
            Math.sqrt(2),
            Point.distance(Point.ZERO, new Point(1, 1)),
            0.01
        );
        Assert.assertEquals(
            Math.sqrt(5),
            Point.distance(Point.ZERO, new Point(1, 2)),
            0.01
        );
    }

    @Test
    public void manhattanDistance() {
        Assert.assertEquals(
            8,
            Point.manhattanDistance(Point.ZERO, new Point(4, 4))
        );

        Assert.assertEquals(
            5,
            Point.manhattanDistance(Point.ZERO, new Point(3, 2))
        );

        Assert.assertEquals(
            5,
            Point.manhattanDistance(new Point(3, 2), Point.ZERO)
        );

        Assert.assertEquals(
            5,
            Point.manhattanDistance(new Point(3, 2), new Point(1, -1))
        );
    }
}