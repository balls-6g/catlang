# Ast construction
-------------------------

see [construction/mod.rs](https://github.com/balls-6g/catlang/blob/main/src/ast/construction/mod.rs)

## Program
--------------

This is the whole program

Fields:
| Field Name | Type | Desc |
| --------------- | --------------- | --------------- |
| `toplevels` | `Vec<TopLevel>` | The top level expression of the program |

## TopLevel
------------

This is the toplevels of the program

Enum:
| Field name | Types | Desc |
| --------------- | --------------- | --------------- |
| `Mod` | `(Module)` | Defining a module |
| `Typ` | `(Type)` | A type (similar to `class`) |
| `Ext` | `(Extenssion)` | Create a extenssion for a typ or enm |
| `Imp` | `(String)` | Importing a module |
| `Function` | `(Function)` | Defining a function |
| `Const` | `(Const)` | Defining a constant |
| `Protocal` | `(Protocal)` | Defining a protocal |
| `Enum` | `(Enum)` | Defining a enum |

## Function
-------------

This is a struct of function:

Fields:
| Field name | Type | Desc |
| --------------- | --------------- | --------------- |
| `name` | `String` | The name of the function |
| `args` | `Vec<Args>` | The args of the function |
| `body` | `FinalExpr` | The body of the funtion |
| `ret_type` | `String` | The return type of the function |
| `pos` | `Pos` | The location of the function defenition in the file |

## Const
--------------------

This is a constant:

Fields:
| Field name | Type | Desc |
| --------------- | --------------- | --------------- |
| `name` | `String` | The name of the constant |
| `typ` | `String` | The type of the constant |
| `value` | `String` | The real value of the constant |

## FinalExpr
----------------

This is a block:

Fields:
| Field name | Type | Desc |
| --------------- | --------------- | --------------- |
| `exprs` | `FinalExpr` | A block of expresions |

## Expr

This is the types of exprs:

Enum:
| Field name | Type | Desc |
| ------------- | --------------- | ----------- |

-- Uncompleted doc --
