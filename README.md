# rimcache 🔄📦

**rimcache** é um projeto Rust criado para melhorar a performance do carregamento do jogo RimWorld, especialmente ao utilizar muitos mods com o gerenciador **RimPy**.

O objetivo principal é escanear os arquivos XML dos mods, gerar um cache otimizado e organizar os dados para que o jogo carregue mais rápido.

---

## 📌 Objetivo

- Ler e indexar arquivos XML presentes na pasta de mods
- Gerar um cache para facilitar o parsing futuro (binário ou JSON)
- Organizar estrutura de arquivos para facilitar diagnósticos e otimizações

---

## 🎯 Tecnologias

- Linguagem: **Rust**
- CLI: [`clap`](https://docs.rs/clap)
- Parsing XML: [`quick-xml`](https://docs.rs/quick-xml)
- Serialização de dados: [`serde`](https://serde.rs/)
- Progresso visual: [`indicatif`](https://docs.rs/indicatif)

---

## 🧠 Por que Rust?

Este projeto faz parte do meu laboratório pessoal de aprendizado com Rust.  
Quero entender profundamente conceitos como:

- Ownership e Borrow Checker
- `Result`, `Option` e tratamento de erros
- Modularização e boas práticas idiomáticas
- Concorrência segura com `rayon` (futuramente)

---

## 🚀 Como rodar

Clone este repositório e execute:

```bash
cargo run
