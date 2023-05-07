# DS210_Final_Project

Hello, this is Heng Chang's final project for DS210. The folder"wikispeedia_project" (Only 3 MB) is the full project. You can download it and run cargo test or cargo run right away (Takes < 2 minutes on my machine). Please see the report for more details, Thank you!

- \wikispeedia_project: The main folder containing the entire project.
    - Cargo.toml
    - testdata.tsv: Data file used for tests.
    - links.tsv: Data file containing the graph data for the project.
    -  \src
        - main.rs: Main Rust file of the project.
        - reader.rs: mod for loading the graph data.
        - distribution.rs: mod for analyzing the graph's degree distribution.
        - seperation.rs: mod for analyzing the graph's degree of seperation.
