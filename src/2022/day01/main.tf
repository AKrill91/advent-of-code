data "local_file" "day01_input" {
  filename = "${path.module}/../../../resources/2022/day01.txt"
}

locals {
  calorie_groups = split("\n\n", data.local_file.day01_input.content)
  elf_calories_str = [
    for elf in local.calorie_groups:
      compact(split("\n", elf))
  ]

  elf_calories = [
    for elf in local.elf_calories_str:
      [
        for calorie in elf:
          tonumber(calorie)
      ]
  ]

  elf_calorie_sums = [
    for elf in local.elf_calories:
      sum(elf)
  ]
  elf_calorie_sums_padded = [
    for n in local.elf_calorie_sums:
      format("%05d", n)
  ]
  elf_calorie_sums_sorted = [
    for n in reverse(sort(local.elf_calorie_sums_padded)):
      tonumber(n)
  ]
}

output "day01" {
  value = {
    a = max(local.elf_calorie_sums...)
    b = sum(slice(local.elf_calorie_sums_sorted, 0, 3))
  }
}