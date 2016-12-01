package com.stexnistudios.adventofcode.util;

import java.util.Objects;

public class Point {
    private final int x;
    private final int y;

    public Point() {
        this(0, 0);
    }

    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return x;
    }

    public int getY() {
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
