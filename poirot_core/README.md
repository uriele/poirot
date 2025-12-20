# Poirot

`poirot` is a Rust library and tool for managing academic resources as a typed knowledge graph backed by a Cozo database.
It provides structured modeling of authors, papers, venues, methods, keywords, and other research entities, along with durable storage, fast graph queries, and import/export utilities.

Poirot helps researchers organize literature, track conceptual links, and find methodologies across fields—turning your personal reading database into a searchable, cross-disciplinary graph.

#Features
## Graph-structured academic knowledge base

Model research entities as rich, interconnected nodes:

- Authors
- Research Groups & Institutions
- Papers, books, preprints, open source code
- Datasets, methodologies, algorithms
- Keywords & topics

All stored in a CozoDB graph/relational hybrid—enabling:

- Fast dependency-like traversals
- Semantic linking (e.g., method X used by paper Y)
- Versioning and incremental updates
- Flexible data modeling

## Bibliography Plug-in Architecture
`poirot` is designed to support modular import/export from:

- BibTeX
- Typst (.typ)
- Zotero exports
- CSL JSON
- Word / .docx embedded citations
- arXiv / CrossRef / Semantic Scholar APIs

## Crossfield Methodology Discovery
Because entities are stored in a graph rather than a flat biliography, you can explore
- numerical and computational methods used across disciplines
- citation neighborhoods
- cluster of related works
- authors with overlapping methodological footprint


# Roadmap

Core

- [ ] Stable schema for academic entities

- [ ] Unified entity resolver (deduplicate authors, venues, etc.)

- [ ] Search and query DSL

- [ ] Import/export modules

- [ ] GPUI client

Indexing & discovery

- [ ] Methodology-level tagging

- [ ] Topic clustering

- [ ] Graph traversals (“show me all papers using method X”)

Integration

- [ ] Typst plug-in

- [ ] Zotero connector

- [ ] arXiv/Semantic Scholar live search

- [ ] SerpAPI for Google Scholar

- [ ] WebOfScienceAPI