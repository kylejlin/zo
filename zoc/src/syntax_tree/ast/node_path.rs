#[derive(Debug, Clone, Copy)]
pub enum NodePath<'a> {
    Nil,
    Snoc(&'a NodePath<'a>, NodeEdge),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeEdge(pub usize);

pub const IND_INDEX_TYPES: NodeEdge = NodeEdge(0);
pub const IND_VCON_DEFS: NodeEdge = NodeEdge(1);

pub const VCON_DEF_PARAM_TYPES: NodeEdge = NodeEdge(0);
pub const VCON_DEF_INDEX_ARGS: NodeEdge = NodeEdge(1);

pub const VCON_IND: NodeEdge = NodeEdge(0);

pub const MATCH_MATCHEE: NodeEdge = NodeEdge(0);
pub const MATCH_RETURN_TYPE: NodeEdge = NodeEdge(1);
pub const MATCH_CASES: NodeEdge = NodeEdge(2);

pub const MATCH_CASE_RETURN_VAL: NodeEdge = NodeEdge(0);

pub const FUN_PARAM_TYPES: NodeEdge = NodeEdge(0);
pub const FUN_RETURN_TYPE: NodeEdge = NodeEdge(1);
pub const FUN_RETURN_VAL: NodeEdge = NodeEdge(2);

pub const APP_CALLEE: NodeEdge = NodeEdge(0);
pub const APP_ARGS: NodeEdge = NodeEdge(1);

pub const FOR_PARAM_TYPES: NodeEdge = NodeEdge(0);
pub const FOR_RETURN_TYPE: NodeEdge = NodeEdge(1);

impl NodePath<'_> {
    pub fn singleton(edge: NodeEdge) -> NodePath<'static> {
        NodePath::Snoc(&NodePath::Nil, edge)
    }

    pub fn len(&self) -> usize {
        match self {
            NodePath::Nil => 0,
            NodePath::Snoc(rdc, _) => 1 + rdc.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_vec(&self) -> Vec<NodeEdge> {
        match self {
            NodePath::Nil => vec![],
            NodePath::Snoc(rdc, rac) => {
                let mut vec = rdc.to_vec();
                vec.push(*rac);
                vec
            }
        }
    }
}
