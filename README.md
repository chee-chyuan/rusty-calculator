# Calculator
This is a simple calculator console app written in Rust
To start up the console, <br>
Run at the root of the folder:

```
cargo run
```

The program will exit with 0 and an Answer if the program is successful and exit with 1 with no output when unsuccessful.

## Operator
The calculator has the following operators:

Symbol | Action|
---|---|
+| Addition
-| Subtraction
\* | Multiplication
/ | Division
^ | Exponentiation

Also supports the following math characters:
Symbol | Representation
---|---
π | the number π
pi | the number π
e | Euler's number

Also supports parentheses with correct '(' and ')'

Example of input:
```
2(π+3)^e+7--(5/(3-2))*pi
```

of which will return:
```
Answer:
300.55126786594144

Exit with 0
```