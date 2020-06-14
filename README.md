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
  - add
  - sub
  - mul
  - div
  - lt
  - lte
  - gt
  - gte
  - eq
  - neq
  - jmp
  - rjmp
  - jmpt
  - jmpf

***

**The Push Instruction**

The Push instruction takes one parameter: the value to be pushed on to the stack.

Example:

```
push 1
```

> The push instruction takes an int, 1, and pushes this value on the stack.

Example:

```
push 1
```

After this instruction, the stack will look like this:
```
[1]
```

**The Pop Instruction**

The Pop instruction takes zero parameters.

Example:

```
pop
```

> The pop instruction removes the top value from the stack and returns this value.

Example:
```
push 1
pop
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the int 1.

**The Add Instruction**

The Add instruction takes zero parameters.

Example:
```
add
```

> The add instruction removes the top two values from the stack and adds them together. It then returns this value.

Example:

```
push 1
push 2
add
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the int 3.

**The Sub Instruction**

The Sub instruction takes zero parameters.

Example:
```
sub
```

> The sub instruction removes the top two values from the stack and subtracts them. It then returns this value.

Example:

```
push 1
push 2
sub
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the int 1.

**The Mul Instruction**

The Mul instruction takes zero parameters.

Example:
```
mul
```

> The mul instruction removes the top two values from the stack and multiplies them. It then returns this value.

Example:

```
push 1
push 2
mul
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the int 2.

**The Div Instruction**

The Div instruction takes zero parameters.

Example:
```
div
```

> The div instruction removes the top two values from the stack and divides them. It then returns this value.

Example:
```
push 1
push 2
div
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the int 0 because 1 / 2 = 0 with integer division. If float division was used, the result would be 0.5.

**The Lt Instruction**

The Lt instruction takes two parameters.

Example:
```
lt 1 5
```

> The lt instruction checks if the first parameter is less than the second parameter. It then returns a boolean value representing the result of the comparison.

Example:
```
lt 1 5
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the boolean true because 1 < 5.

**The Lte Instruction**

The Lte instruction takes two parameters.

Example:
```
lte 1 5
```

> The lte instruction checks if the first parameter is less than or equal to the second parameter. It then returns a boolean value representing the result of the comparison.

Example:
```
lte 5 5
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the boolean true because 5 <= 5.

**The Gt Instruction**

The Gt instruction takes two parameters.

Example:
```
gt 1 5
```

> The gt instruction checks if the first parameter is greater than the second parameter. It then returns a boolean value representing the result of the comparison.

Example:
```
gt 1 5
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the boolean false because 1 < 5.

**The Gte Instruction**

The Gte instruction takes two parameters.

Example:
```
gte 1 5
```

> The gte instruction checks if the first parameter is greater than or equal to the second parameter. It then returns a boolean value representing the result of the comparison.

Example:
```
gte 5 5
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the boolean true because 5 >= 5.

**The Eq Instruction**

The Eq instruction takes two parameters.

Example:
```
eq 1 5
```

> The eq instruction checks if the first parameter is equal to the second parameter. It then returns a boolean value representing the result of the comparison.

Example:
```
eq 1 5
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the boolean false because 1 != 5.

**The Neq Instruction**

The Neq instruction takes two parameters.

Example:
```
neq 1 5
```

> The neq instruction checks if the first parameter is not equal to the second parameter. It then returns a boolean value representing the result of the comparison.

Example:
```
neq 5 5
```

After this instruction, the stack will look like this:
```
[]
```

The value returned will be the boolean false because 5 == 5.

**The Jmp Instruction**

The Jmp Instruction takes one parameter.

Example:
```
jmp 1
```

> The jmp instruction checks if the parameter passed is within the bounds of the program, and if it is, it jump to the location specified.

Example:
```
jmp 4
push "Failed"
push "Succeeded"
```

After this instruction, the stack will look like this:
```
["Succeeded"]
```

No value is returned from the jmp instruction.

**The Rjmp Instruction**

The Rjmp Instruction takes one parameter.

Example:
```
rjmp 1
```

> The rjmp instruction checks if the current location + the parameter passed is within the bounds of the program, and if it is, it jump to the current location + the location specified.
> This parameter can be positive or negative as long as the sum is within the bounds.

Example:
```
rjmp 4
push "Failed"
push "Succeeded"
```

After this instruction, the stack will look like this:
```
["Succeeded"]
```

No value is returned from the rjmp instruction.

**The Jmpt Instruction**

The Jmpt Instruction takes one parameter.

Example:
```
push true
jmpt 3
```

> The jmpt instruction first checks if the top value on the stack is true. If it is, it checks if the parameter passed is within the bounds of the program, and if it is, it jump to the location specified.

Example:
```
push true
jmpt 6
push "Failed"
push "Succeeded"
```

After this instruction, the stack will look like this:
```
["Succeeded", true]
```

No value is returned from the jmpt instruction.

**The Jmpf Instruction**

The Jmpf Instruction takes one parameter.

Example:
```
push false
jmpf 3
```

> The jmpf instruction first checks if the top value on the stack is false. If it is, it checks if the parameter passed is within the bounds of the program, and if it is, it jump to the location specified.

Example:
```
push false
jmpf 6
push "Failed"
push "Succeeded"
```

After this instruction, the stack will look like this:
```
["Succeeded", false]
```

No value is returned from the jmpf instruction.

**The Print Instruction**

The Print Instruction takes one parameter.

Example:
```
print "Hello, World!"
```

> The print instruction prints out the parameter. It does not, however, print a new line character after. If you desire that behavior, check out the "printn" instruction.

Example:
```
push 1
push 1
push add
print "1 + 1 = "
print pop
```

After this instruction, the stack will look like this:
```
[]
```

No value is returned from the print instruction.

**The Printn Instruction**

The Printn Instruction takes one parameter.

Example:
```
printn "Hello, World!"
```

> The printn instruction prints out the parameter with the new line character after.

Example:
```
printn "Hello, World!"
push 1
push 1
push add
printn "1 + 1 = "
printn pop
```

After this instruction, the stack will look like this:
```
[]
```

No value is returned from the print instruction.

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