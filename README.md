# rlox

A flavor of the Lox interpreter from [Crafting Compilers](https://www.craftinginterpreters.com/) implemented in Rust.

## Deviations
**Things I implemented that don't necessarily fit the vanilla language spec.**

- Error reports will show the line that produced the error with an indicator for which character caused the problem.
    - I'm expecting this to give me trouble when I get to using a VM to execute the code.

## TODO

- Implement comma-separated expression parsing.
    - Only return the result of the right-most expression to the user for that sequence.
        - Unless this is a function argument list.
- Implement bitwise and/or operators.
    - Replace ! / && / || with not/and/or.  Use boolean operations with true/false.  With numbers, error if not integer and use bitwise ops.
- Implement the ternary operator.
    - I expect this will be above the precedence of equality.

- Division by 0 should be Literal::NaN.
- "scone" + 4 == "scone4"
- "a" * 4 = "aaaa"
    - This should error out if not an integer.
- "ab" + cd" = "abcd"

- I really like the idea of function expressions.
