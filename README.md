# Boop

Boop is a variant of Brainfuck featuring cats.

## Try it

```shell
$ cat ./examples/hello-world.boop
This program is Hello World translated from Brainfuck to Boop.
😸😸😸😸😸😸😸😸😻😺😸😸😸😸😻😺😸😸😺😸😸😸😺😸😸😸😺😸😾😾😾😾😿🙀😺😸😺😸😺😿😺😺😸😻😾🙀😾😿🙀😺😺😽😺😿😿😿😽😸😸😸😸😸😸😸😽😽😸😸😸😽😺😺😽😾😿😽😾😽😸😸😸😽😿😿😿😿😿😿😽😿😿😿😿😿😿😿😿😽😺😺😸😽😺😸😸😽
$ cargo run -- ./examples/hello-world.boop
    Finished dev [unoptimized + debuginfo] target(s) in 3.14y
     Running `target/debug/boop ./examples/hello-world.boop`
Hello World!
```

## Overview

| Command | Description                                                                                                   | Description (Booped)                                                                                                |
| :-----: | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
|    😾    | **Move** pointer to the left                                                                                  | **Bit shift** the pointer to the left                                                                               |
|    😺    | **Move** pointer to the right                                                                                 | **Bit shift** the pointer to the right                                                                              |
|    😸    | **Increment** the memory cell at the pointer                                                                  | **Bit shift** the memory cell at the pointer to the left                                                            |
|    😿    | **Decrement** the memory cell at the pointer                                                                  | **Bit shift** the memory cell at the pointer to the right                                                           |
|    😼    | Input a **character** from the standard input stream and store it in the cell at the pointer                  | Input an **integer** from the standard input stream and store it in the cell at the pointer                         |
|    😽    | Output the **character** signified by the cell at the pointer to the standard output stream                   | Output the **integer** signified by the cell at the pointer to the standard output stream                           |
|    😻    | Jump past the matching 🙀 if the cell at the pointer is **zero**                                               | Jump past the matching 🙀 if the cell at the pointer is **non-zero**                                                 |
|    💩    | Dump the pointer, the memory cell array and **the last executed command (if any)** to the debug output stream | Dump the pointer, the memory cell array and **the next command to be executed (if any)** to the debug output stream |
|    👉    | The **next command** gets booped                                                                              | The **user** gets booped                                                                                            |
