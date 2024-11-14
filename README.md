# rlox

A flavor of the Lox interpreter from [Crafting Compilers](https://www.craftinginterpreters.com/) implemented in Rust.

## Deviations
*Things I implemented that don't necessarily fit the vanilla language spec.*

- Error reports will show the line that produced the error with an indicator for which character caused the problem.
    - I'm expecting this to give me trouble when I get to using a VM to execute the code.
- Dividing by 0 yields the `NaN` literal, which is definitely not a number.
- Adding strings together concatenates the strings.
- Adding a number to a string converts the number to a string before concatenating.
- Multiplying a string by an integer will concatenate the string with itself a number of times.

## TODO

- Implement comma-separated expression parsing.
    - Only return the result of the right-most expression to the user for that sequence.
        - Unless this is a function argument list.
- Implement bitwise and/or operators.
    - Replace ! / && / || with not/and/or.  Use boolean operations with true/false.  With numbers, error if not integer and use bitwise ops.
- Implement the ternary operator.
    - I expect this will be above the precedence of equality.

## Musings

- I really like the idea of function expressions.
    - `var my_func = fn(a, b, c, d) print a, b, c, d`
    - `var my_func = (a, b, c, d) => print a, b, c, d`
    - `my_func = (a, b, c, d) => { print a; print b; return c; }`
    - `my_func = (a, b) => a + b`
    - There's a lot of ways to do this.
- I might use `let` instead of `var`.  `var` carries some bad JS vibes.
- I think everything should be an "expression" of some sort.
