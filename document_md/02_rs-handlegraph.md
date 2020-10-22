# [RS HANDLEGRAPH](https://github.com/chfi/rs-handlegraph)

## File [handle.rs](https://github.com/chfi/rs-handlegraph/blob/master/src/handle.rs):
Fornisce dei nuovi type per la creazione e gestione di un grafo:
1. ```NodeId```: questo nuovo type permette di rappresentare un nodo dentro un grafo senza preoccuparsi del 
    tipo di implementazione di quest'ultimo.  
    ```rust
    pub struct NodeId(u64);

    impl Add<u64> for NodeId {
        type Output = Self;
        fn add(self, other: u64) -> Self {
            NodeId(self.0 + other)
        }
    }
    ```
    NodeId implementa delle conversioni di tipo per i dati utilizzati come parametri per l'inizializzazione.\
    Ad esempio:
    ```rust
    impl From<NodeId> for u64 {
        fn from(id: NodeId) -> Self {
            id.0
        }
    }

    impl From<i32> for NodeId {
        fn from(num: i32) -> Self {
            NodeId(num as u64)
        }
    }
    ```
2. ```Handle```: Handle rappresenta un Node Id con un orientamento noto, utilizzato come singolo ```u64```.
    ```rust
    pub struct Handle(pub u64);

    impl Handle {
        ...
        pub fn new<T: Into<NodeId>>(id: T, orient: Orientation) -> Handle {
            let id: NodeId = id.into();
            let uint: u64 = id.into();
            let is_reverse = orient != Orientation::Forward;
            if uint < (0x1 << 63) {
                Handle::from_integer((uint << 1) | is_reverse as u64)
            } else {
                panic!("Tried to create a handle with a node ID that filled 64 bits")
            }
        }

        pub fn pack<T: Into<NodeId>>(id: T, is_reverse: bool) -> Handle {
            let id: NodeId = id.into();
            let uint: u64 = id.into();
            if uint < (0x1 << 63) {
                Handle::from_integer((uint << 1) | is_reverse as u64)
            } else {
                panic!("Tried to create a handle with a node ID that filled 64 bits")
            }
        }
        ...
    }
    ```
    Handle implementa delle conversioni di tipo per i dati utilizzati come parametri per l'inizializzazione.\
    Ad esempio:
    ```rust
    impl From<i32> for Handle {
        fn from(i: i32) -> Self {
            Handle::from_integer((i << 1) as u64)
        }
    }

    impl Handle {
        ...
        pub const fn from_integer(i: u64) -> Self {
            Handle(i)
        }
        ...
    }
    ```
3. ```Edge```: viene costruito un Edge, ovvero un angolo di connessione tra 2 nodi, prendendo come partenza
    l'orientamento degli ```Handles``` in input.\
    Vengono comparati i 2 ```Handle``` passati in input per capire come ordinarli per creare un angolo di connessione \
    tra i 2 nodi che andranno a costituire il grafo.
    ```rust
    pub struct Edge(pub Handle, pub Handle);

    impl Edge {
        /// Construct an edge, taking the orientation of the handles into account
        pub fn edge_handle(left: Handle, right: Handle) -> Edge {
            let flipped_right = right.flip();
            let flipped_left = left.flip();

            match left.cmp(&flipped_right) {
                Ordering::Greater => Edge(flipped_right, flipped_left),
                Ordering::Equal => {
                    if right > flipped_left {
                        Edge(flipped_right, flipped_left)
                    } else {
                        Edge(left, right)
                    }
                }
                Ordering::Less => Edge(left, right),
            }
        }
    }
    ```
4. ```Direction```: viene aggiunto un ```enumeratore``` per gestire l'orientamento dei vari nodi nel grafo.
    ```rust
    pub enum Direction {
        Left,
        Right,
    }
    ```

## File [handlegraph](https://github.com/chfi/rs-handlegraph/blob/master/src/handlegraph.rs):
Fornisce dei ```traits``` generici che incapsulano gli aspetti fondamentali di un **Handlegraph**.\
Generalmente queste funzioni permettono di ottenere delle informazioni importanti riguardo la topografia 
del grafo che si sta valutando e alcuni aspetti degli elementi che lo compongono.
1. Informazioni riguardo la ```topologia``` del grafo.\
    Ad esempio:
    ```rust
    ...
    /// The length of the sequence of a given node
    fn length(&self, handle: Handle) -> usize;

    /// Returns the sequence of a node in the handle's local forward
    /// orientation. Copies the sequence, as the sequence in the graph
    /// may be reversed depending on orientation.
    fn sequence(&self, handle: Handle) -> Vec<u8>;

    /// Return the total number of nodes in the graph
    fn node_count(&self) -> usize;

    /// Return the total number of edges in the graph
    fn edge_count(&self) -> usize;
    ...
    ```
2. Dati per l'```attraversamento e manipolazione``` del grafo.\
    Ad esempio:
    ```rust
    /// Returns an iterator over all the handles in the graph
    fn handles_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Handle> + 'a>;

    /// Returns an iterator over all the edges in the graph
    fn edges_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Edge> + 'a>;
    ```

## File [mutablehandlegraph](https://github.com/chfi/rs-handlegraph/blob/master/src/mutablehandlegraph.rs):
Descrive i traits che codificano l'aspetto mutabile di un ```handlegraph```.\
Quest'API e' ancora acerba e implementa solamente pochi traits utili alla gestione della 
mutabilita' di un ```handlegraph```.

## File [pathgraph](https://github.com/chfi/rs-handlegraph/blob/master/src/pathgraph.rs):
Incapsula i ```traits``` che descrivono l'immutabilita' dei ```Path``` in un handlegraph.
1. Creazione di un percorso:
    ```rust
    fn create_path_handle(
        &mut self,
        name: &[u8],
        is_circular: bool,
    ) -> Self::PathHandle;
    ```
2. Creazione nuovi tipi per la gestione di un percorso:
    ```rust
    /// A handle to a path in the graph, can also be viewed as a path identifier
    type PathHandle;
    /// A handle to a specific step on a specific path in the graph
    type StepHandle;
    ```
3. Ottenimento informazioni di un percorso:
    ```rust
    ...
    /// Get the first step of the path
    fn path_begin(&self, path_handle: &Self::PathHandle) -> Self::StepHandle;

    /// Get the last step of the path
    fn path_end(&self, path_handle: &Self::PathHandle) -> Self::StepHandle;

    fn next_step(&self, step_handle: &Self::StepHandle) -> Self::StepHandle;

    fn previous_step(&self, step_handle: &Self::StepHandle) -> Self::StepHandle;
    ...
    ```
4. Controllo informazioni di un percorso:
    ```rust
    ...
    fn has_path(&self, name: &[u8]) -> bool;

    fn is_circular(&self, handle: &Self::PathHandle) -> bool;

    fn has_next_step(&self, step_handle: &Self::StepHandle) -> bool;

    fn has_previous_step(&self, step_handle: &Self::StepHandle) -> bool;
    ...
    ```

## File [hashgraph](https://github.com/chfi/rs-handlegraph/blob/master/src/hashgraph.rs):
Permette la creazione vera e propria del ```Handlegraph``` che viene codificato tramite una ```HashMap```, da qui il nome ```HashGraph```.
```rust
pub struct HashGraph {
    pub max_id: NodeId,
    pub min_id: NodeId,
    pub graph: HashMap<NodeId, Node>,
    pub path_id: HashMap<Vec<u8>, i64>,
    pub paths: HashMap<i64, Path>,
}
```
Un HashGraph permette sia la creazione di un grafo da zero, 
inserendo quindi uno alla volta i vari ```segment```, ```link``` e ```path``` 
che definiscono la topologia del grafo, 
sia utilizzando un file in formato ```GFA1``` come base di partenza.
```rust
impl HashGraph {
    pub fn new() -> HashGraph {
        HashGraph {
            max_id: NodeId::from(0),
            min_id: NodeId::from(std::u64::MAX),
            ..Default::default()
        }
    }

    fn add_gfa_segment<'a, 'b, T: OptFields>(
        &'a mut self,
        seg: &'b Segment<usize, T>,
    ) {
        self.create_handle(&seg.sequence, seg.name as u64);
    }

    fn add_gfa_link<T: OptFields>(&mut self, link: &Link<usize, T>) {
        let left = Handle::new(link.from_segment as u64, link.from_orient);
        let right = Handle::new(link.to_segment as u64, link.to_orient);

        self.create_edge(&Edge(left, right));
    }

    fn add_gfa_path<T: OptFields>(&mut self, path: &gfa::gfa::Path<usize, T>) {
        let path_id = self.create_path_handle(&path.path_name, false);
        for (name, orient) in path.iter() {
            self.append_step(&path_id, Handle::new(name as u64, orient));
        }
    }

    pub fn from_gfa<T: OptFields>(gfa: &GFA<usize, T>) -> HashGraph {
        let mut graph = Self::new();
        gfa.segments.iter().for_each(|s| graph.add_gfa_segment(s));
        gfa.links.iter().for_each(|l| graph.add_gfa_link(l));
        gfa.paths.iter().for_each(|p| graph.add_gfa_path(p));
        graph
    }
    ...
}
```
Vengono successivamente implementati:
- Alcuni traits definiti in ```handlegraph``` per la scansione di un grafo.
- Alcuni traits  definiti in ```mutablehandlegraph``` 
per permettere la manipolazione di un grafo, quindi permettendo l'aggiunta, la rimozione oppure 
la modifica degli elementi di un grafo o del grafo stesso.
- Alcuni traits definiti in ```pathgraph``` fondamentali per la corretta creazione di un grafo.