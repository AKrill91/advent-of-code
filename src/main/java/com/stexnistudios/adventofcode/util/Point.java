package com.stexnistudios.adventofcode.util;

import java.util.Objects;

public class Point {
    public final long x;
    public final long y;

    public Point() {
        this(0, 0);
    }

    public Point(long x, long y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }

    @Override
    public boolean equals(Object obj) {
        boolean eq = false;

        if(obj instanceof Point) {
            Point p = (Point) obj;
            eq = x == p.x && y == p.y;
        }

        return eq;
    }
}
