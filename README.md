**DarkVM**
-
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