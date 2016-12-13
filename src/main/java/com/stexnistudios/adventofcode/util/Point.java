package com.stexnistudios.adventofcode.util;

import java.util.Objects;

public class Point {

    public static final Point ZERO = new Point(0, 0);

    private final long x;
    private final long y;

    public Point() {
        this(0, 0);
    }

    public Point(long x, long y) {
        this.x = x;
        this.y = y;
    }

    public long getX() {
        return x;
    }

    public long getY() {
        return y;
    }

    public Point add(Point other) {
        return new Point(getX() + other.getX(), getY() + other.getY());
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        Point point = (Point) o;
        return getX() == point.getX() &&
            getY() == point.getY();
    }

    @Override
    public int hashCode() {
        return Objects.hash(getX(), getY());
    }

    @Override
    public String toString() {
        return "Point{" + "x=" + getX() + ", y=" + getY() + '}';
    }
}
