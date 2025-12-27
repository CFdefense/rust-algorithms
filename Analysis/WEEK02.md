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

```
for i = 1 to A.length - 1       // c1 = n
    minIndex = i                // c2 = n
    for j=i to A.length         // c3 = (n(n-1))/2 -> Summation i=1 to n-1 (n-i-2)
        if A[j] < A[minIndex]   // c4 = (n(n-1))/2
            minIndex = j        // c5 = 0..(n(n-1))/2
    tmp = A[i]                  // c6 = n
    A[i] = A[minIndex]          // c7 = n
    A[minIndex] = tmp           // c8 = n

```

##### b. Selection Sort Loop Invariant

**Loop Invariant**: At the start of outer loops iterations the array A[1..i-1] has been sorted using ith smallest elements in increasing order and A[i..n] contains all remaining unsorted elements 

##### Proof using the 3 necessary properties:

**Initialization**: At initialization the array A[1..0] is empty. 

**Maintenance**: The inner loop finds the smallest element at index j and swaps it with A[i], as such A[1..i] are now sorted using the ith smallest elements.

**Termination**: After the loop runs n-1 times, the array A[1..n-1] is sorted and A[n] must be the greatest so it is also sorted.

##### c. Why does it only need to run for n-1 elements of A rather than all elements?

The algorithm's outer loop only needs to run for n-1 elements because once we reach that final element we have already sorted all smaller elements before it therefore, the final element must be the greatest and is as such already sorted. 

##### d. Best-Case Running Time.

T(n) = c1 + c2 + c3 + c4 + c6 + c7 + c8
T(n) = n + n + n(n-1) + n + n + n = n^2 - 2n + 5n 
T(n) = n^2 + 3n
O(n^2)

##### e. Worst-Case Running Time.

T(n) = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
T(n) = n + n + (3n(n-1))/2 + n + n + n = (3n^2 -3n)/2 + (10n)/2 
T(n) = (3n^2 + 7n)/2
O(n^2)

#### 2. Consider the linear search algorithm again.

##### a. Annotate the psuedocode with **costs** and **times**

```
for i = 1 to n:         // c1 = 1..(n + 1)
    if A[i] equals x    // c2 = 1..(n + 1) 
        return i        // c3 = 0 or 1
return nil              // c4 = 0 or 1
```

##### b. How many elements in the input sequence need to be checked on average assuming that the element being searched for is equally likely to be anywhere in the array?

If the target element is equally likely to be anywhere in the array and we must atleast check once, on average we will have to check half the array to find the target value (n + 1)/2.

##### c. How about the Worst Case scenerio?

The worst case scenerio is that the target element is not present in the array, as such we traverse every element and find nothing and then also have to do the final loop check before returning nil.

##### d. What is the Average Case Running Time?

Using the above average the average case running time is: 
T(n) = (n+1)/2 
O(n)

##### e. What is the Worst Case Running Time?

c1 + c2 + c4 = n + 1 + n + 1 + 1
T(n) = 2n + 3
O(n)

### Chapter 3 Exercises:

#### 1. Explain why the statement, “The running time of algorithm A is at least O(n2),” is meaningless.

This above statement is meaningless because Big O describes an upper bound, not a lower bound. Saying “at least O(n²)” confuses the upper bound for a lower one which is incorrect.

#### 2. Indicate, for each pair of expressions (A, B) in the table below, whether A is O, o, Ω, ω, or Θ of B. Assume that k ≥ 1, ϵ > 0, and c > 1 are constants. 

Bounds CheatSheet:
O: Upper Bound (worst case)
o: Strict Upper Bound (grows slower than)
Ω: Lower Bound (best case)
ω: Strict Lower Bound (grows faster than)
Θ: Tight Bound (both upper and lower bound)

Limits CheatSheet:
If the limit = 0 → 𝐴∈𝑜(𝐵)
(A grows strictly slower).

If the limit = ∞ → 𝐴∈𝜔(𝐵)
(A grows strictly faster).

If the limit = a positive finite constant → 𝐴∈Θ(𝐵)
(same rate).

If the limit = finite but not 0 → also means 𝐴∈𝑂(𝐵) and 𝐴∈Ω(𝐵).
(limit bounded)

| A       | B       | O   | o   | Ω   | ω   | Θ   |
|---------|---------|-----|-----|-----|-----|-----|
| lg^k n  | n^ε     | yes | yes | no  | no  | no  |
| n^k     | c^n     | yes | yes | no  | no  | no  |
| √n      | n^sin n | no  | no  | no  | no  | no  |
| 2^n     | 2^(n/2) | no  | no  | yes | yes | no  |
| n^lg c  | c^lg n  | yes | no  | yes | no  | yes |
| lg(n!)  | lg(n^n) | yes | no  | yes | no  | yes |


1. A = lg^k n,  B = n^ε

**Compute the limit of A/B as n approaches infinity**: 0

2. A = n^k, B = c^n

**Compute the limit of A/B as n approaches infinity**: 0

3. A = √n, B = n^sin n 

**Compute the limit of A/B as n approaches infinity**: Cannot Genuinely Compute Due to Sin()'s Oscillatory Behavior

4. A = 2^n, B = 2^(n/2)

**Compute the limit of A/B as n approaches infinity**: ∞

5. A = n^lg c, B = c^lg n

**Compute the limit of A/B as n approaches infinity**: 1

6. A = lg(n!), B = lg(n^n)

**Compute the limit of A/B as n approaches infinity**:

##### AI Disclosure

I used AI to help compute the limits: [View ChatGPT Conversation](https://chatgpt.com/share/68bdc063-98e0-8003-948d-7633c5964456) 
