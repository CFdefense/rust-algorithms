#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EdgeClass {
    Tree,
    Back,
    Forward,
    Cross,
}

#[derive(Clone, Debug)]
struct EdgeInfo {
    source: usize,
    target: usize,
    class: EdgeClass,
}

#[derive(Debug)]
struct DfsResult {
    discovery_time: Vec<usize>,
    finish_time: Vec<usize>,
    parent: Vec<Option<usize>>,
    edges: Vec<EdgeInfo>,
    is_cyclic: bool,
}

#[derive(Debug)]
struct Graph {
    node_count: usize,
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(num_vertices: usize) -> Self {
        Self { node_count: num_vertices, adj_list: vec![Vec::new(); num_vertices] }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.node_count && v < self.node_count, "vertex out of range");
        self.adj_list[u].push(v);
    }
}

fn dfs(graph: &Graph) -> DfsResult {
    let n = graph.node_count;
    let mut color = vec![Color::White; n];
    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut discovery_time = vec![0usize; n];
    let mut finish_time = vec![0usize; n];
    let mut edges: Vec<EdgeInfo> = Vec::new();
    let mut time: usize = 0;
    let mut is_cyclic = false;

    fn visit(
        u: usize,
        graph: &Graph,
        color: &mut [Color],
        parent: &mut [Option<usize>],
        discovery_time: &mut [usize],
        finish_time: &mut [usize],
        edges: &mut Vec<EdgeInfo>,
        time: &mut usize,
        is_cyclic: &mut bool,
    ) {
        *time += 1;
        discovery_time[u] = *time;
        color[u] = Color::Gray;

        for &v in &graph.adj_list[u] {
            match color[v] {
                Color::White => {
                    edges.push(EdgeInfo { source: u, target: v, class: EdgeClass::Tree });
                    parent[v] = Some(u);
                    visit(
                        v,
                        graph,
                        color,
                        parent,
                        discovery_time,
                        finish_time,
                        edges,
                        time,
                        is_cyclic,
                    );
                }
                Color::Gray => {
                    // Edge to an ancestor in the DFS tree → back edge.
                    edges.push(EdgeInfo { source: u, target: v, class: EdgeClass::Back });
                    *is_cyclic = true;
                }
                Color::Black => {
                    // Finished vertex: forward if v is a descendant of u; else cross.
                    let class = if discovery_time[u] < discovery_time[v] {
                        EdgeClass::Forward
                    } else {
                        EdgeClass::Cross
                    };
                    edges.push(EdgeInfo { source: u, target: v, class });
                }
            }
        }

        color[u] = Color::Black;
        *time += 1;
        finish_time[u] = *time;
    }

    for u in 0..n {
        if color[u] == Color::White {
            visit(
                u,
                graph,
                &mut color,
                &mut parent,
                &mut discovery_time,
                &mut finish_time,
                &mut edges,
                &mut time,
                &mut is_cyclic,
            );
        }
    }

    DfsResult { discovery_time, finish_time, parent, edges, is_cyclic }
}

fn main() {
    // Example Directed Graph
    let mut g = Graph::new(8);
    // 0:A, 1:B, 2:C, 3:D, 4:E, 5:F, 6:G, 7:H
    g.add_edge(0, 1); // A→B
    g.add_edge(1, 2); // B→C
    g.add_edge(2, 0); // C→A (creates a cycle among A-B-C)
    g.add_edge(2, 3); // C→D
    g.add_edge(3, 4); // D→E
    g.add_edge(4, 5); // E→F
    g.add_edge(5, 3); // F→D (another cycle)
    g.add_edge(6, 7); // G→H (disconnected component)
    g.add_edge(0, 4); // A→E (forward or cross depending on traversal)

    let result = dfs(&g);

    if result.is_cyclic {
        println!("Graph is cyclic");
    } else {
        println!("Graph is acyclic");
    }

    for e in &result.edges {
        let class_str = match e.class {
            EdgeClass::Tree => "tree",
            EdgeClass::Back => "back",
            EdgeClass::Forward => "forward",
            EdgeClass::Cross => "cross",
        };
        println!("{} -> {} : {}", e.source, e.target, class_str);
    }

    // Display final results 
    for u in 0..result.discovery_time.len() {
        let parent_str = match result.parent[u] {
            Some(p) => p.to_string(),
            None => String::from("nil"),
        };
        println!(
            "v{}: d={}, f={}, parent={}",
            u, result.discovery_time[u], result.finish_time[u], parent_str
        );
    }
}
