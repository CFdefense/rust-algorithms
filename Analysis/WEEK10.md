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

The Binary Search Tree property maintains that every node’s left subtree contains values less than or equal to the node, and its right subtree contains values greater than the node. This defers from the min-heap property which only says that a parent node should have a value less than or equal to its children. 

###### Can the min-heap property be used to print out the keys of an n-node tree in sorted order in O(n) time? (Note: Should be understood to include a requirement that the tree not be modified.)

No we cannot guarantee that the min-heap property can be used to print out keys of an n-node tree in sorted order in O(n) time. This is because the min-heap property only ensures that the parent node is greater than its child nodes, it does not force a sorted ordering of the child nodes.

###### Show how, or explain why not.

        1
       / \
      5   3
     / \
    8  10

This example maintains the min-heap property but we see that child nodes 5 and 3 are not in sorted order while child nodes 8 and 10 are. 


### Chapter 12.2 Exercise 

##### Professor Kilmer claims to have discovered a remarkable property of binary search trees.

###### Suppose that the search for key k in a binary search tree ends up at a leaf. Consider three sets: 

a. A, the keys to the left of the search path
b. B, the keys on the search path
c. C, the keys to the right of the search path. 

###### Professor Kilmer claims that any three keys a in A, b in B, and c in C must satisfy a le b le c. Give a smallest possible counterexample to the professor's claim. (Note: Your counterexample should be in the form of a specific binary search tree T and key k.)
       
       10
      /  \
     5    15
    / \
   1   8

Insertion Order: {10,5,15,1,8}

Search for 1:

Path A: {}
Path B: {10,5,1}
Path C: {8,15}

Given the above key T, when searching for k: 1 - From set C's: 8 violates the property as it is less than set B's: 10.


### Chapter 12.3 Exercise 

##### Is the operation of deletion "commutative" in the sense that deleting x and then y from a binary search tree leaves the same tree as deleting y and then x? Argue why it is or give a counterexample.

The deletion operation on a BST is commutative, this is due to deletion being deterministic based only on key comparisons, and each deletion depends only on the keys present at that moment.

**Example:**

Tree:
      5
     / \
    3   8
   / \
  2   4

**Case 1:**

After Removing 3:

    5
   / \
  4   8
 /
2

After Removing 4:

    5
   / \
  2   8

**Case 2:**

After Removing 4:

    5
   / \
  3   8
 /
2

After Removing 3:

    5
   / \
  2   8


### Chapter 13.1 Exercise

##### Describe a red-black tree on n keys that realizes the largest possible ratio of red internal nodes to black internal nodes. What is this ratio? What tree has the smallest possible ratio, and what is the ratio?

###### Describe a red-black tree on n keys that realizes the largest possible ratio of red internal nodes to black internal nodes. What is this ratio? 

A perfectly balanced red–black tree where every black node has two red children (and red nodes have only black children). The maximum posible ratio of red internal nodes to black internal nodes is a 1:1 ratio. 

###### What tree has the smallest possible ratio, and what is the ratio?

The smallest possible ratio is 0: The situation where a RB tree has 0 red nodes.

An example of such tree.

        10(B)
       /    \
    6(B)    16(B)
   /  \     /  \
  2(B) 7(B)12(B)19(B)


### Chapter 13.3 Exercise

##### Show the red-black trees that result after successively inserting the keys 41, 38, 31, 12, 19, 8 into an initially empty red-black tree.

# RB Tree Rules

1. **Color Property**: Every node is either **Black** or **Red**.  
2. **Root Property**: The root is always **Black**.  
3. **Red Property**: No red node has a red child.  
4. **Black-Height Property**: Every path from a node to its descendant leaves contains the same number of black nodes.  
5. **Leaf Property**: All leaves (NIL/null nodes) are considered **Black**.

1. Insert 41 (root must be Black)

        41(B)

2. Insert 38 (to left and as red)

        41(B)
       / 
    38(R)

3. Insert 31 (to left and as red)

Violation: Two consecutive reds: Uncle is nil - Case 2.

        41(B)
       / 
    38(R)
     /
    31(R)

3a. Right Rotation on 41

        38(B)
       /    \
    31(R)  41(R)

4. Insert 12 (to left and as red)

Violation: Two consecutive reds: Parent and Uncle Red - Case 1.

        38(B)
       /    \
    31(R)  41(R)
    /
 12(R)

4a. Recolor Parent and Uncle Black, Grandparent Red (root so keep it black)

        38(B)
       /    \
    31(B)  41(B)
    /
 12(R)

5. Insert 19 (left, right then as red)

        38(B)
       /    \
    31(B)  41(B)
    /   \
 12(R) 19(R)

6. Insert 8 (left, left, left, then as red)

Violation: Two consecutive reds: Parent and Uncle Red: Case 1

        38(B)
       /    \
    31(B)  41(B)
    /   \
 12(R) 19(R)
  /
8(R)

6a. Recolor Parent and Uncle Black, Grandparent Red (root so keep it black)

        38(B)
       /    \
    31(R)  41(B)
    /   \
 12(B) 19(B)
  /
8(R)


### Chapter 13.4 Exercise

##### A node x is inserted into a red-black tree with RB-INSERT and then is immediately deleted with RB-DELETE. Is the resulting red-black tree always the same as the initial red-black tree? Justify your answer.

