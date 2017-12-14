package com.stexnistudios.adventofcode.year2017.day07;

import java.util.*;

public class Program {
    private final String name;
    private final int weight;
    private Collection<Program> children;
    private Program parent;

    public Program(String name, int weight) {
        this.name = name;
        this.weight = weight;
        this.children = new ArrayList<>();
    }

    public void setParent(Program parent) {
        this.parent = parent;
    }

    public void addChild(Program child) {
        this.children.add(child);
    }

    public String getName() {
        return name;
    }

    public int getWeight() {
        return weight;
    }

    public int getTotalWeight() {
        return weight + children.stream()
            .mapToInt(Program::getTotalWeight)
            .sum();
    }

    public Program getUnbalanced() {
        Program output = null;

        if (children.size() > 1) {
            Iterator<Program> childIter = children.iterator();

            while (output == null && childIter.hasNext()) {
                Program next = childIter.next();

                Program unbal = next.getUnbalanced();
                if (unbal != null) {
                    output = unbal;
                }
            }

            if (output == null) {
                Map<Integer, List<Program>> perWeightChildren = new HashMap<>();

                for (Program child : children) {
                    int weight = child.getTotalWeight();
                    if (!perWeightChildren.containsKey(weight)) {
                        perWeightChildren.put(weight, new ArrayList<>());
                    }

                    perWeightChildren.get(weight).add(child);
                }

                for (List<Program> programs : perWeightChildren.values()) {
                    if (programs.size() == 1) {
                        output = programs.get(0);
                        break;
                    }
                }
            }
        }

        return output;
    }

    public boolean isBalanced() {
        return getUnbalanced() == null;
    }

    public Collection<Program> getChildren() {
        return children;
    }

    public Program getParent() {
        return parent;
    }

    public Program getRoot() {
        Program output = this;

        if (parent != null) {
            output = parent.getRoot();
        }

        return output;
    }
}
