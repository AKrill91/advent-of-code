package com.stexnistudios.adventofcode.year2017.day07;

import com.stexnistudios.adventofcode.Solver;

import java.util.*;

public class Day07Solver extends Solver {

    public Day07Solver(String input) {
        super(input);
    }

    @Override
    public String solveA() {
        return buildTree().getName();
    }

    @Override
    public Integer solveB() {
        Program root = buildTree();

        while (!root.isBalanced()) {
            root = root.getUnbalanced();
        }

        //Everything up the tree is balanced, so our current node is not

        Program parent = root.getParent();
        Collection<Program> siblings = parent.getChildren();

        int goalWeight = -1;

        for (Program sibling : siblings) {
            if(sibling == root) {
                continue;
            }

            goalWeight = sibling.getTotalWeight();
        }

        int rootTotalWeight = root.getTotalWeight();

        return goalWeight - (rootTotalWeight - root.getWeight());
    }

    private Program buildTree() {
        Queue<String> queue = new LinkedList<>(
            Arrays.asList(getInput().split("\n"))
        );

        Map<String, Program> output = new HashMap<>();

        while (!queue.isEmpty()) {
            String line = queue.poll();

            String[] parts = line.split(" ");
            boolean hasChildren = parts.length > 2;
            String name = parts[0];
            int weight = Integer.parseInt(
                parts[1].substring(1, parts[1].length() - 1)
            );

            Program program = new Program(name, weight);

            boolean canProcess = !hasChildren;

            if (hasChildren) {
                List<String> children = new ArrayList<>();
                canProcess = true;
                int numChildren = parts.length - 3;

                for (int i = 0; i < numChildren && canProcess; ++i) {
                    int index = 3 + i;
                    String child = parts[index];
                    child = child.replace(",", "");
                    children.add(child);

                    canProcess = output.containsKey(child);
                }

                if (canProcess) {
                    for (String child : children) {
                        Program c = output.get(child);
                        c.setParent(program);
                        program.addChild(c);
                    }
                }
            }

            if (canProcess) {
                output.put(name, program);
            } else {
                queue.offer(line);
            }
        }

        return output.values().iterator().next().getRoot();
    }
}
