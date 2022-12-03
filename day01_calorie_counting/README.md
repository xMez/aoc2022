# Calorie counting

## Problem

The jungle must be to overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of **Calories** each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

```text
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```

This list represents the Calories of the food carried by five Elves:

- The first Elf is carrying food with `1000`, `2000`, and `3000` Calories, a total of `6000` Calories.
- The second Elf is carrying one food item with `4000` Calories.
- The third Elf is carrying food with `5000` and `6000` Calories, a total of `11000` Calories.
- The fourth Elf is carrying food with `7000`, `8000`, and `9000` Calories, a total of `24000` Calories.
- The fifth Elf is carrying one food item with `10000` Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the **most** Calories. In the example above, this is `24000` (carried by the fourth Elf).

Find the Elf carrying the most Calories. **How many total Calories is that Elf carrying?**

## Solution

The rust implementation takes an input file `--file` and output file `--out`.

### Part 1

First we read the input and split it at every `\n` character with the `split_terminator` function in rust, this gives us empty strings where there are two `\n`. We use this when we map each number to `u32` and set every missing value to `0`.

Second we split the resulting vector at each `0` value and sum each Elf's Calories by mapping a sum on the iterator of each inner vector we get from the split.

Thirdly we get the index of the Elf who is carrying the most Calories by adding an enumerator to the iterator when we compare the values using `max_by` so we can select which we want to compare. To then get the index for the Elf we apply a `map` to get the index value from the result of `max_by`, as this is an `Optional` we have to use `unwrap` to get the value.

The number of the Elf and how many Calories they are carrying gets written to the output file.
