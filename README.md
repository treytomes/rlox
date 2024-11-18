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
- Truthiness:
    - NaN is not truthy.
    - Empty strings are not truthy.
    - Any number != 0 is truthy.
- Equalness:
    - NaN is not equal to anything.
- The result of the last expression will be automatically returned to the user.
- The last statement need not end with a semicolon.
- String escape sequences for: \n, \r, \t, \", \\
- Variable declarations are allowed anywhere, just like any other statement.
- Expressions can be delimited by commas, which incidentally causes them to function just like semicolons.

## TODO

- Implement bitwise and/or operators.
    - Replace ! / && / || with not/and/or.  Use boolean operations with true/false.  With numbers, error if not integer and use bitwise ops.
- Implement the ternary operator.
    - I expect this will be above the precedence of equality.
    - If the if-statement is an if-expression, is this really necessary?
    - I do like a bit of syntactic sugar.
- A program is essentially a block.  Refactor this later.

## Musings

- I really like the idea of function expressions.
    - `var my_func = fn(a, b, c, d) print a, b, c, d`
    - `var my_func = (a, b, c, d) => print a, b, c, d`
    - `my_func = (a, b, c, d) => { print a; print b; return c; }`
    - `my_func = (a, b) => a + b`
    - `let my_func(a, b) => a + b`
    - `let my_func(a, b) a + b`
    - `let my_func(a, b) { return a + b }`
    - There's a lot of ways to do this.
- I might use `let` instead of `var`.  `var` carries some bad JS vibes.
- I think everything should be an "expression" of some sort.
- I also like everything being an object.  I want to be able to do things like `(a > b).ifTrue(...).else(...)`.
- The `print` and `let` expressions both return `nil`.  It kinda makes sense with print, as print has an effect on stdout,
  but doesn't change the environment.  But what about `let`?  It adds a variable.  Should it return anything?
  e.g. `while (let v = 10) > 0 v--;`
  I suppose the `let` statement could return a reference to that variable?  But somehow only initialize the variable once?
  That would get weird if the while-loop was structured like this:
  ```
  while (true) {
    let v = 10;
    print v * 20;
  }
  ```
  I would expect `v` to be reinitialized on every iteration.
  In both cases the initialize of `v` is embedded in a larger expression.
  I think I'm going to say that `let` will only initialize the variable if it's not already initialize, and otherwise quietly do nothing.
- Should I create a `del` to remove variables?
- Do I want to allow function arguments to be separated by semicolons?  It feels weird, but is also internally consistent?
  Mostly it just feels weird.
