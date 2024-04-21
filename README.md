# Mashup Lang

### This is a personal project of my own to learn about how programming languages work

## Types:

```
- int
- float
- list: [(static size) | dynamic; type]; indexing starts at 0 eg names[0]
  if static size is fixed to size declared
- tuple: (type1, type2, ...)
- string: dynamic and static; u8 array by type
- char: u8 by type
- bool
- Structs
- Enums
- void: no type / void
- Some and Result types: A wrapper type that has an inner value of a standard type they can either be Some(inner value), None or Ok(inner value), Err
```

# Basic Syntax

int (all math operations)
float (all math operations)
list all append(value), remove(value?), take(size), reduce(amount to reduce by), filter(predicate)
strings: all list operations as they are u8 arrays by type

default math operations: \
`+, -, *, /, pow or **, sqrt(), %,`

Standard Liberary:
math - extra math function such sin, cos, etc as well as constants such as pi
strings - extra functions for working with strings and chars

### Variables

Variables are simple and are smiliar to most other programming languages:

```
let hello_world: [static auto; string] = "Hello World!"; // u8 char array under the hood; auto indicates the program is determining the size of the array
let age: mut int = 5;

println f"{hello_world}"; // prints "Hello World" to the console
```

#### Types for Variables

#### Standard Types

Variables can have different types and are structured differently, see types above for more info on each type. Types for a variable are declared after the variable with a `:`.

Example:

```
let x: int = 5; // creates a variable 'x' with type of integer and a value of 5;
```

#### Arrays

All arrays are defined with the type surrounded by braces eg: `[int]`, this is not disimilar to other languages but the differences is that the size of the array must be known when the array is created. We acheive this by either stating if the array is dynamic, in which the compiler will handle the size of the array, or static, in which you give the array a size youself.

Example:

```
let first: [static 5; int] = [1, 2, 3, 4, 5]; // arrays with a size of 5
// you can also let the compiler determine the size for you
let second: [static auto; int] = [1, 2, 3, 4, 5]; // same size as first
```

This creates an array with a size of five and ensures that it cannot exceed or drop below a size of five

```
first :: append with 6; // error: variable 'first' has a static size of 5;
first :: remove with 6; // error: variable 'first' has a static size of 5;
```

As seen in the example above, arrays have special operations that can be applied to them such as append and remove. These argumes always start with the variable or value as the first argument, in this case `first`. The variable is followed by a `::` which signifies that a type method is following and is the followed by `with` to signifiy the value that is being passed into the argument and then the value, in this case `6`. With some type methods, these extra argmunets sush as `with 6` are optional. All methods that require mutation of the original array must have the array specified with `mut` even if the array is declared dynamic.

Arrays are index with a `[position]` syntax and are 0 indexed. example:
`let value: int = first[1]; // value = 2`

Array Methods:

Array used in the examples below: `let array: mut [dynamic; int] = [1, 2, 3, 4, 5]`

- append: MUST be followed by a value. Example: \
  `array :: append with 6;` appends 6 to an array yeilding the array `[1, 2, 3, 4, 5, 6]`
- remove: May or may not be followed by an value. When not followed by an example, it will attempt to remove the last element of the list similar to `.pop()` in other languages. Example: \
  `array :: remove // yeilds [1, 2, 3, 4]` \
  `array :: remove with 4 // yeilds [1, 2, 3, 5]`. \
- filter: MUST be followed by a predicate that must either return a boolean true and will filter out anything that matches the predicate . Example: \
  `array :: filter with pred (x: int) yeild x == 5;` (predicates will be touched on later)
- Contains; Must be followed by a value. see example below
- foreach and map; Both MUST Be followed by a predicate however, foreach cannot mutate the array where as map and and requires a mutable array. examples below

contains example:

```
let does_six_exist: bool = array :: contains 6; // false
let does_six_exist: bool = array :: not contains 6; // true
```

foreach and map example:

```
let fruit_people_like: [static 5; string] = ["Apple", "Banana", "Grape", "Orange", "Pineapple"];
let actually_good_fruit: [static 3; string] = ["Apple", "Banana", "Grape"];

let people_who_are_correct: mut [dynamic; string] :: foreach pred (fruit); do
    if (actually_good_fruit contains fruit) -> yeild fruit;
end;
```

### If statements

If statements are similar to all other languages but are different in one way. statements must be followed by a `->` and signifiy the code that follows under the condition.

Example:

```
if (true == false); then
    println "How did you get here";
else if (true == true); then
    println "I'm coming home, Ace.";
else; then
    println "How did you get here?";
endif;

```

### Functions

functions work similar to most other programming languages but are defined differently:

```
// main function:
fun main = (); do
...body
end
```

```
// with cli args
fun main = (argc: int, argv: [dynamic; string]); do
...body
end;
```

```
// creating your own function
fun function_name = (param1: type, param2: type, ...) : return_type; do
    body...
end;
```

Recursive functions must be defined as recursive as they have additional overhead:

```
recursive function example:
fun fib = rec (num: int) : int; do
    yeild num :: match with
        | 0 -> 0
        | 1 -> 1
        | _ -> fib(n - 1) + fib(n - 2);
end

```

Another example:

```
let nums_list: mut [dynamic; int]; // creates a dynamic int array with nothing inside

fun fib = rec (num: int) : (); do
    nums_list :: append match num with n
        | 0 -> 0;
        | 1 -> 1;
        | _ -> fib(n - 1) + fib(n - 2);
end

```

### Advanced Types (Struct, Enums, and Custom Types)

Some of the more modern features in languages are structs, enums, and custom types. Structs and enums can be a core part of what makes up a language and how it is used. Structs and enums are integreated by default and defined accordingly.

Enum Example

```
type Programmer enum with
    | soy_boy_js;
    | gigachad_rust;
```

Enum fields can also have values

```
type enum ItemTypeWithPower with
    | Sword: int
    | Flail: int
    | Bow: int;
```

You can also assign default values to enums

```
type enum KeyboardEvent with
    | h_keypress: string = "h"
    | left_click: string = "left click"
    | right_click: string = "right click";
};
```

Struct are also important too (they make people happier)

```
type struct Person {
    | name: string
    | age: int
    | programming_lang: Programmer;
```

you can also define static fields that cannot be changed even if the variable is mutable

```
type struct Book with
    | title: string
    | pages: static int;

let Lord_of_the_Rings: mut Book = Book { title: "Lord of the Rings", pages: 1137 };

Lord_of_the_Rings.title = "LOTR"; // Book { title: "LOTR", pages: 1137 }
Lord_of_the_Rings.pages = 556 // error, cannot mutate / reassign static field
```

### Implementations

Taking inspiration from rust, implimentations are supported for structs and enums that are defined. Example:
traits are not supported at this time

```
type struct Book with
    | title: string
    | pages: static int;

fun (book: Book) get_title = () : string; do
    yeild book.title
end

let i_dont_read: Book = Book { title: "LOTR", pages: 1137 };

println f"What was the name of that book? Oh, yea { i_dont_read :: get_title self }."
// prints: What was the name of that book? Oh, yea LOTR.
```

To-Do:
Predicates
