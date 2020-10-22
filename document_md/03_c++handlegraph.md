# [C++ HANDLEGRAPH](https://github.com/vgteam/libhandlegraph)

## File [handle.cpp](https://github.com/vgteam/libhandlegraph/blob/master/src/handle.cpp):
fornisce delle funzioni di utility, come ad esempio:
1. Metodi:
    ```cpp
    bool HandleGraph::has_edge(const handle_t& left, const handle_t& right) const {
        bool not_seen = true;
        follow_edges(left, false, [&](const handle_t& next) {
            not_seen = (next != right);
            return not_seen;
        });
        return !not_seen;
    }

    size_t HandleGraph::get_edge_count() const {
        size_t total = 0;
        for_each_edge([&](const edge_t& ignored) {
            total++;
        });
        return total;
    };
    ```
1. Operatori:
    ```cpp
    ...
    /// Define equality on path handles
    bool operator==(const path_handle_t& a, const path_handle_t& b) {
        return as_integer(a) == as_integer(b);
    }

    /// Define inequality on path handles
    bool operator!=(const path_handle_t& a, const path_handle_t& b) {
        return as_integer(a) != as_integer(b);
    }
    ...
    ```
3. Implementazione di default

## File [types.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/types.hpp):
Vengono definiti i ```types``` usati per gli handles e gli ```operatori``` con cui si andranno ad interfacciare.
1. Types:
    ```cpp
    ...
    /// Represents the internal id of a node traversal
    struct handle_t { char data[sizeof(nid_t)]; };
        
    /// Represents an edge in terms of its endpoints
    typedef std::pair<handle_t, handle_t> edge_t;
        
    /// Represents the internal id of a path entity
    struct path_handle_t { char data[sizeof(int64_t)]; }
    ...
    ```
2. Operatori:
    ```cpp
    /**
    * Define hashes for handles.
    */
    template<> struct hash<handlegraph::handle_t> {
    public:
        inline size_t operator()(const handlegraph::handle_t& handle) const {
            // TODO: We can't include util.cpp for as_integer because it includes us!
            // But we need anyone with access to the handle_t to be able to hash it.
            // So we just convert to integer manually.
            return std::hash<int64_t>()(reinterpret_cast<const uint64_t&>(handle));
        }
    };
    ```

## File [util.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/util.hpp):
Vengono specificati i metodi per le implementazioni di un ```handlegraph``` per "impacchettare" e "spacchettare" gli  ```handles```.
1. Handles:
    ```cpp
    ...
    /// View a handle as an integer
    inline uint64_t& as_integer(handle_t& handle) {
        return reinterpret_cast<uint64_t&>(handle);
    }

    /// View an integer as a handle
    inline handle_t& as_handle(uint64_t& value) {
        return reinterpret_cast<handle_t&>(value);
    }

    /// It's convenient to be able to sort handles
    inline bool operator<(const handle_t& a, const handle_t& b) {
    return (as_integer(a) < as_integer(b));
    }
    ...
    ```
    1. Metodo per "impacchettare" e "spacchettare" i dati degli ```Handles```:
        ```cpp
        struct number_bool_packing {
            ...
            /// Extract the packed bit
            inline static bool unpack_bit(const handle_t& handle) {
                return as_integer(handle) & 1;
            }
            
            /// Pack up an integer and a bit into a handle
            inline static handle_t pack(const uint64_t& number, const bool& bit) {
                // Make sure the number doesn't use all the bits
                assert(number < (0x1ULL << 63));
                
                return as_handle((number << 1) | (bit ? 1 : 0));
            }
            ...
        };
        ```
Gli stessi metodi ```inline``` vengono poi implementati anche per i ```Path Handles``` e ```Step Handles```, 
ovviamente con alcune differenze per poter adattarsi ai ```type``` differenti.

## File [iteratee.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/iteratee.hpp):
Definisce i metodi per implementare il pattern ```iteratee``` con la possibilita' di terminare in maniera precoce.\
\
L'idea alla base di un pattern ```iteratee``` e' quella di avere una funzione che prende un ```lambda iteratee``` e 
ne fa una chiamata a funzione, possibilmente in parallelo, con un insieme di elementi.\
L'```iteratee``` potra' poi ritornare ```void``` oppure un valore ```bool```.\
Se il valore di ritorno e' ```false``` l'iterazione viene fermata.
```cpp
/// This template has a static method that takes a callable on items and
/// returns a wrapped version that returns the calable's returned bool, or true
/// for void-returning callables.
template<typename Iteratee, typename Iterated,
    typename IterateeReturnType = decltype(std::declval<Iteratee>().operator()(std::declval<Iterated>()))>
struct BoolReturningWrapper {
    static inline std::function<bool(const Iterated&)> wrap(const Iteratee& iteratee);
};

/// This specialization handles wrapping void-returning callables.
template<typename Iteratee, typename Iterated>
struct BoolReturningWrapper<Iteratee, Iterated, void> {
    static inline std::function<bool(const Iterated&)> wrap(const Iteratee& iteratee) {
        return [&](const Iterated& item){
            iteratee(item);
            return true;
        };
    }
};

/// This specialization handles wrapping bool-returning callables.
template<typename Iteratee, typename Iterated>
struct BoolReturningWrapper<Iteratee, Iterated, bool> {
    static inline std::function<bool(const Iterated&)> wrap(const Iteratee& iteratee) {
        return [&](const Iterated& item){
            return iteratee(item);
        };
    }
};
```

## File [handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/handle_graph.hpp):
Viene definita l'interfaccia di un ```HandleGraph```.
1. Interfaccia che necessita di essere implementata:
    ```cpp
    ...
    /// Method to check if a node exists by ID
    virtual bool has_node(nid_t node_id) const = 0;

    /// Invert the orientation of a handle (potentially without getting its ID)
    virtual handle_t flip(const handle_t& handle) const = 0;

    /// Return the number of nodes in the graph
    virtual size_t get_node_count() const = 0;
    ...
    ```
2. Interfaccia di default che si appoggia ai ```backing virtual methods```:
    ```cpp
    /// Loop over all the handles to next/previous (right/left) nodes. Passes
    /// them to a callback. If called with a bool-returning invocable thing,
    /// can stop early when the function returns false. Returns true if we
    /// finished and false if we stopped early.
    template<typename Iteratee>
    bool follow_edges(const handle_t& handle, bool go_left, const Iteratee& iteratee) const;
    
    /// Loop over all the nodes in the graph in their local forward
    /// orientations, in their internal stored order. If called with a
    /// bool-returning invocable thing, can stop early when the function
    /// returns false. Returns true if we finished and false if we stopped
    /// early. Can be told to run in parallel, in which case stopping after a
    /// false return value is on a best-effort basis and iteration order is not
    /// defined.
    template<typename Iteratee>
    bool for_each_handle(const Iteratee& iteratee, bool parallel = false) const;
    ```
3. Interfaccia aggiuntiva con implementazione di default:
    ```cpp
    ...
    /// Returns true if there is an edge that allows traversal from the left
    /// handle to the right handle. By default O(n) in the number of edges
    /// on left, but can be overridden with more efficient implementations.
    virtual bool has_edge(const handle_t& left, const handle_t& right) const;

    /// Return the total number of edges in the graph. If not overridden,
    /// counts them all in linear time.
    virtual size_t get_edge_count() const;
    ...
    ```
4. Utility methods:
    ```cpp
    ...
    /// A pair of handles can be used as an edge. When so used, the handles have a
    /// canonical order and orientation.
    edge_t edge_handle(const handle_t& left, const handle_t& right) const;
    ...
    ```
5. ```Backing protected utility methods``` che necessitano di essere implementati:
    ```cpp
    /// Loop over all the handles to next/previous (right/left) nodes. Passes
    /// them to a callback which returns false to stop iterating and true to
    /// continue. Returns true if we finished and false if we stopped early.
    virtual bool follow_edges_impl(const handle_t& handle, bool go_left, const std::function<bool(const handle_t&)>& iteratee) const = 0;
    
    /// Loop over all the nodes in the graph in their local forward
    /// orientations, in their internal stored order. Stop if the iteratee
    /// returns false. Can be told to run in parallel, in which case stopping
    /// after a false return value is on a best-effort basis and iteration
    /// order is not defined. Returns true if we finished and false if we 
    /// stopped early.
    virtual bool for_each_handle_impl(const std::function<bool(const handle_t&)>& iteratee, bool parallel = false) const = 0;
    ```
6. Definizione interfacce per un ```HandleGraph```:
    ```cpp
    ...
    /**
    * Defines an interface providing a vectorization of the graph nodes and edges,
    * which can be co-inherited alongside HandleGraph.
    */
    class VectorizableHandleGraph : virtual public RankedHandleGraph {

    public:

        virtual ~VectorizableHandleGraph() = default;

        /// Return the start position of the node in a (possibly implict) sorted array
        /// constructed from the concatenation of the node sequences
        virtual size_t node_vector_offset(const nid_t& node_id) const = 0;

        /// Return the node overlapping the given offset in the implicit node vector
        virtual nid_t node_at_vector_offset(const size_t& offset) const = 0;

        /// Return a unique index among edges in the graph
        virtual size_t edge_index(const edge_t& edge) const = 0;
    };
    ```
7. Implementazione Templates:
    ```cpp
    template<typename Iteratee>
    bool HandleGraph::follow_edges(const handle_t& handle, bool go_left, const Iteratee& iteratee) const {
        return follow_edges_impl(handle, go_left, BoolReturningWrapper<Iteratee, handle_t>::wrap(iteratee));
    }
    ...
    ```
## File [path_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/path_handle_graph.hpp):
Definisce un'interfaccia per tutti quei grafi che includono dei ```Path (P)```.\
Il concetto alla base di questa interfaccia e' molto simile a quello alla base dell'interfaccia ```handle_graph.hpp```,
per cui vi sono:
1. Interfacce di default 
2. Interfacce che necessitano di essere implementate
3. Interfacce aggiuntive
4. Metodi di utility
5. Templates

## File [path_position_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/path_position_handle_graph.hpp):
Questa interfaccia fornisce un modo per gestire i ```Path HandleGraph``` con ```Path Position```.
1. Interfaccia che necessita di essere implementata:
    ```cpp
    /// Returns the length of a path measured in bases of sequence.
    virtual size_t get_path_length(const path_handle_t& path_handle) const = 0;
    
    /// Returns the position along the path of the beginning of this step measured in
    /// bases of sequence. In a circular path, positions start at the step returned by
    /// path_begin().
    virtual size_t get_position_of_step(const step_handle_t& step) const = 0;
    
    /// Returns the step at this position, measured in bases of sequence starting at
    /// the step returned by path_begin(). If the position is past the end of the
    /// path, returns path_end().
    virtual step_handle_t get_step_at_position(const path_handle_t& path,
                                               const size_t& position) const = 0;
    ```
2. Interfacce agggiuntive opzionali con implementazione di default:
    ```cpp
    /// Execute an iteratee on each step on a path, along with its orientation relative to
    /// the path (true if it is reverse the orientation of the handle on the path), and its
    /// position measured in bases of sequence along the path. Positions are always measured
    /// on the forward strand.
    ///
    /// Iteration will stop early if the iteratee returns false. This method returns false if
    /// iteration was stopped early, else true
    virtual bool for_each_step_position_on_handle(const handle_t& handle,
                                                  const std::function<bool(const step_handle_t&, const bool&, const size_t&)>& iteratee) const;
    ```

## File [serializable_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/serializable_handle_graph.hpp):
Definisce la base per l'interfaccia ```SerializableHandleGraph```.\
Viene definita un interfaccia per la ```serializzazione``` e ```deserializzazione``` di ```handlegraph```, 
che puo' venire ereditata insieme a ```HandleGraph```.
```cpp
...
 /// Write the contents of this graph to a named file. Makes sure to include
/// a leading magic number.
inline void serialize(const std::string& filename) const;

/// Sets the contents of this graph to the contents of a serialized graph from
/// a file. The serialized graph must be from the same implementation of the
/// HandleGraph interface as is calling deserialize(). Can only be called on an
/// empty graph.
inline void deserialize(const std::string& filename);
...
```

## File [mutable_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/mutable_handle_graph.hpp):
Definisce un'interfaccia per un ```MutableHandleGraph```, ovvero un ```handlegraph``` che supporta l'aggiunta
di nuovo materiale al suo grafo.
```cpp
...
/// Create a new node with the given id and sequence, then return the handle.
/// The sequence may not be empty.
/// The ID must be strictly greater than 0.
virtual handle_t create_handle(const std::string& sequence, const nid_t& id) = 0;

/// Create an edge connecting the given handles in the given order and orientations.
/// Ignores existing edges.
virtual void create_edge(const handle_t& left, const handle_t& right) = 0;
...
/// Adjust the representation of the graph in memory to improve performance.
/// Optionally, allow the node IDs to be reassigned to further improve
/// performance.
/// Note: Ideally, this method is called one time once there is expected to be
/// few graph modifications in the future.
virtual void optimize(bool allow_id_reassignment = true) = 0;
...
```

## File [deletable_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/deletable_handle_graph.hpp):
Definisce un'interfaccia per un ```DeletableHandleGraph```, ovvero un ```handlegraph``` che supporta la rimozione
di materiale dal suo grafo.
```cpp
/// Remove the node belonging to the given handle and all of its edges.
/// Either destroys any paths in which the node participates, or leaves a
/// "hidden", un-iterateable handle in the path to represent the sequence
/// of the removed node.
/// Invalidates the destroyed handle.
/// May be called during serial for_each_handle iteration **ONLY** on the node being iterated.
/// May **NOT** be called during parallel for_each_handle iteration.
/// May **NOT** be called on the node from which edges are being followed during follow_edges.
/// May **NOT** be called during iteration over paths, if it could destroy a path.
/// May **NOT** be called during iteration along a path, if it could destroy that path.
virtual void destroy_handle(const handle_t& handle) = 0;

/// Remove the edge connecting the given handles in the given order and orientations.
/// Ignores nonexistent edges.
/// Does not update any stored paths.
virtual void destroy_edge(const handle_t& left, const handle_t& right) = 0;

/// Convenient wrapper for destroy_edge.
inline void destroy_edge(const edge_t& edge) {
    destroy_edge(edge.first, edge.second);
}

/// Remove all nodes and edges. May also remove all paths, if applicable.
virtual void clear() = 0;
```

## File [mutable_path_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/mutable_path_handle_graph.hpp):
Questa interfaccia si puo' vedere come una crasi di ```MutableHandleGraph``` e ```DeletableHandleGraph``` 
sui ```Path```, dove un grafo supporta l'alterazione dei propri ```Path```.
```cpp
/**
* Destroy the given path. Invalidates handles to the path and its steps.
*/
virtual void destroy_path(const path_handle_t& path) = 0;

/**
* Create a path with the given name. The caller must ensure that no path
* with the given name exists already, or the behavior is undefined.
* Returns a handle to the created empty path. Handles to other paths must
* remain valid.
*/
virtual path_handle_t create_path_handle(const std::string& name,
                                            bool is_circular = false) = 0;
...                                            
```

## File [mutable_path_mutable_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/mutable_path_mutable_handle_graph.hpp):
Interfaccia per tutti quei grafi che supportano la modifica dei propri ```Path``` e l'aggiunta di 
elementi all'interno del grafo.

## File [mutable_path_deletable_handle_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/mutable_path_deletable_handle_graph.hpp):
Interfaccia per tutti quei grafi che supportano la modifica dei propri ```Path``` e la rimoziones di 
elementi all'interno del grafo.

## File [expanding_overlay_graph.hpp](https://github.com/vgteam/libhandlegraph/blob/master/src/include/handlegraph/expanding_overlay_graph.hpp):
Definisce un interfaccia per i grafi che si sovrappongono creando dei nodi duplicati.
```cpp
/**
 * This is the interface for a graph that represents a transformation of some underlying
 * HandleGraph where every node in the overlay corresponds to a node in the underlying
 * graph, but where more than one node in the overlay can map to the same underlying node.
 */
class ExpandingOverlayGraph : virtual public HandleGraph {

public:
    
    virtual ~ExpandingOverlayGraph() = default;
    
    /**
     * Returns the handle in the underlying graph that corresponds to a handle in the
     * overlay
     */
    virtual handle_t get_underlying_handle(const handle_t& handle) const = 0;
};
```