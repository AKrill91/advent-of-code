package com.stexnistudios.adventofcode.day10;

import com.stexnistudios.adventofcode.Solver;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day10aSolver extends Solver implements BotCompareObserver {
    private final int lowChip;
    private final int highChip;

    private final Map<Integer, Output> outputs;
    private final Map<Integer, Bot> bots;
    private final Map<Integer, Input> inputs;

    private int interestId = -1;

    public Day10aSolver(String input) {
        this(input, 17, 61);
    }

    public Day10aSolver(String input, int value1, int value2) {
        super(input);

        this.lowChip = Math.min(value1, value2);
        this.highChip = Math.max(value1, value2);

        outputs = new HashMap<>();
        bots = new HashMap<>();
        inputs = new HashMap<>();
    }

    @Override
    public Integer call() {
        List<String> instructions = Arrays.asList(getInput().split("\n"));

        Pattern inputPattern = Pattern.compile("value (\\d+) goes to bot (\\d+)");
        Pattern behaviorPattern = Pattern.compile(
            "bot (\\d+) gives low to (bot|output) (\\d+) and high to (bot|output) (\\d+)"
        );

        for (String instruction : instructions) {
            instruction = instruction.trim();
            Matcher inputMatcher = inputPattern.matcher(instruction);

            if (inputMatcher.matches()) {
                int inputId = Integer.parseInt(inputMatcher.group(1));
                int botId = Integer.parseInt(inputMatcher.group(2));

                getInputById(inputId).addConsumer(getBotById(botId));
            } else {
                Matcher behaviorMatcher = behaviorPattern.matcher(instruction);

                if (!behaviorMatcher.matches()) {
                    logger.error("Unknown instruction found: {}", instruction);
                    continue;
                }

                int botId = Integer.parseInt(behaviorMatcher.group(1));
                String lowConsumerType = behaviorMatcher.group(2);
                int lowId = Integer.parseInt(behaviorMatcher.group(3));
                String highConsumerType = behaviorMatcher.group(4);
                int highId = Integer.parseInt(behaviorMatcher.group(5));

                ChipConsumer lowConsumer = getConsumerByTypeAndId(lowConsumerType, lowId);
                ChipConsumer highConsumer = getConsumerByTypeAndId(highConsumerType, highId);

                if (lowConsumer == null || highConsumer == null) {
                    logger.error("Unable to determine consumers for instruction: {}", instruction);
                }

                Bot bot = getBotById(botId);
                bot.setLowConsumer(lowConsumer);
                bot.setHighConsumer(highConsumer);
            }
        }

        logger.info("{} inputs, {} bots, {} outputs", inputs.size(), bots.size(), outputs.size());

        inputs.values().forEach(Input::supply);

        for (Output output : outputs.values()) {
            logger.info("{}: {}", output, output.getChips());
        }

        return interestId;
    }

    private ChipConsumer getConsumerByTypeAndId(String type, int id) {
        ChipConsumer output = null;

        switch (type) {
            case "bot":
                output = getBotById(id);
                break;
            case "output":
                output = getOutputById(id);
                break;
            default:
                logger.warn("Unknown consumer type `{}`", type);
                break;
        }

        return output;
    }

    private Input getInputById(int id) {
        if (!inputs.containsKey(id)) {
            inputs.put(id, new Input(id));
        }

        return inputs.get(id);
    }

    private Bot getBotById(int id) {
        if (!bots.containsKey(id)) {
            bots.put(id, new Bot(id, this));
        }

        return bots.get(id);
    }

    private Output getOutputById(int id) {
        if (!outputs.containsKey(id)) {
            outputs.put(id, new Output(id));
        }

        return outputs.get(id);
    }

    @Override
    public void notify(int id, int lowChip, int highChip) {
        logger.debug("Bot {} is comparing {} and {}", id, lowChip, highChip);
        if (lowChip == this.lowChip && highChip == this.highChip) {
            interestId = id;
        }
    }
}
