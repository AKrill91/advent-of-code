package com.stexnistudios.adventofcode.year2017.day07;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

public class ProgramTest {

    @Test
    public void getTotalWeightNoChildren() {
        int weight = 4;

        Assert.assertEquals(weight, new Program("foo", weight).getTotalWeight());
    }

    @Test
    public void getTotalWeightSingleChild() {
        Program root = new Program("foo", 4);

        root.addChild(new Program("foo", 6));

        Assert.assertEquals(10, root.getTotalWeight());
    }

    @Test
    public void getTotalWeightChildren() {
        Program root = new Program("foo", 4);

        root.addChild(new Program("foo", 6));
        root.addChild(new Program("foo", 3));

        Assert.assertEquals(13, root.getTotalWeight());
    }

    @Test
    public void isBalancedNoChildren() {
        Assert.assertTrue(new Program("foo", 3).isBalanced());
    }

    @Test
    public void isBalancedSingleChild() {
        Program root = new Program("foo", 3);

        root.addChild(new Program("foo", 2));

        Assert.assertTrue(root.isBalanced());
    }

    @Test
    public void isBalancedWithChildrenTrue() {
        Program root = new Program("foo", 3);

        root.addChild(new Program("foo", 2));
        root.addChild(new Program("foo", 2));

        Assert.assertTrue(root.isBalanced());
    }

    @Test
    public void isBalancedWithChildrenFalse() {
        Program root = new Program("foo", 3);

        root.addChild(new Program("foo", 2));
        root.addChild(new Program("foo", 5));

        Assert.assertFalse(root.isBalanced());
    }

    @Test
    public void getUnbalancedSingleLevel() {
        Program root = new Program("foo", 3);
        root.addChild(new Program("foo", 2));
        root.addChild(new Program("foo", 2));
        Program problem = new Program("foo", 5);
        root.addChild(problem);

        Assert.assertSame(problem, root.getUnbalanced());
    }

    @Test
    public void getUnbalancedMultiLevel() {
        Program left = new Program("foo", 3);
        left.addChild(new Program("foo", 2));
        left.addChild(new Program("foo", 2));
        left.addChild(new Program("foo", 2));

        Program right = new Program("foo", 6);
        right.addChild(new Program("foo", 1));
        right.addChild(new Program("foo", 1));

        Program problem = new Program("foo", 5);
        right.addChild(problem);

        Program root = new Program("foo", 3);
        root.addChild(left);
        root.addChild(right);

        Assert.assertSame(problem, root.getUnbalanced());
    }
}