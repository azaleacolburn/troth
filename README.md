# troth

A simple Lambda-Calculus parser

## Features

- Lambda Expressions
- Expression Aliasing
- Expression Reduction

## Syntax

### Tokens

```
ALIAS: [A-Z0-9_$&+,:=?@#|'<>.-^\*%!]+
ID: [a-z_]+
OPAREN: \(
CPAREN: \)
DOT: \.
LAMBDA: l
DEFINITION: fn
SEMI: ;
```

### Grammar Rules

```
definiton: DEFINITION ALIAS expression SEMI
expression: ALIAS | ID | call
call: OPAREN expression expression CPAREN
lambda: LAMBDA ID DOT expression
```

## Examples

| Lambda Calculus Definition       | Rust Usage |
| -------------------------------- | ---------- |
| `fn T lx.ly.x`                   | `true`     |
| `fn F lx.ly.y`                   | `false`    |
| `fn ! lx.((x F) T)`              | `!x`       |
| `fn & lx.ly.((x y) x)`           | `x & y`    |
| `fn \| lx.ly.((x x) y)`          | `x \| y`   |
| `fn SUCC lx.ln.lm.(n ((x n) m))` | `x + 1`    |
| `fn 0 ls.lz.z`                   | `0`        |
| `fn 1 (SUCC 0)`                  | `1`        |
| `fn + lx.ly.(x (SUCC y))`        | `x + y`    |
| `fn * lx.ly.lz.(x (y z))`        | `x * y`    |
| `fn <=0 lx.(((x F) N) F)`        | `x <= 0`   |
| `fn >= lx.ly.(Z ((x P) y))`      | `x >= y`   |
| `fn >=1 lx.(((x F) N) T)`        | `x >= 1`   |

> [!NOTE]
> This grammar does support numerical and 'special' characters in (and even as the first character of) aliases, meaning full high-level languages could be defined with normal arithmatic

## TODO

- [x] Lexer
- [x] Parser
- [x] Reducer
- [ ] Automatic Alpha Converter
