data "local_file" "day03_input" {
  filename = "${path.module}/../../../resources/2022/day03.txt"
}

locals {
  example_input = <<EOT
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
EOT

#  rucksacks = compact(split("\n", local.example_input))
  rucksacks = compact(split("\n", data.local_file.day03_input.content))

  rucksack_compartments = [
    for rucksack in local.rucksacks:
      [
        substr(rucksack, 0, length(rucksack) / 2),
        substr(rucksack, length(rucksack) / 2, length(rucksack) / 2),
      ]
  ]

  rucksack_compartment_sets = [
    for compartments in local.rucksack_compartments:
      [
        for compartment in compartments:
          toset(split("", compartment))
      ]
  ]

  rucksack_compartment_intersects = [
    for rucksack in local.rucksack_compartment_sets:
      tolist(setintersection(rucksack[0], rucksack[1]))[0]
  ]

  rucksack_compartment_priorities = [
    for intersect in local.rucksack_compartment_intersects:
      parseint(intersect, 62) - 9
  ]

  rucksack_groups = [
    for i in range(length(local.rucksacks) / 3):
      [
        local.rucksacks[i * 3],
        local.rucksacks[i * 3 + 1],
        local.rucksacks[i * 3 + 2]
      ]
  ]

  rucksack_group_intersections = [
    for group in local.rucksack_groups:
      tolist(setintersection(toset(split("", group[0])), toset(split("", group[1])), toset(split("", group[2]))))[0]
  ]

  rucksack_group_priorities = [
    for intersect in local.rucksack_group_intersections:
      parseint(intersect, 62) - 9
  ]
}

output "day03" {
  value = {
    a = sum(local.rucksack_compartment_priorities)
    b = sum(local.rucksack_group_priorities)
  }
}