use crate::reader::Graph;
use std::collections::VecDeque;
use plotters::prelude::*;

//BFS search for investigating degrees of seperation
fn bfs(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    
    let mut distance = vec![None; graph.n];
    distance[start] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }

    
    distance //returns the shortest distance from start node to all nodes in the graph
}

pub fn distance_distribution(graph: &Graph) -> (f64, f64,usize,f64) {
    let num_nodes = graph.n;

    
    let mut distances: Vec<usize> = Vec::new(); // Will store the shortest distance between all pairs of nodes(nC2)


    for i in 0..num_nodes {// iterates over all nodes in the graph
        let dist_from_i = bfs(graph, i); //the shortest distance from node i to all nodes in the graph
        for j in i + 1..num_nodes {// iterates over all nodes in the graph that has not been paired with node i yet

            if let Some(dist) = dist_from_i[j] {
                distances.push(dist);// if the target node is reachable from node i, push the distance between this pair into distance

            }
        }
    }

    //plot the distribution
    match plot_distance_distribution(distances.clone()) {
        Ok(_) => println!("Distance distribution plotted\n"),
        Err(err) => println!("Error plotting distance distribution: {:?}", err),
    }

    // calculate descriptive statistics
    distances.sort_unstable();

    let len = distances.len();
    let sum: usize = distances.iter().sum();
    let mean = sum as f64 / len as f64;
    let max = distances.last().unwrap();


    let median = if len % 2 == 0 {
        distances[len / 2 - 1] as f64 
    } else {
        distances[len / 2] as f64
    };

    let variance = distances.iter().map(|d| {let diff = *d as f64 - mean; diff * diff}).sum::<f64>() / len as f64;
    let std_deviation = variance.sqrt();

    
    (mean, median, *max, std_deviation)
}


fn plot_distance_distribution(distances: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {

    //plotting configs
    let root = BitMapBackend::new("Distance_distribution.png", (1920 , 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Distance between nodes distribution", ("sans-serif", 40).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(

            0..*distances.iter().max().unwrap() + 1,
            0..distances.len(),
        )?;

    chart
        .configure_mesh()
        .x_labels(50)
        .y_labels(50)
        .y_desc("Frequency")
        .x_desc("Distance")
        .axis_desc_style(("sans-serif", 40))
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    //plot a histogram from the distances vector
    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(0)

            .data( distances.iter().map(|x| (*x, 1)),
                
            ),
    )?;

    Ok(())
}