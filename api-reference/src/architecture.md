# Architecture

This page describes the internal architecture and build pipeline of Sapling.

## High-Level Overview

```mermaid
flowchart LR
    Input[Input Files] --> Parse[Parse Phase]
    Parse --> Discover[Discover Content]
    Discover --> Merge[Merge Phase]
    Merge --> Render[Render Phase]
    Render --> PostProcess[Post-Process]
    PostProcess --> Output[static/]
```

## Phase 1: Parse Templates

```mermaid
flowchart LR
    templates["templates/*.html"] --> PT1["Tera::new(glob)"]
    PT1 --> PT2[Compile templates]
    PT2 --> PT3[Validate syntax]
    PT3 --> Tera["Tera instance (cached)"]
    
    Tera -.->|"used in"| Render[Render Phase]
    Tera -.->|"used in"| ReverseIndex[Reverse Index Phase]
```

Templates are loaded once via glob pattern and compiled into a Tera instance. This instance is reused for all rendering operations.

## Phase 2: Discover Content

```mermaid
flowchart TB
    content["content/*.md"] --> WalkDir["WalkDir traversal"]
    WalkDir --> Spawn["tokio::spawn (per file)"]
    
    subgraph PerFile["Per-File Parallel Processing"]
        Spawn --> Read["fs::read_to_string"]
        Read --> Comrak["comrak::parse_document"]
        Comrak --> Extract["Extract frontmatter"]
        Extract --> YAML["serde_yaml::from_str"]
        Comrak --> HTML["comrak::format_html"]
        YAML --> FI["BuildForwardIndex"]
        HTML --> FI
        YAML --> RI["BuildReverseIndex"]
        HTML --> RI
    end
    
    FI --> Merge["MergeForwardIndex"]
    RI --> Merge
    
    Merge --> Discovered["Discovered { HashMap&lt;String, ContentDocument&gt; }"]
```

Each markdown file is parsed in parallel. Indexes are built during discovery.

### Per-File Flow Detail

```mermaid
flowchart LR
    MD["blog.md"] --> Parse["ParseMarkdown::parse"]
    Parse --> FM["frontmatter: YAML Value"]
    Parse --> Content["content: HTML string"]
    Parse --> Name["name: String"]
    
    FM --> Doc["ContentDocument"]
    Content --> Doc
    Name --> Doc
```

## Phase 3: Merge ForwardIndex

```mermaid
flowchart TB
    subgraph Before["Before Merge"]
        FI["ForwardIndex&lt;br/&gt;HashMap&lt;String, Vec&lt;Value&gt;&gt;"]
        CD1["ContentDocument A"]
        CD2["ContentDocument B"]
        CD3["ContentDocument C"]
    end
    
    FI -->|"clone"| CD1
    FI -->|"clone"| CD2
    FI -->|"clone"| CD3
    
    subgraph After["After Merge"]
        CD1M["ContentDocument A&lt;br/&gt;+ forwardindex"]
        CD2M["ContentDocument B&lt;br/&gt;+ forwardindex"]
        CD3M["ContentDocument C&lt;br/&gt;+ forwardindex"]
    end
```

**Bottleneck:** Each ContentDocument gets a full clone of the forwardindex. For N documents, this is O(n²) memory.

## Phase 4: Render HTML

```mermaid
flowchart LR
    Discovered["Discovered HashMap"] --> Spawn2["tokio::spawn (per doc)"]
    Tera["Tera instance"] --> Spawn2
    
    subgraph PerDoc["Per-Document Rendering"]
        Spawn2 --> Path["decide_static_serve_path"]
        Path --> Validate["validate_template_request"]
        Validate --> Context["Context::from_serialize"]
        Context --> Render["tera.render"]
        Render --> Write["fs::write"]
    end
    
    Write --> Output["static/{link}.html"]
```

Each document is rendered in parallel using the cached Tera instance.

## Phase 5: CSS Bundle

```mermaid
flowchart LR
    CSS["css/*.css"] --> Walk["WalkDir::new"]
    Walk --> Bundle["LightningCSS Bundler"]
    Bundle --> Imports["Resolve @import"]
    Imports --> Minify["Minify"]
    Minify --> Output["static/css/*.css"]
```

CSS files are bundled and minified using LightningCSS. This phase is sequential.

## Phase 6: Copy Assets

```mermaid
flowchart LR
    Assets["assets/*"] --> Walk2["WalkDir::new"]
    Walk2 --> Copy["fs::copy"]
    Copy --> Output2["static/assets/*"]
```

Direct file copy, no transformation.

## Phase 7: Reverse Index Render

```mermaid
flowchart TB
    RI["ReverseIndex"] --> Iter["Iterate keys"]
    
    subgraph PerKey["Per Key (e.g. 'tags')"]
        Iter --> IterVals["Iterate values (e.g. 'rust', 'go')"]
        IterVals --> Mkdir["mkdir static/tags/{value}/"]
        Mkdir --> BuildCtx["Build Context&lt;br/&gt;{ reverseindex, reverseindexon }"]
        BuildCtx --> Render2["tera.render"]
        Render2 --> Write2["write index.html"]
    end
    
    Tera2["Tera instance"] -.-> Render2
    Write2 --> Output3["static/tags/rust/index.html&lt;br/&gt;static/author/navin/index.html"]
```

Generates listing pages for each tag, author, or other reverse-indexed field.

## Phase 8: RSS Generation

```mermaid
flowchart LR
    Settings["settings.yaml"] --> Groups["rss_groups config"]
    Discovered2["Discovered HashMap"] --> Map["generate_rss_map"]
    Groups --> Map
    
    Map --> RenderRSS["render_rss"]
    RenderRSS --> Output4["static/rss/*.xml"]
```

## Pipeline Stages Summary

| Stage | Module | Parallel? | Description |
|-------|--------|-----------|-------------|
| Parse Templates | `ParseTemplate` | No | Load all templates via Tera glob, compile once |
| Discover Content | `LoadMemory` | Yes (per file) | Walk content/, parse MD in parallel, build indices |
| Merge ForwardIndex | `LoadMemory` | No | Clone forwardindex into every ContentDocument |
| Render HTML | `RenderMarkdown` | Yes (per doc) | Apply templates, write HTML files |
| Bundle CSS | `RenderMarkdown` | No | LightningCSS bundle + minify each CSS file |
| Copy Assets | `RenderMarkdown` | No | Direct file copy |
| Reverse Index | `ReverseIndex` | No | Generate tag/author listing pages |
| RSS | `rss` | No | Generate RSS XML feeds |

## Data Structures

### ContentDocument

```mermaid
classDiagram
    class ContentDocument {
        +frontmatter_raw: Option~String~
        +frontmatter: Option~Value~
        +content: Option~String~
        +name: Option~String~
        +forwardindex: Option~HashMap~
    }
```

### Index Structures

```mermaid
flowchart TB
    subgraph ForwardIndex["ForwardIndex"]
        FI_H["HashMap&lt;String, Vec&lt;Value&gt;&gt;"]
        FI_H --> FI_K1["'blog' → [post1_fm, post2_fm, ...]"]
        FI_H --> FI_K2["'news' → [news1_fm, ...]"]
    end
    
    subgraph ReverseIndex["ReverseIndex"]
        RI_H["HashMap&lt;String, HashMap&lt;String, Vec&lt;Value&gt;&gt;&gt;"]
        RI_H --> RI_O["'tags' → HashMap"]
        RI_H --> RI_O2["'author' → HashMap"]
        RI_O --> RI_V1["'rust' → [post1_fm, post3_fm]"]
        RI_O --> RI_V2["'go' → [post2_fm]"]
    end
```

### Core Types

| Type | Location | Description |
|------|----------|-------------|
| `ContentDocument` | `ParseMarkdown.rs` | Primary data carrier: frontmatter, content HTML, name |
| `Discovered` | `LoadMemory.rs` | Wrapper `HashMap<String, ContentDocument>` |
| `TemplatesMetaData` | `ParseTemplate.rs` | Thin wrapper around Tera instance |
| `ForwardIndex` | `LoadMemory.rs` | `HashMap<String, Vec<Value>>` - keyed by index name |
| `ReverseIndex` | `LoadMemory.rs` | Nested HashMap for tag/author pages |

## Known Bottlenecks

| Issue | Location | Impact | Fix |
|-------|----------|--------|-----|
| O(n²) Memory | `LoadMemory.rs:188-200` | Clones forwardindex into every document | Use `Arc<HashMap>` or Tera global context |
| Sync MD Parsing | `ParseMarkdown::parse` | Blocks tokio worker threads | Use `tokio::fs` or `rayon` |
| Parent Mutex | `LoadMemory.rs:72-79` | Serializes index updates | Remove or use proper RwLock strategy |
| Sequential CSS | `copy_css_files` | No parallelism | Parallelize bundling |