# Contained keyword
------------

## Variable
--------------
- `var` -> variable
- `val` -> value
- `con` -> constants

## Function
--------------
- `ret` -> return value

## Typ, Pro, ....
--------------
- `typ` -> create a type
- `enm` -> create a enum
- `pro` -> create a protocal
- `ext` -> create a extension
- `mon` -> create a monad

## Module managing
----------------
- `imp` -> import a module or crate, or what ever
- `pub` -> changing a function, protocal, or what ever to be public

## Control flow
-------------
- `if` -> if ...
- `elf` -> else if ...
- `els` -> else ...
- `for .. in ..` -> for loop
- `while` -> while ...

## Block
--------
- `do` -> a new block

## Expression
--------------
- `... orelse ...` -> excute something if there is err presented in the expression

## Pattern Matching
-------------
- `switch ...` -> pattern matching

## Unsafe
---------
- `unsafe ...` -> unsafe codes

# Symbol syntax
---------------

## The '@'
------------
'@' is a symbol that is used to call 'Compiler-level instruction', can be extend or change
 
example:

```cat
var v1 = 1
@kill v1 -- kill the variable (delete from mem)
```

**bulitin compiler-level instructions can be call straightly, but for extend compiler-level instructions must write
parent module**
