This language was written to be difficult to program in, and as such I greatly encourage you to try and figure out the language by yourself before referencing these docs to get the true NIL experience

---

## Basics

NIL is written **bottom** to **top** to parsing with start with the end of the file and work its way up line by line. Additional the completely unnecessary  ```;```  is mandatory and each statement must start with one.

## Types

The NIL language has 3 types: **str**, **bool**, and of course **num**

### Number Operations

| Op      | Args | Return   | Desc.                             |
| ------- |:----:|:--------:|:---------------------------------:|
| **NUM** | +    | NUM, NUM | Adds two numbers                  |
| **NUM** | -    | NUM, NUM | Subtracts two numbers             |
| **NUM** | /    | NUM, NUM | Divides two numbers               |
| **NUM** | *    | NUM, NUM | Take a guess                      |
| **NUM** | %    | NUM, NUM | Returns the Remainder of division |

## Variables

Inorder to initialize variables in NIL the following syntax is used:

```rust
;name type
```

Type **has to** be ignored if a variable is initialize with a value

```rust
;value = name
```

Variables are all mutable by default so inorder the change the value the same syntax can be used ex:

```Rust
;count + 1 = count
;0 = count
```

## Control Flow

Seeing as almost every programming languages uses some form of the famous ```if then else``` statements NIL has chose to use a 100% original and different ```not if(nif) then else nif``` statements

```
)
    stantments...
) else nif !cond (
    stantments...
nif cond (
```

this is the equivalent of the following js segment: 

```javascript
if (!cond) {
    stantments...
} else {
    stantments...
}
```

## Loops

To add to the 100% original innovations the NIL language as created are not loops or noops demonstrated below

```
)
    stantments...
;noop cond (
```

* note that as I don't want noops becoming more popular then nifs they require semicolons in front to make them less lovable

equivalent js code:

```js
while (!cond) {
    stantments...
}
```



## Functions

NIL has both built in and user definable functions
