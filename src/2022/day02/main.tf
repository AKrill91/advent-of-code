data "local_file" "day02_input" {
  filename = "${path.module}/../../../resources/2022/day02.txt"
}

locals {
  score_by_shape = {
    A = 1
    B = 2
    C = 3
    X = 1
    Y = 2
    Z = 3
  }

  score_by_outcome = {
    lose = 0
    X    = 0
    draw = 3
    Y    = 3
    win  = 6
    Z    = 6
  }

  outcome_by_shape = {
    A = {
      X = "draw"
      Y = "win"
      Z = "lose"
    }
    B = {
      X = "lose"
      Y = "draw"
      Z = "win"
    }
    C = {
      X = "win"
      Y = "lose"
      Z = "draw"
    }
  }

  shape_by_outcome = {
    X = {
      A = "C"
      B = "A"
      C = "B"
    }
    Y = {
      A = "A"
      B = "B"
      C = "C"
    }
    Z = {
      A = "B"
      B = "C"
      C = "A"
    }
  }

  rounds_str = compact(split("\n", data.local_file.day02_input.content))
  rounds = [
    for round in local.rounds_str:
      split(" ", round)
  ]
  round_outcomes = [
    for round in local.rounds:
      local.outcome_by_shape[round[0]][round[1]]
  ]

  round_scores = [
    for i, round in local.rounds:
      local.score_by_outcome[local.round_outcomes[i]] + local.score_by_shape[round[1]]
  ]

  alt_scores = [
    for i, round in local.rounds:
      local.score_by_outcome[round[1]] + local.score_by_shape[local.shape_by_outcome[round[1]][round[0]]]
  ]
}

output "day02" {
  value = {
    a = sum(local.round_scores)
    b = sum(local.alt_scores)
  }
}