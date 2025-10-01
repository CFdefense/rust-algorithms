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

```

```

### Divide & Conquer:

#### Independent Investigation

##### Search our textbook or the Web to find an implementation of a divide-and-conquer algorithm of your choice - I recommend choosing a relatively short one. Copy-and-paste the code into your WEEK04.md file and annotate the code to clearly identify (i.e., indicate the line numbers for) the divide, conquer, and combine steps. Then, without doing a detailed analysis, guess at the asymptotic running time and briefly explain your guess.

### Exercise 4.4-1

##### For two of the following four recurrences, sketch its recursion tree, and guess a good asymptotic upper bound on its solution. Then use the substitution method to verify your answer.

###### T(n) = T(n/2) + n^3


###### T(n) = 4T(n/2) + n