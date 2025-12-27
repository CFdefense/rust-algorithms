## Week 4 Recursion + Divide & Conquer

### Chapter 2.3-5. Exercises:

##### Write psuedocode for a recursive approach to Insertion Sort.

```
insertionSort(array, n):
    // Base Case: single element is already sorted
    if n <= 1:
        return

    // Step 1: Recursively sort the first n-1 elements
    insertionSort(array, n - 1)

    // Step 2: Insert the nth element into the sorted subarray
    key = array[n-1]
    j = n - 2

    while j >= 0 and array[j] > key:
        array[j+1] = array[j]
        j = j - 1

    array[j+1] = key

```

##### Worst Case Running Time:

{O(1), n <=1; T(n-1) + O(n), n > 1}. 

This scenerio occurs when every newly inserted element is compared against every existing element before being inserted ie: the input is reverse sorted.

### Chapter 20.3-2. Exercises:

##### Draw the graph from the main method of the attached DFS code and then show how depth-first search works on this graph. Assume that the for loop of lines 5-7 of the DFS procedure pseudocode considers the vertices in alphabetical order, and assume that each adjacency list is ordered alphabetically. Show the discovery and finish times for each vertex, and show the classification of every edge.

Adjacency (directed):

- A: B, D
- B: C, E
- C: F
- D: E, G
- E: F, G
- F: —
- G: H
- H: F

Graph

```
A ────→ B ─→ C ─→ F ←─┐ 
|       |         ↑   |
│       ↓         │   | 
└→ D ─→ E ─→ G ─→ H   |
   │    │    ↑        |
   └─────────┘        |
        └─────────────┘

```

DFS run (vertices considered A, B, C, D, E, F, G, H in order; each adjacency list is already alphabetical):

Discovery/finish times (d/f):

| Vertex | d  | f  |
|-------:|---:|---:|
| A      |  1 | 16 |
| B      |  2 | 13 |
| C      |  3 |  6 |
| D      | 14 | 15 |
| E      |  7 | 12 |
| F      |  4 |  5 |
| G      |  8 | 11 |
| H      |  9 | 10 |

Edge classification (directed):

- Tree edges: A→B, B→C, C→F, B→E, E→G, G→H, A→D
- Back edges: none
- Forward edges: none
- Cross edges: D→E, D→G, E→F, H→F

### Divide & Conquer:

#### Independent Investigation

##### Search our textbook or the Web to find an implementation of a divide-and-conquer algorithm of your choice - I recommend choosing a relatively short one. Copy-and-paste the code into your WEEK04.md file and annotate the code to clearly identify (i.e., indicate the line numbers for) the divide, conquer, and combine steps. Then, without doing a detailed analysis, guess at the asymptotic running time and briefly explain your guess.

Below is a short, standard implementation of Merge Sort.

```python
#  1 def merge_sort(arr):
#  2     # Base case: a list of zero or one elements is already sorted
#  3     if len(arr) <= 1:
#  4         return arr
#  5
#  6     # DIVIDE: split array into two roughly equal halves
#  7     mid = len(arr) // 2
#  8     left = arr[:mid]
#  9     right = arr[mid:]
# 10
# 11     # CONQUER: recursively sort each half
# 12     left_sorted = merge_sort(left)
# 13     right_sorted = merge_sort(right)
# 14
# 15     # COMBINE: merge the two sorted halves
# 16     return merge(left_sorted, right_sorted)

# 17 def merge(left, right):
# 18     result = []
# 19     i = j = 0
# 20     while i < len(left) and j < len(right):
# 21         if left[i] <= right[j]:
# 22             result.append(left[i])
# 23             i += 1
# 24         else:
# 25             result.append(right[j])
# 26             j += 1
# 27     # append any remaining elements
# 28     result.extend(left[i:])
# 29     result.extend(right[j:])
# 30     return result
```

Annotation:
- Divide: lines 6-9 (split the array at `mid`)s.
- Conquer (recursive): lines 11-13 (recursive calls to `merge_sort` on each half).
- Combine (merge): lines 15 and 17-30 (call to `merge` and the `merge` function body which interleaves elements).

Asymptotic running time (informal guess): O(n log n).

Brief explanation: at each level of recursion the array is split into two halves (log n levels), and the merge/combine step at each level processes all n elements in linear time, so the total work is roughly n per level times log n levels, giving O(n log n).

### Exercise 4.4-1

##### For two of the following four recurrences, sketch its recursion tree, and guess a good asymptotic upper bound on its solution. Then use the substitution method to verify your answer.

###### T(n) = T(n/2) + n^3

Recursion tree:

```
n^3
 |
(n/2)^3 = n^3/8
 |
(n/4)^3 = n^3/64
 |
...
 |
(n/2^L)^3 = n^3 / 8^L   where L = log_2 n
 |
 1
```

Total cost:

S = n^3 (1 + 1/8 + 1/8^2 + ... + 1/8^{log_2 n})

Geometric series (ratio 1/8) ⇒ total Θ(n^3).

T(n) = Θ(n^3).

Substitution:

Claim: T(n) <= c n^3 for some constant c >= 8/7 and large n.
Base: For n <= n0 choose c large enough so the inequality holds.
Inductive step: assume for all m < n, T(m) <= c m^3. Then

T(n) = T(n/2) + n^3 <= c (n/2)^3 + n^3 = (c/8) n^3 + n^3.

Require (c/8) n^3 + n^3 <= c n^3 ⇒ 1 <= c(1 - 1/8) = (7/8)c ⇒ c >= 8/7. 

A lower bound T(n) >= n^3 gives T(n) = Θ(n^3).


###### T(n) = 4T(n/2) + n

Recursion tree:

```
Level 0:                               [ n ]
                                      /  |  |  \
Level 1:                    [ n/2 ] [ n/2 ] [ n/2 ] [ n/2 ]
                              /        |       |       \ 
Level 2:           16 nodes total (4^2), each with cost [ n/4 ]
                                        ...
Level i:                4^i nodes, each with cost [ n / 2^i ]
                                        ...
Final level L = log_2 n: 4^L = n^2 leaves, each with cost [ 1 ]

Per-level total cost: 4^i * (n/2^i) = n * 2^i, so the last level dominates at n * 2^{log_2 n} = n^2.
```

Substitution - upper:

We prove T(n) <= c n^2 - k n for suitable constants c, k > 0 by induction.

Base: Choose c, k so the inequality holds for small n (n <= n0).
Inductive step: assume for m < n, T(m) <= c m^2 - k m. Then
T(n) = 4 T(n/2) + n <= 4 (c (n/2)^2 - k (n/2)) + n = c n^2 - 2kn + n = c n^2 - (2k - 1)n.

To get T(n) <= c n^2 - k n we need (2k - 1) >= k, i.e., k >= 1. Choose k = 1 and pick c large enough to satisfy the base cases; the induction goes through and yields an upper bound Θ(n^2).

Substitution- lower:

Assume T(n) >= d n^2 for some small d > 0. Then

T(n) = 4 T(n/2) + n >= 4 d (n/2)^2 + n = d n^2 + n >= d n^2,

Thus T(n) = Θ(n^2).