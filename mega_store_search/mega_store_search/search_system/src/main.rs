use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::time::{Duration, Instant};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct Product {
    id: usize,
    name: String,
    brand: String,
    category: String,
    description: Option<String>,
}

struct HashIndex {
    index: HashMap<String, HashSet<usize>>,
}

impl HashIndex {
    fn new() -> Self {
        Self { index: HashMap::new() }
    }

    fn index_product(&mut self, p: &Product) {
        let mut tokens = tokenize(&p.name);
        tokens.extend(tokenize(&p.brand));
        tokens.extend(tokenize(&p.category));

        for t in tokens {
            self.index.entry(t).or_default().insert(p.id);
        }
    }

    fn search_tokens_and(&self, tokens: &[String]) -> Vec<usize> {
        if tokens.is_empty() {
            return Vec::new();
        }

        let mut sets: Vec<&HashSet<usize>> = tokens.iter()
            .filter_map(|t| self.index.get(t))
            .collect();

        if sets.is_empty() {
            return Vec::new();
        }

        let mut result = sets[0].clone().clone();
        for s in sets.iter().skip(1) {
            result = result.intersection(s).cloned().collect();
            if result.is_empty() {
                break;
            }
        }
        result.into_iter().collect()
    }
}

struct RecGraph {
    adj: HashMap<usize, HashSet<usize>>,
}

impl RecGraph {
    fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        if a == b { return; }
        self.adj.entry(a).or_default().insert(b);
        self.adj.entry(b).or_default().insert(a);
    }

    fn recommend(&self, product_id: usize, limit: usize) -> Vec<usize> {
        let neighbors = self.adj.get(&product_id)
            .cloned()
            .unwrap_or_default();

        let mut scored: Vec<(usize, usize)> = neighbors.iter()
            .map(|&nid| {
                let degree = self.adj.get(&nid).map(|s| s.len()).unwrap_or(0);
                (nid, degree)
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(id, _)| id).take(limit).collect()
    }
}

struct NameBTree {
    tree: BTreeMap<String, Vec<usize>>,
}

impl NameBTree {
    fn new() -> Self {
        Self { tree: BTreeMap::new() }
    }

    fn insert(&mut self, name: &str, id: usize) {
        self.tree.entry(name.to_lowercase()).or_default().push(id);
    }

    fn search_prefix(&self, prefix: &str, limit: usize) -> Vec<usize> {
        let prefix = prefix.to_lowercase();
        let mut out = Vec::new();

        for (k, v) in self.tree.range(prefix.clone()..) {
            if !k.starts_with(&prefix) {
                break;
            }
            for id in v {
                out.push(*id);
                if out.len() >= limit {
                    return out;
                }
            }
        }
        out
    }
}

struct Catalog {
    products: HashMap<usize, Product>,
    next_id: usize,
    hash_index: HashIndex,
    rec_graph: RecGraph,
    name_tree: NameBTree,
}

impl Catalog {
    fn new() -> Self {
        Self {
            products: HashMap::new(),
            next_id: 1,
            hash_index: HashIndex::new(),
            rec_graph: RecGraph::new(),
            name_tree: NameBTree::new(),
        }
    }

    fn add_product(&mut self, mut p: Product) {
        p.id = self.next_id;
        self.next_id += 1;

        self.hash_index.index_product(&p);
        self.name_tree.insert(&p.name, p.id);
        self.products.insert(p.id, p);
    }

    fn add_recommendation_edge(&mut self, a: usize, b: usize) {
        self.rec_graph.add_edge(a, b);
    }

    fn search_exact_name(&self, name: &str) -> Vec<&Product> {
        let key = name.to_lowercase();
        self.products.values()
            .filter(|p| p.name.to_lowercase() == key)
            .collect()
    }

    fn search_tokens(&self, query: &str) -> Vec<&Product> {
        let tokens = tokenize(query);
        let ids = self.hash_index.search_tokens_and(&tokens);
        ids.iter()
            .filter_map(|id| self.products.get(id))
            .collect()
    }

    fn search_prefix_ordered(&self, prefix: &str, limit: usize) -> Vec<&Product> {
        let ids = self.name_tree.search_prefix(prefix, limit);
        ids.iter()
            .filter_map(|id| self.products.get(id))
            .collect()
    }

    fn recommend_for(&self, product_id: usize, limit: usize) -> Vec<&Product> {
        let rec_ids = self.rec_graph.recommend(product_id, limit);
        rec_ids.iter()
            .filter_map(|id| self.products.get(id))
            .collect()
    }
}

fn tokenize(s: &str) -> Vec<String> {
    let re = Regex::new(r"[^\w]+").unwrap();
    re.split(&s.to_lowercase())
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect()
}

fn time_it<F, R>(label: &str, f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let res = f();
    let dur = start.elapsed();
    println!("[METRIC] {}: {:?} ms", label, dur.as_millis());
    (res, dur)
}

fn main() {
    println!("=== MegaStore Search System (Versão Otimizada) ===");

    let mut catalog = Catalog::new();

    let sample = vec![
        Product { id: 0, name: "Notebook Dell Inspiron 15".into(), brand: "Dell".into(), category: "Eletrônicos".into(), description: Some("Intel i5, 8GB RAM".into()) },
        Product { id: 0, name: "Notebook Dell XPS 13".into(), brand: "Dell".into(), category: "Eletrônicos".into(), description: Some("Performance e portabilidade".into()) },
        Product { id: 0, name: "Camiseta Polo Masculina".into(), brand: "MarcaX".into(), category: "Vestuário".into(), description: Some("Algodão Pima".into()) },
        Product { id: 0, name: "Ração Golden Adulto".into(), brand: "Golden".into(), category: "Pet Shop".into(), description: Some("Proteína e vitaminas".into()) },
        Product { id: 0, name: "Capa para Notebook 15".into(), brand: "AcessoriosPro".into(), category: "Acessórios".into(), description: Some("Resistente à água".into()) },
    ];

    time_it("Indexing sample catalog", || {
        for p in sample {
            catalog.add_product(p);
        }
    });

    catalog.add_recommendation_edge(1, 2);
    catalog.add_recommendation_edge(1, 5);
    catalog.add_recommendation_edge(4, 3);

    let (res, _) = time_it("Search token 'dell'", || catalog.search_tokens("dell"));
    println!("-> Results for 'dell':");
    for p in res {
        println!("   {} | {} | {}", p.id, p.name, p.category);
    }

    let (res2, _) = time_it("Prefix search 'notebook'", || catalog.search_prefix_ordered("notebook", 10));
    println!("-> Results for prefix 'notebook':");
    for p in res2 {
        println!("   {} | {}", p.id, p.name);
    }

    let (recs, _) = time_it("Recommend for product id 1", || catalog.recommend_for(1, 5));
    println!("-> Recommendations for product id 1:");
    for p in recs {
        println!("   {} | {}", p.id, p.name);
    }

    let (res3, _) = time_it("Search tokens 'dell 15'", || catalog.search_tokens("dell 15"));
    println!("-> Results for 'dell 15':");
    for p in res3 {
        println!("   {} | {}", p.id, p.name);
    }

    println!("=== Demo finished ===");
}
