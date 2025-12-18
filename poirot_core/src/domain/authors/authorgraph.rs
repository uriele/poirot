use super::Author;
use petgraph::prelude::UnGraphMap;
use std::collections::{BTreeSet, HashMap};
use uuid::Uuid;
pub type AuthorId = Uuid;
#[derive(Debug, Clone, Default)]
pub struct AuthorGraph {
    root: AuthorId,
    authors: HashMap<AuthorId, Author>,
    ordered: BTreeSet<AuthorId>,
    graph: UnGraphMap<AuthorId, u32>,
}

// Helper function to get AuthorId from Author
pub fn author_id(author: &Author) -> AuthorId {
    author.id.clone()
}

impl AuthorGraph {
    pub fn new(root: Author) -> Self {
        let root_id = author_id(&root);
        let mut authors = HashMap::new();
        authors.insert(root_id, root);

        let mut ordered = BTreeSet::new();
        ordered.insert(root_id.clone());

        let mut graph = UnGraphMap::new();
        graph.add_node(root_id.clone());

        Self {
            root: root_id,
            authors,
            ordered,
            graph,
        }
    }

    pub fn root_id(&self) -> &AuthorId {
        &self.root
    }

    pub fn graph(&self) -> &UnGraphMap<AuthorId, u32> {
        &self.graph
    }

    pub fn author_ids_stable(&self) -> impl Iterator<Item = &AuthorId> {
        self.ordered.iter()
    }

    pub fn upsert_author(&mut self, author: Author) -> AuthorId {
        let id = author_id(&author);
        //TODO: finish
        self.authors.entry(id.clone()).or_insert(author);
        id
    }
}

// TODO: Given a query about 1 author, I want to create queries about the coauthors such
// that
// 1. search_query=au:"Author A"
// 2. \forall   coauthor: create edges with weight equal to paper in common
// 3. save all coauthors and initialize their edges with weight equal to the paper in common.
// 4. Search  (au:"AuthorB")* !&& (au:"Author A") to get papers only by AuthorB not with AuthorA
// 5. Update nodes weights of coauthors
// 6. Extract new coauthorship
// 7. Go back to 3
// 8. Repeat for N levels of coauthorship
// 9. Generate final author graph
