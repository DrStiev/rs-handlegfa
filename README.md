# RS-HANDLEGFA
rs-handlegfa it's a tool to manipulate GFA files as graphs.
This tool use 2 libraries to make this possible:
- rs-gfa2 that is used for parsing a file and check if it's correct for the chosen format. [link here](https://github.com/DrStiev/rs-gfa2)
- rs-handlegraph2 that is used to create the hashgraph associated with a GFA object. [link here](https://github.com/DrStiev/rs-handlegraph2)

## HOW IT WORKS
HandleGFA performs three main tasks while running: 
1. Control wheter the file is comform to the format GFA1 or GFA2 and create the associated HashGraph
2. Manipulate the graph through 3 main operation:
   - ADD Operation: such as add nodes, links between them and paths
   - REMOVE Operation: such as remove nodes, links between them and paths
   - MODIFY Operation: such as modify nodes, links between them and paths
3. And at last, save the resulting graph back as a GFA file

## Usage