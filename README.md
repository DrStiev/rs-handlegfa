# RS-HANDLEGFA
rs-handlegfa it's a tool to manipulate GFA files as a graph.
This tool use 2 libraries to make this possible:
- rs-gfa2 that is used for parsing a file and check if it's correct for the chosen format. [link here](https://github.com/DrStiev/rs-gfa2)
- rs-handlegraph2 that is used to create the hashgraph associated with a GFA object. [link here](https://github.com/DrStiev/rs-handlegraph2)

## HOW TO RUN THIS TOOL
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

## HOW IT WORKS
HandleGFA performs three main tasks while running: 
1. Control wheter the file is comform to the format GFA1 or GFA2 and create the associated HashGraph
2. Manipulate the graph through 3 main operation:
   - ADD Operation: such as add nodes, links between them and paths
   - REMOVE Operation: such as remove nodes, links between them and paths
   - MODIFY Operation: such as modify nodes, links between them and paths
3. And at last, save the resulting graph back as a GFA file

## UI
After run the program the first thing shown is the graph obtained from the file passed as input.\
After that it's show the possible operation that can be done on the graph.
> *The graph will be shown only if the file is smaller than 10KB*\
**REMEMBER**, displaying the graph is done only to make easier the later operation on it. But if the file it's bigger than 10KB generally creates a graph not easier to read by a human.\
So even if the file it's relative small, it's better to not display it.

```
Graph: {
        Nodes: {
                13: CTTGATT
                12: TCAAGG
                11: ACCTT
        }
        Edges: {
                12- --> 13+
                11+ --> 12-
                11+ --> 13+
        }
        Paths: {
                14: ACCTT -> CCTTGA -(TCAAGG) -> CTTGATT
                15: ACCTT -> CTTGATT
        }
}

The possible operation on a graph are:
1. Add Node(s), Link(s) [or Edge(s)] and Path(s)
2. Remove Node(s), Link(s) [or Edge(s)] and Path(s)
3. Modify the value of Node(s), Link(s) [or Edge(s)] and Path(s)

To STOP modifying the graph, or STOP perform a certain operation type [STOP] (case insensitive)

To ADD an element to the graph type: ADD [NODE|LINK|PATH] (case insensitive)

To REMOVE an element to the graph type: REMOVE [NODE|LINK|PATH] (case insensitive)

To MODIFY an element to the graph type: MODIFY [NODE|LINK|PATH] (case insensitive)
```
After chose an operation, it will be displayed what information need to be inserted to make the operation come true, for example: 
```
add node

To ADD a NODE into the graph, please type [NODEID] [SEQUENCE|*] where:
[NODEID] is the new id of the node (always a number, otherwise an error will be raised)     
[SEQUENCE|*] is the new sequence of the node. The character "*" represent that the sequence 
it's not provided.
The 2 elements MUST BE separated by a SINGLE whitespace.
```
After done with the first instruction it will be displayed the resulting graph with the message associated to the last operation done.
```
42 IT_IS_NOT_THE_ANSWER

Graph: {
        Nodes: {
                13: CTTGATT
                12: TCAAGG
                11: ACCTT
                42: IT_IS_NOT_THE_ANSWER
        }
        Edges: {
                12- --> 13+
                11+ --> 12-
                11+ --> 13+
        }
        Paths: {
                14: ACCTT -> CCTTGA -(TCAAGG) -> CTTGATT
                15: ACCTT -> CTTGATT
        }
}

To ADD a NODE into the graph, please type [NODEID] [SEQUENCE|*] where:
[NODEID] is the new id of the node (always a number, otherwise an error will be raised)     
[SEQUENCE|*] is the new sequence of the node. The character "*" represent that the sequence 
it's not provided.
```
After typing "STOP" the program will exit the current inner branch to the first outer branch available, if there's none, the program will enter the exit state
```
stop

To ADD an element to the graph type: ADD [NODE|LINK|PATH] (case insensitive)

To REMOVE an element to the graph type: REMOVE [NODE|LINK|PATH] (case insensitive)

To MODIFY an element to the graph type: MODIFY [NODE|LINK|PATH] (case insensitive)

To STOP modifying the graph, or STOP perform a certain operation type [STOP] (case insensitive)

stop

Do you want to save the changes?
``` 
Type "Y" or "YES" to trigger the save procedure 
```
y
Specify the path where to save the file or the input file will be overwritten.
"*" is the character to use to not specify any path and so overwritten the input file.      
the whitespace character is used to not specify any path and so use the default path where to save the file.
```
"N" or "NO" to exit without save
```
n
File not saved!
Program terminated correctly!
```
everything else to exit without saving with an "Unrecognized input" as message
```
42
Command not recognized!
Program terminated and file not saved!
```
