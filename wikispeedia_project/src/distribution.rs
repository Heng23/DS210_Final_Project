use crate::reader::Graph;
use plotters::prelude::*;
use std::collections::HashMap;

// return a hashmap of the number of outedges; key = number of edges a node has, value = frequency of such nodes
fn calculate_degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_distribution = HashMap::new();

    for edges in &graph.outedges {
        let degree = edges.len();

        *degree_distribution.entry(degree).or_insert(0) += 1;//frequency += 1 if a node with [degree] many outedges appear
    }

    degree_distribution
}

//use crate plotter to plot the distribution of in/out edges(outedges when input graph is original, inedges when input graph is reversed)
pub fn plot_degree_distribution(graph: &Graph, direction: &str) -> Result<(), Box<dyn std::error::Error>> {
    //set names for the plot
    let (file_name, plot_title, xdesc);
    match direction {
        "Out" => {
            file_name = "OutDegree_distribution.png";
            plot_title = "OutDegree Distribution";
            xdesc = "Number of OutEdges";
        }
        "In" => {
            file_name = "InDegree_distribution.png";
            plot_title = "InDegree Distribution";
            xdesc = "Number of InEdges";
        }
        _ => return Err("Invalid direction".into())
    }

    //get the hashmap for degree distribution and initialize variables used for plotting configs
    let degree_distribution = calculate_degree_distribution(graph);
    let max_degree = degree_distribution.keys().max().unwrap();
    let max_count = degree_distribution.values().max().unwrap();

    //plotting configs
    let root = BitMapBackend::new(&file_name, (1920 , 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&plot_title, ("sans-serif", 40).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..*max_degree + 10,
            0..*max_count + 10,
        )?;

    chart
        .configure_mesh()
        .x_labels(50)
        .y_labels(50)
        .y_desc("Frequency")
        .x_desc(xdesc)
        .axis_desc_style(("sans-serif", 40))
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    //plot a histogram from the hashmap
    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(0)

            .data(
                //transform the elements of the hashmap into tuples for plotter to plot
                degree_distribution.iter().map(|(degree, count)| (*degree, *count))
            ),
    )?;

    Ok(())
}

//prints the top nodes with most in/out edges(outedges when input graph is original, inedges when input graph is reversed)
pub fn print_top_nodes(graph: &Graph, nodes: &HashMap<usize, String>, top_n: usize, direction: &str) -> Vec<usize> {

    let mut top_edge_counts = vec![]; // the biggest counts of #of in/out going edges; mainly for test 


    //sort the nodes(indices) by the number of outedges
    let mut node_indices: Vec<usize> = (0..graph.n).collect();
    node_indices.sort_unstable_by_key(|&i| graph.outedges[i].len());

    println!("Top {} nodes with most {}edges:", top_n,direction);
    for i in node_indices.iter().rev().take(top_n) {

        //get the corresponding wikipedia entry from the hashmap nodes with the node's index
        println!("{}: {}", *nodes.get(&i).unwrap(), graph.outedges[*i].len());

        top_edge_counts.push(graph.outedges[*i].len());

    }
    println!("\n");
    top_edge_counts

}



   
