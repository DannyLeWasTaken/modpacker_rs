/// The graph for all the mods as well handles
use crate::mc_mod;
use std::rc::Rc;
use std::cell::RefCell;


struct Handle<T> {
    test: T,
}

struct Node {
    mc_mod: mc_mod::McMod,
    edges: RefCell<Vec<Edge>>,
}

/// Represents the relationship types between the tail to head
pub enum EdgeRelations {
    /// The tail depends on the head
    Requirement,

    /// The tail optionally needs the head
    Optional,

    /// The tail is a dependency of the head
    Included,

    /// The tail is optionally a dependency of the head
    OptionalIncluded,
}

/// Describes the connections between nodes
struct Edge {
    /// Head of the edge
    head: Handle<Rc<mc_mod::McMod>>,

    /// Relationship that tail has to edge
    relation: EdgeRelations,
}

/// A simple graph to hold all the mods used
struct Graph {
    nodes: Vec<Node>,
}