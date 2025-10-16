# Syntax

example:

hello world

```cat
main =>> IO() = do
    println "Hello, world!"
```

variables

```cat
var x: I32 = 0
val c = 9
con PI = 3.14
```

function

```cat
square x = x * 2
```

monads

```cat
mon Console : IO

main =>> Console = do
    println "hello, world!"
```

currying~

```cat
thisIsACurryingFunction -> (Integer Integer -> Integer)
```

typ

```cat
typ Human name: String age: U32 =
    init

    intro =>> IO = do
        println "my name is {}, my age is {}"

val human = Human "Bobby" 10

human.intro
```

output:

```sh
my name is Bobby, my age is 10
```

protocal & extension

```cat
typ Human name: String age: U32 =
    init

pro Animal =
    intro =>> IO

ext Animal <: Human =
    intro =>> IO = do
        println "my name is {}, my age is {}"

val human = Human "Bobby" 10

human.intro
```

output:

```sh
my name is Bobby, my age is 10
```

PLEASE PASS ALL OF THE CHECK BEFORE MAKING A PULL REQUEST

Supported FP features:

- [x] currying
- [x] monads
- [x] immutable (optional rule, you can also use mutable value)

-- Uncompleted Doc --
