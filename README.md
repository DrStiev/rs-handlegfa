# RS-HANDLEGFA
rs-handlegfa it's a tool to manipulate GFA files as a graph.
This tool use 2 libraries to make this possible:
- rs-gfa2 that is used for parsing a file and check if it's correct for the chosen format. [link here](https://github.com/DrStiev/rs-gfa2)
- rs-handlegraph2 that is used to create the hashgraph associated with a GFA object. [link here](https://github.com/DrStiev/rs-handlegraph2)

# HOW TO RUN THIS TOOL
First, clone this repo and move to the resulting directory:
```
git clone https://github.com/DrStiev/rs-handlegfa
cd rs-handlegfa
```

Then compile the program:
```
cargo build --release
```

To run the program use the following command:

- To manipulate a GFA1 file: ``` cargo run --release {input_file.gfa} ```
- To manipulate a GFA2 file: ``` cargo run --release {input_file.gfa2} ```

# HOW IT WORKS
HandleGFA performs three main tasks while running: 
1. Control wheter the file is comform to the format GFA1 or GFA2 and create the associated HashGraph
2. Manipulate the graph through 3 main operation:
   - ADD Operation: such as add nodes, links between them and paths
   - REMOVE Operation: such as remove nodes, links between them and paths
   - MODIFY Operation: such as modify nodes, links between them and paths
3. And at last, save the resulting graph back as a GFA file