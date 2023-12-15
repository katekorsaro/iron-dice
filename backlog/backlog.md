# iron-dice, a CLI die roller for RPGs enthusiasts

## use

irondice DICE

DICE: dice descriptor such as 3d6, 1d20, d10, d100

# task

- [] #5: support single die notation with modifier such as d20+2, d10-3

- [] #4: support negative modifier notation such as 3d6-3, d6-2

- [x] #3: support modifier notation such as 3d6+2, d20+5

- [x] #2: support single die shorthand notation such as d10, d6

- [x] #1: create a roller object that con be created from a string (DICE) and has a
  method to roll dice. It returns a tuple with a Vector of result plus the final
  result. IE: 3d6 -> ([4,2,6], 12)
