**DarkVM**

A new VM that focuses on providing current features without comprimising
on speed and simplicity.

> This project is also a way for me to learn more about VM's and compilers.

**Details About DarkVM**

DarkVM operates on a stack and uses different operations to operate on it.

**Types In DarkVM**

Currently, DarkVM supports the following types:
- void
- any
- int
- float
- boolean
- string

**Instruction Set**

*In this section, each instruction will be elaborated upon, along with examples*

The instructions that DarkVM currently supports are:
  - push
  - pop

***

**The Push Instruction**

The Push Instruction takes one parameter: the value to be pushed on to the stack.

Example:

```
push 1
```

> The push instruction takes an int, 1, and pushes this value on the stack.

After this operation, the stack will look like this:
```
[1]
```

**The Pop Instruction**

The Pop Instruction takes zero parameters.

Example:

```
pop
```

> The pop instruction removes the top value from the stack and returns this value.

After this operation, the stack will look like this:
```
[]
```

The value returned will be the int 1.

**Comments In DarkVM**

DarkVM supports both single line and multiline comments.

Single line comments can be created using the following syntax:
```sql

-- This is a comment.
```

Multiline comments can be created using the following syntax:
```
-!
  This is a multiline comment.
  I can write anything I want here.
!-
```