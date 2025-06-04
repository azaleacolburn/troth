# troth

A simple λ-Calculus interpreter

## Features

- λ-Abstractions
- Expression Aliasing
- Expression Reduction

## Syntax

### Tokens

```
ALIAS: [A-Z0-9_$&+,:=?@#|'<>.-^\*%!]+
ID: [a-km-z_]+
OPAREN: \(
CPAREN: \)
DOT: \.
LAMBDA: l
DEFINITION: fn
USE: use "[a-km-z_\\]+"
SEMI: ;
```

> [!NOTE]
> Comments are denoted by `//` and run until the end of the line
> Use statements are one token, just like strings are in other languages

### Grammar Rules

```
definiton: DEFINITION ALIAS expression SEMI
use: USE SEMI
expression: OPAREN (ALIAS | ID | call) CPAREN
call: expression expression
lambda: LAMBDA ID DOT expression
```

## Examples

| λ-Calculus Definition            | Rust Usage |
| -------------------------------- | ---------- |
| `fn T lx.ly.x`                   | `true`     |
| `fn F lx.ly.y`                   | `false`    |
| `fn ! lx.x F T`                  | `!x`       |
| `fn & lx.ly.x y x`               | `x & y`    |
| `fn \| lx.ly.((x x) y)`          | `x \| y`   |
| `fn SUCC lx.ln.lm.(n ((x n) m))` | `x + 1`    |
| `fn 0 ls.lz.z`                   | `0`        |
| `fn 1 SUCC 0`                    | `1`        |
| `fn + lx.ly.x (SUCC y)`          | `x + y`    |
| `fn * lx.ly.lz.(x (y z))`        | `x * y`    |
| `fn <=0 lx.(x F) N F`            | `x <= 0`   |
| `fn >= lx.ly.(Z (x P) y)`        | `x >= y`   |
| `fn >=1 lx.x F N T`              | `x >= 1`   |

> [!NOTE]
> This grammar does support numerical and 'special' characters in (and even as the first character of) ALIASes, meaning full high-level languages could be defined with normal arithmatic. However, IDs must be lowercase alphabetic characters and the '\_' character.

## TODO

- [x] Lexer
- [x] Parser
- [x] Reducer
- [x] Automatic Alpha Converter
- [x] CLI Functionality
- [x] Cross-File Linking (not object files)
  - [x] Basic Static Linking
  - [x] Standard Library for Basic Functions (auto-included)
- [ ] Multiple Backend Options
  - [x] Beta Reduction
  - [x] Naive transpilation to λ-calculus with JS syntax

## Quirks

- When Troth encounters an `ALIAS` being invoked, it first runs the alias through a an alpha conversion process, where each id within the aliased expression is appended with an alias-specific postfix to avoid conflation
- Troth performs rudementry static linking by parsing the included file, then copying all entries in the resultant `definitions` map into the original parser's map
- Currently, Troth immediantly evaluates the expression without displaying intermediate steps
