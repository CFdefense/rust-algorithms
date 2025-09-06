## Week 2 Running Time of Algorithms

### Chapter 2.1 Exercises:

#### 1. Consider the searching problem:

**Input**: A sequence of n numbers <a1,a2,an> stored in an array A[1:n] and a value x.
**Output**: An index i such that x equals A[i] or Nil if x does not appear in A.

##### Write psuedocode for a linear search algorithm which scans through A to find the targer value

```
for i = 1 to n:
    if A[i] equals x
        return i
return nil
```

**Loop Invariant**: At the start of each loop iteration at index i, the target value x has not been found in the array so far: A[1..i-1].

##### Proof using the 3 necessary properties:

**Initialization**: Before the first iteration the subarray is empty and as such we have not found x.

**Maintenance**: After checking if A[i] == x, if it is true we return i, or continue on having verified that x is not in A[1..i], both are correct.

**Termination**: When our loop ends we have either returned i after confirming A[i] == x or we have not found x and return Nil, both are correct.

### Chapter 2.2 Exercises:

#### 1. Consider the sorting algoritm: **Selection Sort**, in which for each iteration i we find the smallest number in the array A[i..n] and replace it with A[i].

##### a. Annotate the psuedocode with the **costs** and **times**.


##### b. Selection Sort Loop Invariant


##### c. Why does it only need to run for n-1 elements of A rather than all elements?


##### d. Best-Case Running Time.


##### e. Worst-Case Running Time.

#### 2. Consider the linear search algorithm again.

##### a. Annotate the psuedocode with **costs** and **times**


##### b. How many elements in the input sequence need to be checked on average assuming that the element being searched for is equally likely to be anywhere in the array?


##### c. How about the Worst Case scenerio?


##### d. What is the Average Case Running Time?


##### e. What is the Worst Case Running Time?


### Chapter 3 Exercises:

#### 1. Explain why the statement, “The running time of algorithm A is at least O(n2),” is meaningless.

#### 2. Indicate, for each pair of expressions (A, B) in the table below, whether A is O, o, Ω, ω, or Θ of B. Assume that k ≥ 1, ϵ > 0, and c > 1 are constants. 

| A       | B       | O   | o   | Ω   | ω   | Θ   |
|---------|---------|-----|-----|-----|-----|-----|
| lg^k n  | n^ε     |     |     |     |     |     |
| n^k     | c^n     |     |     |     |     |     |
| √n      | n^sin n |     |     |     |     |     |
| 2^n     | 2^(n/2) |     |     |     |     |     |
| n^lg c  | c^lg n  |     |     |     |     |     |
| lg(n!)  | lg(n^n) |     |     |     |     |     |
