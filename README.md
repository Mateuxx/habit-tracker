# 🧠 Habit Tracker API – Rust + Axum

Uma API simples e escalável para gerenciamento de hábitos, desenvolvida com [Rust](https://www.rust-lang.org/) e o framework web [Axum](https://docs.rs/axum/latest/axum/). Esse projeto faz parte de um estudo prático de backend com foco em boas práticas, entendimento profundo e arquitetura limpa.

---

## 🚀 Funcionalidades

- ✅ Criar um novo hábito (`POST /habits`)
- ✅ Listar todos os hábitos (`GET /habits`)
- ✅ Atualizar um hábito existente (`PATCH /habits/:id`)
- ✅ Deletar um hábito (`DELETE /habits/:id`)

---

## 🧱 Tecnologias e Conceitos

- [x] Rust 2021
- [x] Axum (roteamento e handlers)
- [x] sqlx (queries assíncronas e seguras)
- [x] PostgreSQL
- [x] AppState com injeção de dependência
- [x] .env + dotenvy
- [x] UUIDs e timestamps com chrono

---

## 📦 Instalação e uso local

### 1. Clone o repositório

```bash
git clone https://github.com/Mateuxx/habit-tracker.git
cd habit-tracker
