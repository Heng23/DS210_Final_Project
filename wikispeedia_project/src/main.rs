mod reader;
mod distribution;
mod seperation;

fn main(){

    println!("Loading File...\n");
    let (nodes, graph, graph_reverse) = reader::load_tsv("links.tsv").expect("Error loading file");
    println!("File loaded\n");

    println!("Investigating OutDegree...\n");
    match distribution::plot_degree_distribution(&graph, "Out",) {
        Ok(_) => println!("OutDegree distribution plotted\n"),
        Err(err) => println!("Error plotting OutDegree distribution: {:?}", err),
    }

    distribution::print_top_nodes(&graph,&nodes,15,"out");

    println!("Investigating InDegree...\n");
    match distribution::plot_degree_distribution(&graph_reverse, "In") {
        Ok(_) => println!("InDegree distribution plotted\n"),
        Err(err) => println!("Error plotting InDegree distribution: {:?}", err),
    }

    distribution::print_top_nodes(&graph_reverse,&nodes,15,"in");


    println!("Investigating Seperation...\n");
    let (mean, median, max, std_deviation) = seperation::distance_distribution(&graph);

    println!("Mean distance: {}", mean);
    println!("Median distance: {}", median);
    println!("Max distance: {}", max);
    println!("Standard deviation of distance: {}", std_deviation);
    println!("Completed");
 
    }





#[test]
fn test_top_nodes_outedge() { 
    let (nodes, graph, graph_reverse) = reader::load_tsv("testdata.tsv").expect("Error loading file");
    let result_out = distribution::print_top_nodes(&graph,&nodes,7,"out");
    let result_in = distribution::print_top_nodes(&graph_reverse,&nodes,7,"in");
    let correct_out:Vec<usize> = [2,1,1,1,1,1,1].to_vec();
    let correct_in:Vec<usize> = [2,2,1,1,1,1,0].to_vec();

    assert_eq!(correct_in, result_in);
    assert_eq!(correct_out, result_out);

}


#[test]
fn test_plotting_degree_distribution() { 
    let (nodes, graph, graph_reverse) = reader::load_tsv("testdata.tsv").expect("Error loading file");

    match distribution::plot_degree_distribution(&graph, "Out",) {
        Ok(_) => println!("OutDegree distribution plotted\n"),
        Err(err) => panic!("Error plotting OutDegree distribution: {:?}", err),
    }

    match distribution::plot_degree_distribution(&graph_reverse, "In") {
        Ok(_) => println!("InDegree distribution plotted\n"),
        Err(err) => panic!("Error plotting InDegree distribution: {:?}", err),
    }

}

#[test]
fn test_distance_distribution_and_plotting_distance_distribution() { 
    let (nodes, graph, graph_reverse) = reader::load_tsv("testdata.tsv").expect("Error loading file");
    let (mean, median, max, std_deviation) = seperation::distance_distribution(&graph);
    assert_eq!(max, 5);
    assert_eq!(median, 2.0);
    assert!(mean - 2.333334 < 0.001);
  
}




