# About the syntax of catlang.....

at first, it had been designed like this (for better syntax highlighting, we will use rust):

```rust
fun main() {
    println!("hello, world!");
}
```

but then, after I have learned functional programming (haskell and F#), the syntax started to be like this:

```haskell
main =>> IO = do
    println "Hello, World"
```

and after the `toplevel expression` feature, the simple helloworld became like this:

```haskell
println "Hello, World"
```

... functional programming is crazy
