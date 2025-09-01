## Week 1 Graph Representations

### Chapter 20.1 Exercise

1. Given an adjacency-list representation of a directed graph, how long does it take to compute the out-degree of every vertex?

**Response:**
It takes O(V + E) where V is the number of vertices and E is the number of edges, this is due to the fact that we must visit each vertex and then each node in its adjacency list to compute the out degree.

2. How long does it take to compute in-degrees?

**Response:**
It takes O(V + E) aswell to compute the total in-degrees because in order to count all the in-degrees we must traverse each vertex (V) and each edge (E) in its adjacency list.


### Extra-CLRS Excersises

1. For each set of properties described below, draw an example graph (8-12 vertices is sufficient) either by hand (then submit a photo) or using a drawing/diagram app (e.g., drawio.com and then upload exported image).

    a. Undirected graph
        ![Undirected Graph](Resources/Und_Graph.png)

    b. Directed acyclic graph
        ![Undirected Graph](Resources/Dir_Graph.png)

    c. Weighted, directed graph with one or more cycles
        ![Undirected Graph](Resources/Wei_Graph.png)

2. For each one of the graphs you drew above, answer the following questions:

    a. Are the graphs you drew sparse or dense? Explain.

    **Response:** The graphs are all sparse, this is because the the number of edges is far from the maximum total of V<sup>2</sup>.

    b. Is the graph connected? What is the fewest number of connections for any one of the vertices? What is the maximum number of connections for any of the vertices?

    **Response:**

        a. The undirected graph is connected, we can reach any node from any node. If the graph is connected the fewest connections must be 1, the maximum can be n-1 where n is the number of vertices.

        b. The directed acyclic graph is not connected, some nodes cannot be reached from others. For the graph to be connected the fewest connections must be 2 such that we can go to and from each vertice. The maximum is 2(n-1) where n is the number of vertices.

        c. The weighted cyclic graph is not connected, some nodes cannot be reached from others. For the graph to be connected each node must again have atleast 2 edges, and the maximum connections is 2(n-1) where n is the number of vertices.

    c. What real-world scenario/relationships could this kind of graph model?

        a. The undirected graph can be found in mutual connection social networks such as linkedin where each party must follow eachother therefore creating a bidirectional edge.

        b. The directed acyclic graph can be found in family trees (assuming nothing weird is going on). 

        c. The Weighted directed and cyclic graph can be found in transportation systems, a prime example of the famous traveling salesman problem.

3. Choose one of your sample graphs and provide representations of that graph in the form of both an adjacency matrix and dictionary-of-adjacency lists, using the style of Python syntax.

ex: 

adjMatrix = [[0,1,1],[1,0,1],[1,1,0]]
adjLists = {"A":["B","C"], "B":["A","C"], "C":["A","B"]}

**Response:**

Ill use the undirected graph

adjMatrix = [[0,0,1,0,0,0,0,0],[0,0,1,0,0,0,0,0],[1,1,0,1,1,0,1,0],[0,0,1,0,0,0,0,0],[0,0,1,0,0,1,0,0],[0,0,0,0,1,0,0,1],[0,0,1,0,0,0,0,0],[0,0,0,0,0,1,0,0]]

adjLists = {"A":["C"], "B":["C"], "C":["A","B","D","E","G"], "D":["C"], "E":["C","F"], "F":["E","H"], "G":["C"], "H":["F"]}
