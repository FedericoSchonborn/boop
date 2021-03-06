# Boop

Boop is a variant of Brainfuck featuring cats.

## Try it

```shell
$ cat ./examples/hello-world.boop
This program is Hello World translated from Brainfuck to Boop.
πΈπΈπΈπΈπΈπΈπΈπΈπ»πΊπΈπΈπΈπΈπ»πΊπΈπΈπΊπΈπΈπΈπΊπΈπΈπΈπΊπΈπΎπΎπΎπΎπΏππΊπΈπΊπΈπΊπΏπΊπΊπΈπ»πΎππΎπΏππΊπΊπ½πΊπΏπΏπΏπ½πΈπΈπΈπΈπΈπΈπΈπ½π½πΈπΈπΈπ½πΊπΊπ½πΎπΏπ½πΎπ½πΈπΈπΈπ½πΏπΏπΏπΏπΏπΏπ½πΏπΏπΏπΏπΏπΏπΏπΏπ½πΊπΊπΈπ½πΊπΈπΈπ½
$ cargo run -- ./examples/hello-world.boop
    Finished dev [unoptimized + debuginfo] target(s) in 3.14y
     Running `target/debug/boop ./examples/hello-world.boop`
Hello World!
```

## Overview

| Command | Description                                                                                                   | Description (Booped)                                                                                                |
| :-----: | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
|    πΎ    | **Move** pointer to the left                                                                                  | **Bit shift** the pointer to the left                                                                               |
|    πΊ    | **Move** pointer to the right                                                                                 | **Bit shift** the pointer to the right                                                                              |
|    πΈ    | **Increment** the memory cell at the pointer                                                                  | **Bit shift** the memory cell at the pointer to the left                                                            |
|    πΏ    | **Decrement** the memory cell at the pointer                                                                  | **Bit shift** the memory cell at the pointer to the right                                                           |
|    πΌ    | Input a **character** from the standard input stream and store it in the cell at the pointer                  | Input an **integer** from the standard input stream and store it in the cell at the pointer                         |
|    π½    | Output the **character** signified by the cell at the pointer to the standard output stream                   | Output the **integer** signified by the cell at the pointer to the standard output stream                           |
|    π»    | Jump past the matching π if the cell at the pointer is **zero**                                               | Jump past the matching π if the cell at the pointer is **non-zero**                                                 |
|    π©    | Dump the pointer, the memory cell array and **the last executed command (if any)** to the debug output stream | Dump the pointer, the memory cell array and **the next command to be executed (if any)** to the debug output stream |
|    π    | The **next command** gets booped                                                                              | The **user** gets booped                                                                                            |
