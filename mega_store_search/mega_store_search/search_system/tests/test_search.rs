use hashbrown::HashMap;

#[test]
fn test_busca_produto_existente() {
    let mut catalogo = HashMap::new();
    catalogo.insert("Notebook Dell", "Eletrônicos");

    assert_eq!(catalogo.get("Notebook Dell"), Some(&"Eletrônicos"));
}
