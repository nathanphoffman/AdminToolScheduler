# Work in Progress
This is still a heavy work in progress, I highly recommend just going to the [Documentation Index](index) to learn Deor until this is done.

---
---
Work in Progress Follows
---
---

# Learning Deor
I will spend only a few words in this section, **please skip to the next heading if you are short on time**. 

My name is Nathan Hoffman, I am the creator behind Deor. I want this to be useful for both first time coders and technical people, hence why you will find three sections for each chapter below: the main one (in the middle), the newcomers block (above) and the techies block (below). This doc also serves as a great starting point for people who struggle learning Rust: Deor transpiles to Rust, has some strong Rust similarities, and allows raw rust to be written inside of rust blocks.

To give you an idea of why I created Deor, you have to understand my background: I was born into a sea of Commodore Amiga hardware and software in the late 80s through mid-90s when my dad's computer store closed; throughout much of my life, I have loved programming, and anytime I move away from it, I end up coming back to it. I began with C and JavaScript the age of 13 in the early 2000s; I moved to PHP for fun in the mid-late 00s, and then finally landed programming as a career starting around 2014, so it would not be an exaggeration to say I have been coding around about a quarter of a century. It wasn't all roses though, in the 2020s I started to have doubts about my programming career (a series of personal struggles), yet even so my love for code was so strong I still did it as a career (for some of that period) and for fun (for much of it). But as of writing this (2026) everything has changed, and my code love has grown so strong that coding may be at its pinnacle for me: so when I say I adore this language, I really do, it has been one of the chief outputs so far for me this year. I want you to know that I thought extremely long and hard about the design decisions made here. All this time coding has taken me back to my days on my dad's Amiga looking at pop-up boot console scroll across in wonder. Note: Everything on this page are my own human words (not AI).

## 1. Storing Information: *(Compiler Implicit as)*

```deor
times_to_say_hello as 10
i_should_say_hello as true
fahrenheit as 75.5
message as "hello, the temperature is "
```

You will see other ways to declare variables in this documentation, but ```as``` is best when the context of the value is obvious (like it is above) but when assigning things from outside as we will see later, it can be less obvious, in which case it is better to use the explicit type (covered next!)

> **Newcomers**: Variables are portions of information, think names, numbers, and lists. If you tell a computer to say hello, it needs to know what hello is, not just how to say it. Variables have names that are used to reference their value in the code. For some variables you just enter the bare value (like numbers), for others which are hard for the computer to determine, such as strings (bits of writing), you use "" to wrap the string. 


> **Techies**: Although these assignments look dynamic, they actually are validated and assigned types by the compiler, and the Rust compiler will hard-fail if it can't determine a good type to assign. All variables also get clone() applied in the Rust output, the Rust compiler will strip away cloned copy-types (so there is no added performance cost for copy-types), but it exists to make dealing with reference-types like Strings, Structs, and Vectors easier for the user at a cost to performance. To avoid this performance hit, the move keyword exists to allow ownership, and rust blocks exist for manual control (more on these later)


## 2. Data Types: Primitives and Strings (1 of 2)

```deor
int times_to_say_hello = 7
bool i_should_say_hello = true
float fahrenheit = 75.5
string message = "hello, the temperature is "
```

The form {type} {name} = is used for explicit variable declarations, a hold-over from older languages like C to make the convention more familiar to programmers who know what data types are, allowing newcomers to use the simpler syntax ```as```, or techies to reserve ```as``` when declarations are obvious like our literal values.
- ```int``` is a whole number (comes from math: integer)
- ```bool``` short for boolean, a true/false value (comes from George Boole, a founder of logical/discrete mathematics)
- ```float``` is a decimal (fractional eg. 1.56) value (it comes from the name floating-point calculation, basically a storage form of scientific notation) and can be inaccurate to use for extremely heavy amounts of math and financial data over many transactions due to limits of storage and binary accuracy. For regular day applications, you don't need to worry, as it has an accuracy of ~16 digits.
- ```string``` a portion of human-text comes from (a string of characters) a hold-over from early languages like C which treated strings as lists of individual characters.

> **Newcomers**: Data types (like int or bool) expose the underbelly of computers. They tell the computer how to manage their memory, this was more important back in the day (and remains today for high-performance applications), but even back in the 60s some programming languages like Fortran allowed omitting these, so don't feel bad if you don't like them. These "data types" tell the computer what is expected to be stored in the variable both for safety and memory regulation, and it can matter in some languages where smaller or bigger values were expected. The computer can't know what information is coming from outside, and so it can't know how to prepare storage for it. By providing computers the type they can both guard against bad data being processed and improve performance by limiting how much memory is reserved for that value. Fahrenheit is obviously never going to be 1,000 trillion on Earth, but a computer (and the rest of the world ;) ) don't know that! Deor simplifies things by making the default for these types very large, so 99.99% of the time you don't need to worry about it, and for safety the Rust code Deor generates is already safe: so the only real remaining factors are clarity and communicating expectations.

> **Techies**: Floats use Double Precision 64-bit, and Integers use long precision 64-bit. This decision was made due to modern hardware and because it vastly simplifies types, for more explicit types you can use rust blocks. In Rust strings are stored on the heap, which makes them expandable. However, this also means they are reference-types and are the only of these types not to be a copy-type. Deor does its best to disguise this fact by using to_string everytime it is met, this can be an expensive operation as the heap and pointer are both cloned.

## 3. Comments

```deor
# this is a comment so I can say whatever I want!

# let us go back to using as for these `primitive` values, they are obvious, and simple!
times_to_say_hello as 7  # this is a prime number, fun fact!
i_should_say_hello as true
fahrenheit as 75.5

# strings can be weird, as we will see, so we will keep explicit typing to differentiate it
string message = "hello, the temperature is "
```
Comments are applied with the # symbol called "pound" or "hash".  They can be applied above the line (Deor best practice) but they can also be put to the side as well.

> **Newcomers**: Time to relax after that nonsense about low level stuff! This is the easiest section of all, comments are just for the coders benefit, they are literally stripped away by the computer, it doesn't care about them at all, which is why most highlighters make them fade into the background! All you need to do with a comment is to lead the line with a # and anything on that line that follows is ignored!

> **Techies**: There is no block-comment, just use # for everything

## 4. If Statement (1 of 2)

```deor
times_to_say_hello as 7
i_should_say_hello as true
fahrenheit as 75.5

string message = "hello, the temperature is "

# since i_should_say_hello is set to true, this is true and runs
if i_should_say_hello is true
    print(message)  # prints: hello, the temperature is 
    print(fahrenheit)  # prints: 75.5

```
```If``` can be a boolean or a comparison (which returns a boolean value) which is what we see here. The equality comparison uses: ```is true``` which asks "is this statement true."  There is also an ```is false``` for the opposite. Like math there is order of operations so you could also have said ```if (i_should_say_hello is true)```. 

Deor uses tabbed-blocks which just means that when you need to make code a "child" of something else, you tab it below it, everything on that same tab-level lives at the same level of nesting or resides in the same "block".

The print you see here, prints to the terminal, and is explained in the next section.

> **Newcomers**: We are about to embark on our first bit of logic, code that does something! "If statements" are logical conditions, just as they read in english.  If statements are essential in programming languages for asking "should this code actually run?" When the expression (the condition to the right of the if) is true, then the code in the block below it (the tabbed code section) runs. If it is false, it is skipped. The block below the code is a "tabbed-block" think of it like a nested level where anything with a tab below it is considered inside of that block, and will run if the statement is true.


> **Techies**: is represents ==    is not represents !=

## 5. Else & Basic Built-Ins (crash, len, print)

```deor
times_to_say_hello as 7
i_should_say_hello as true
fahrenheit as 75.5

string message = "hello, the temperature is "

if fahrenheit < -459.67
    crash("Oh no, the temperature is less than absolute 0! That doesn't seem possible...")
else if i_should_say_hello is true
    # this prints: hello, the temperature is
    print(message)
    
     # this prints: 75.7 (right below the first message)
    print(fahrenheit)
else
    # our message is 26 characters long, so this stores 26
    length_of_message as len(message)

    print("I wasn't allowed to print your message, but you missed out, it was this long!: ")
    print(length_of_message)

```

Deor has only six built-in functions, three are explained in this section which are:
- len() outputs an int using the length of the string (the length of the text in "") for ```list``` (not yet discussed) it provides the list length as an int. 
- crash() hard crashes the entire program with the string error message provided, use this as a last resort if your internal logic can no longer resolve the problem.
- print() prints the string to the terminal, it can also take in float, int, or bool.

> **Newcomers: Else/Else If**: You will notice we have new keywords here "else if" and "else". Just like human language else only can follow an if, you can even ignore else if/else all together and just make a bunch of if statements. The advantage of else if / else though is that else if and else only run if the statements above them are false, or in otherwords they are tied to each other in a flow. This means that if the temperature is absolute 0 we don't even bother to run any of the else statements below that if. Likewise if i should_say_hello is true, then the else does not run at all. If it wasn't for else if / else pattern you would have to manually rewrite every test above it checking for absolute 0 on your else if line and else line as well!  else if differs from else because else if requires a conditional and you can have as many else ifs as you want trailing your if statement, whereas else is the last catch-all condition that runs if everything above it was false, which is why it doesn't take a condition and it must terminate the if-else chain. You are not required to use else if or else, and can even just use else if without else or else without else if (depending on if you need to check a condition).

> **Newcomers: Built-Ins**: Built-ins which are called with () like our print() and are called functions, we haven't discussed what functions are, don't worry about that. Just think of them as little units of work, you will end up defining some of your own soon enough! You just need to know what they do as described above for crash, len, and print.

> **Techies** Print is translated to rust as print!("{}",x), so it can be replaced with the standard library for low level OS concerns

## 6. For Loops (1 of 2)

```deor
times_to_say_hello as 7
i_should_say_hello as true
fahrenheit as 75.5

string message = "hello, the temperature is "

# prints "hello, the temperature is 75.5"  x7 times
for in range(times_to_say_hello)
    print(message)
    print(fahrenheit)
```

> **Newcomers** So far we have seen "times_to_say_hello" go unused. In programs, we often need to loop (we call this iterate) certain items like lists, mathematical formulas like summations (1+2+3+4...), or repeating processes. Since we need to say hello 7 times, we could manually type the two print statements 7 times over, but it is much easier to put our two print statements in a for loop and have it loop those 7 times, this also means times_to_say_hello can be adjusted numerically without touching the rest of our code!  You could even print it 1000 times just by replacing the 7 with 1000, this is the advantage of loops!  The range() built-in you see here is another built-in like those we discussed in the prior section, for now just know that when you type in a single input into a range it populates a placeholder list of that size (like writing the numbers 1 to 10 on a top 10 ranking before writing in the items).


## 7. Built In Functions: Inputs
> **Newcomers** What if we wanted to ask our user what the temperature was so we know what temperature to output?
Built-ins that will be discussed in the future (you can ignore for now):
- input() allows input to be taken in from the terminal
- args() same as input but used for function arguments 


The only built-in we haven't devlved into deeply is ```range``` which is next!


## 8. Complex For Loops (and range)


## 9. Data Types: Structs
> **Newcomers**: So far we have left out only three more data types: structs, lists

## 11. Data Types: Lists and Shapes  (3 of 4)
Shapes are used to declare what other languages call "generics". These are types that rely on other types. Think about a list. A list is a type, but a list itself doesn't tell us what is inside of it. A shape does, and allows us to alias it with a human readable syntax. 

Shapes can also define func (which are basically functions stored as variables). The func shape is really quite complex, and is covered later on, but I provided the definition here for documentation clarity. Func are an advanced feature and are covered at the very end of this documentation. Most of the times you encounter shapes in this documentation it will be for defining list types.

# 20. You are done the basics! Only Advanced Features Remain
> **Newcomers**: You did it! Nice, thanks so much for reading, it is an honor of mine to help you learn to code! If I ever had another job, it would be being a programming teacher.  You have greatly honored me by partaking in this journey.

You have completed this documentation page, only one more remains on advanced features if you want to read it, advanced features covers:
- First Class Functions (Func)
- Rust Blocks
- Lib Wrappers / Rust Interop
- Integration with Cargo

> **Techies**: Let me know what you think of my programming language, and I strongly encourage you to check out the advanced page