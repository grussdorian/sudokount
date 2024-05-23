## Problem statement: count the number of valid solutions of a given $size^4$ puzzle 

### Example:

Input
```
3
1 3 0 0 6 0 0 2 5 
0 0 0 0 5 0 0 0 0 
0 0 6 1 0 7 9 0 0 
0 0 5 6 3 9 4 0 0 
0 0 0 0 0 0 0 0 0 
9 0 2 0 4 0 3 0 7 
0 5 0 8 0 3 0 7 0 
0 0 7 0 0 0 0 0 0 
4 0 0 0 0 0 0 0 6
```

Output
```
11
```

### To compile the C code
```bash
foo@bar$ gcc sudokount.c -o sudokount
```
### To compile and run the rust code first [install rust](https://www.rust-lang.org/tools/install) and then

```bash
foo@bar$ cargo run < input/sudokount1.in
```

### To check the solution with the sequential C code
```bash
foo@bar$ ./sudokount < input/sudokount1.c
```

### Currently only [sudokount1.in](./input/sudokount1.in) file gives the solution within feasible time