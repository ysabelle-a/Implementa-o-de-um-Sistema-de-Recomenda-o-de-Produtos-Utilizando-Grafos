# Implementa-o-de-um-Sistema-de-Recomenda-o-de-Produtos-Utilizando-Grafos
Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

Descrição do Projeto
  Este projeto foi desenvolvido para o e-commerce da empresa MegaStore. Foi criado uma aplicação em Rust com o objetivo de realizar buscas e recomendações de produtos. O sistema criado oferece uma solução para o problema de lentidão e buscas imprecisas do sistema atual da MegaStore.
  Este novo sistema atende ao requisito de buscas de forma eficiente e escalável, sendo capaz de lidar com o grande volume de dados. Proporcionando uma excelente experiência ao cliente.

Tecnologias Utilizadas
  Linguagem: Rust

 Crates utilizadas:
  hashbrown
  regex
  serde e serde_json
  Estruturas de dados: HashMap, HashSet, BTreeMap, grafos de adjacência.


Como Executar o Sistema de Busca
 
  Compilar o projeto:
  cargo build --release

  Verificar a compilação:
  cargo check

  Executar:
  cargo run


Executar testes
  cargo test
  cargo test -- --nocapture
  cargo test test_tokenize_basic
  cargo test --release

Exemplos de Uso
  cargo run

Arquitetura do Sistema
  Product - Estrutura base com id, nome, marca, categoria e descrição.
  HashIndex - busca por palavras-chave usando HashMap.
  NameBTree - Busca por prefixo usando BTreeMap. 
  RecGraph - Sistema de recomendações baseado em grafo, usa HashMap.
  Catalog - Gerencia produtos, IDs.
  Add Product: Catalog, indexa em HashIndex, NameBTree
  Busca Tokens: Tokenização, HashIndex (AND), Products Map
  Busca Prefixo: NameBTree


Algoritmos e Estruturas de Dados Utilizados
  Tabela Hash (HashMap)
  Conjunto (HashSet)
  Árvore B (BTreeMap)
  Grafo de Recomendações (RecGraph)

Considerações sobre Desempenho e Escalabilidade

  O sistema permite utilizar catálogos contendo diversos produtos. As buscas foram otimizadas usando HashIndex, que consegue manter as respostas rápidas mesmo quando aumentar a quantidade de produtos. O NameBTree permite que as sugestões apareçam enquanto o usuário digita. Os testes demonstraram que a busca por tokens e prefixos ocorre em milissegundos, garantindo uma experiência eficiente para o usuário.

Licença

  Este projeto está licenciado sob a MIT License — consulte o arquivo LICENSE para mais detalhes.