# rlox

A flavor of the Lox interpreter from [Crafting Compilers](https://www.craftinginterpreters.com/) implemented in Rust.

## Features

- Explicit variable declaration.
- Everything is an expression.

## Deviations
*Things I implemented that don't necessarily fit the vanilla language spec.*

- Variables initialize to `nil` if you do not provide a value.
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
- The last statement in any block (or program) need not end with a semicolon.
- String escape sequences for: \n, \r, \t, \", \\
- Variable declarations are allowed anywhere, just like any other statement.
- Expressions can be delimited by commas, which incidentally causes them to function just like semicolons.
- Variables must be defined before they are used, and cannot be defined multiple times.
- Variable assignment is an expression, which means this: `a = b = 10`, will assign `a` and `b` to 10.
    - You can also do this to print and assign `b` at the same time: `print b=10`.
    - You cannot similarly cascade the `let` statement at this time.
- The result of the most recent statement will be stored in the `_` variable.

### If Expressions

- The parenthesis around the condition are not necessary, though due to how expressions are formed you can add them if you really want to.
- They will return the value of the "then" clause if the condition if truthy, otherwise it will return the value of the "else" clause.
    - If no "else" clause is provided in a falsy state, it will return `nil`.
- The ternary operator (?:) available, and desugared into an if-expression at compile time.

## TODO

- Implement bitwise and/or operators.
    - Replace ! / && / || with not/and/or.  Use boolean operations with true/false.  With numbers, error if not integer and use bitwise ops.
- Implement the ternary operator.
    - I expect this will be above the precedence of equality.
    - If the if-statement is an if-expression, is this really necessary?
    - I do like a bit of syntactic sugar.
- A program is essentially a block.  Refactor this later.
- Test synchronization and the ErrorSet.
    - Parsing should continue after an error is found, then a list of errors should be returned to the user.
    - The error indicators are lining up correctly right now?

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
- An `if` expression should return whichever side evaluates to true.  What should a loop return?  Let's say that it returns the value of the final expression evaluated in it's block and see how it goes.
    - If that doesn't work I can always have it return `nil`.
- Should I allow any arbitrary statement in the condition of an `if` or `while`?  Or `print`?  It would make the language more flexible, but might also lead to needless chaos.  Not doing it feels inconsistent with the "everything is an expression" thing though.

