# iron-dice, a CLI die roller for RPGs enthusiasts

## use

irondice DICE

DICE: dice descriptor such as 3d6, 1d20, d10, d100

# to do

- [ ] #23: hardening. find combinations that don't make sense, such as max and success counting
- [ ] #21: handling operator mid with notation "5d6 mid3", meaning "throw 5d6 and yield the sum of 3
  middle dice"
- [ ] #18: accept parameters from cli
- [ ] #17: define differnt types of success while counting successes. 4d10 sc9 sv10:2 fv1:-1 standing
  for: throw 4d10 count success on a 9 or more. On a 10 add 1 more success, on a 1 subtract 1
  success

# doing


# done

- [x] #19: handling operator max with notation "4d6 max3", meaning "throw 4d6 and yield the sum of 3
  higher dice"
- [x] #22: for success counting create a separate field into RollResult to store actual dice results
- [x] #25: refactor code a little better, see test output to improve
- [x] #24: test min/max x of y with a fewer dice number -> 2d6 max3
- [x] #20: handling operator min with notation "4d6 min3", meaning "throw 4d6 and yield the sum of 3
  lower dice"
- [x] #25: test max N with exploding dice
- [x] #24: refactor roller.rs in a module plus multiple files
- [x] #19: draft output. Define a result struct to hold roll result.
- [x] #13: handling exploding dice and counting success with notation "3d6 ex6 sc4" standing for
  "throw 3d6, explode every die showing 6 and count every die showing 4 or more"
- [x] #12: handling exploding dice with notation "3d6 ex5" standing for "throw 3d6, for every die
  showing 5 or more, throw another die. Yield the total sum"
- [x] #16: refactor use of isize and usize
- [x] #11: handling "success" dice with notation "3d6 sc5" standing for "throw 3d6 and count every
  die greater than or equal to 5". sc stands for success count.
- [x] #10: refactor test in 2 categories: parse, roll
- [x] #6: error handling and propagation during parse for Roller
- [x] #9: code cleaning (unwrap, parse)
- [x] #5: support single die notation with modifier such as d20+2, d10-3
- [x] #8: clen the messed up code
- [x] #7: split unit tests
- [x] #4: support negative modifier notation such as 3d6-3
- [x] #3: support modifier notation such as 3d6+2, d20+5
- [x] #2: support single die shorthand notation such as d10, d6
- [x] #1: create a roller object that con be created from a string (DICE) and has a method to roll
  dice. It returns a tuple with a Vector of result plus the final result. IE: 3d6 -> ([4,2,6], 12)

# discarded

- [x] #15: handling checks such as > < <= >=
- [x] (to be refined) #14: handling die expressions...
