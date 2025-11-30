pub const SCHEMA: &str = r#"
:create entity {
    
    id: String,
    =>
    kind: String,          
    title: String,        
    autors: String,   
    uri: String?,          
    year: Int?,           
    props: Json?      
}

:create edge {
    src: String,
    dst: String,
    
    kind: String,
    =>
    props: Json?      
}

:create tag {
    name: String,
}


:create entity_tag {
    entity_id: String,
    tag_name: String, 
    =>
}

:create entity_vec {
    entity_id: String,
    =>            
    embedding: <F32; 768>          
}
"#;



pub const HNSW_INDEX: &str = r#"
::hnsw create entity_vec:entity_vec_hnsw {
    dim: 768,
    m: 32,
    dtype: F32,
    fields: [embedding],
    distance: L2,
    ef_construction: 20,
    filter: true,                  
    extend_candidates: false,
    keep_pruned_connections: false,
}
"#;