use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/* Sample data:

1928_Okeechobee_Hurricane   United_States
1928_Okeechobee_Hurricane   United_States_Virgin_Islands
1973_oil_crisis Algeria
1973_oil_crisis Arab-Israeli_conflict
1973_oil_crisis Australia

*/

//we will store our graph in the struct Graph, in forms of adjacency lists
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug, PartialEq)]
pub struct Graph {
    pub n: usize, 
    pub outedges: AdjacencyLists,
}

fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
}

//Load data from links.tsv into a graph. The tsv represents a directed graph, each row has node and a node pointed to; Every node is an Wikipedia entry
pub fn load_tsv(file_name: &str) -> std::io::Result<(HashMap<usize, String>, Graph, Graph)> {
    let mut nodes: HashMap<String, usize> = HashMap::new(); //the hashmap nodes keeps track of the name(wikipedia entry) of the node
    let mut node_set: HashSet<String> = HashSet::new(); //node_set keeps track of unique nodes 
    let mut edges: ListOfEdges = vec![]; 

    // First loop: build nodes, count the number of nodes
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut num_nodes = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().starts_with('#') || line.trim().is_empty() {//ignores the comments on top of the file
            continue;
        }

        let parts: Vec<&str> = line.trim().split('\t').collect(); 
        let from = parts[0].to_string(); //starting node
        let to = parts[1].to_string(); //target node

        //expand nodes, noteset and num_nodes if is an unseen node
        //we use num_nodes(which = i, where the current node the ith unseen node) as the indices for nodes
        if !node_set.contains(&from) {

            //           ⬇name of the node, ⬇index of the node
            nodes.insert(from.clone(), num_nodes);
            node_set.insert(from.clone());
            num_nodes += 1;
        }

        if !node_set.contains(&to) {
            nodes.insert(to.clone(), num_nodes);
            node_set.insert(to.clone());
            num_nodes += 1;
        }
    }

    // Second loop: build list of edges
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim().split('\t').collect();
        let from = parts[0].to_string();
        let to = parts[1].to_string();

        //get index of the node from the hashmap "nodes"
        let from_index = *nodes.get(&from).unwrap();
        let to_index = *nodes.get(&to).unwrap();

        edges.push((from_index,to_index))
    }

    let graph = Graph::create_directed(num_nodes, &edges);

    //we return reversed graph for later use
    let graph_reverse = Graph::create_directed(num_nodes,&reverse_edges(&edges));

    //the nodes in our graph is represented by their index; reverse the hashmap nodes so that later we can look up the name of the nodes by their indices
    let nodes_reverse: HashMap<usize, String> = nodes.into_iter().map(|(k, v)| (v, k)).collect();

    Ok((nodes_reverse, graph,graph_reverse))
}

#[test]
fn test_reader() {
    let (nodes, graph, graph_reverse) = load_tsv("testdata.tsv").expect("Error loading file");
    assert!(nodes.len() == 7);
    assert!(graph.n == 7);

    let correct_graph = Graph::create_directed(7, &vec![(0,1),(1,2),(2,0),(3,4),(4,5),(5,3),(2,3),(6,5)]);
    assert_eq!(graph, correct_graph);

    let correct_graph_reverse = Graph::create_directed(7, &vec![(1,0),(2,1),(0,2),(4,3),(5,4),(3,5),(3,2),(5,6)]);
    assert_eq!(graph_reverse, correct_graph_reverse);

   
}