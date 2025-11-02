## Week 10 Binary Search Trees & Ordered Sets

### Chapter 12.1 Exercises:

##### For the set {1, 4, 5, 10, 16, 17, 21} of keys, draw binary search trees of heights 2, 3, 4, 5, and 6. Additionally for each tree you draw, propose a specific ordering of the keys that would result in the construction of that tree.

###### Height 2

        10
       /  \
      5    16
     / \   / \
    1   4 17 21

Insertion Order: {10,5,16,1,4,17,21}

###### Height 3

       4
      / \
     1   10
        /  \
       5    17
           /  \
          16   21

Insertion Order: {4,1,10,5,17,16,21}

###### Height 4

    1
     \
      4
        \
         10
        /  \
       5    17
           /  \
          16   21


Insertion Order: {1,4,10,5,17,16,21}

###### Height 5

    1
     \
      4
       \
        5
         \
          10
            \
             17
            /  \
           16   21


Insertion Order: {1,4,5,10,17,16,21}

###### Height 6

    1
     \
      4
       \
        5
         \
          10
            \
             16
               \
                17
                  \
                   21


Insertion Order: {1,4,5,10,16,17,21}

##### What is the difference between the binary-search-tree property and the min-heap property of page 163? Can the min-heap property be used to print out the keys of an n-node tree in sorted order in O(n) time? Show how, or explain why not.

###### What is the difference between the binary-search-tree property and the min-heap property of page 163? 

###### Can the min-heap property be used to print out the keys of an n-node tree in sorted order in O(n) time? (Note: Should be understood to include a requirement that the tree not be modified.)

###### Show how, or explain why not.

### Chapter 12.2 Exercise 

##### Professor Kilmer claims to have discovered a remarkable property of binary search trees.

###### Suppose that the search for key k in a binary search tree ends up at a leaf. Consider three sets: 

a. A, the keys to the left of the search path
b. B, the keys on the search path
c. C, the keys to the right of the search path. 

###### Professor Kilmer claims that any three keys a in A, b in B, and c in C must satisfy a le b le c. Give a smallest possible counterexample to the professor's claim. (Note: Your counterexample should be in the form of a specific binary search tree T and key k.)

### Chapter 12.3 Exercise 

##### Is the operation of deletion "commutative" in the sense that deleting x and then y from a binary search tree leaves the same tree as deleting y and then x? Argue why it is or give a counterexample.

### Chapter 13.1 Exercise

##### Describe a red-black tree on n keys that realizes the largest possible ratio of red internal nodes to black internal nodes. What is this ratio? What tree has the smallest possible ratio, and what is the ratio?

### Chapter 13.3 Exercise

##### Show the red-black trees that result after successively inserting the keys 41, 38, 31, 12, 19, 8 into an initially empty red-black tree.

### Chapter 13.4 Exercise

##### A node x is inserted into a red-black tree with RB-INSERT and then is immediately deleted with RB-DELETE. Is the resulting red-black tree always the same as the initial red-black tree? Justify your answer.

