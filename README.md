# RsHtml Examples

This repository contains various examples demonstrating the usage of [RsHtml](https://github.com/rshtml/rshtml), a Rust compile-time, type-safe HTML template engine. The goal is to showcase RsHtml's features both as small instructional snippets and as fully working real-world applications.

---

## Directory Structure

- `snippets/` – Small, educational examples that produce direct string output.
- `webapp/` – Larger project that demonstrate RsHtml in fully working web application.

---

## Instructional Snippets (`snippets/`)

These examples focus on teaching the fundamental features of RsHtml. Each file demonstrates a specific functionality or usage pattern.

| File | Description |
|------|-------------|
| `simple-expression.rs` | Demonstrates how to use Rust variables inside templates (`@self.field`, `@expr()`). |
| `parantheses-expression.rs` | Shows how to use parentheses in expressions (`@(expr)`). |
| `if-else.rs` | Shows conditional statements (`@if`, `@else if`, `@else`). |
| `for.rs` | Demonstrates for loops (`@for`). |
| `while.rs` | Demonstrates while loops (`@while`). |
| `match.rs` | Demonstrates pattern matching using `@match`. |
| `include.rs` | Shows how to include other templates. |
| `component.rs` | Demonstrates how to use components. |
| `layout.rs` | Shows how to use layouts with `@extends`. |
| `functions.rs` | Demonstrates how to use built-in functions. |
| `escaping.rs` | Shows how to escape HTML. |
| `raw-block.rs` | Shows how to include raw HTML blocks with `@raw`. |
| `code-block.rs` | Shows how to use code blocks (`@{ ... }`). |
| `comment.rs` | Demonstrates how to use comments. |

**Run instructional examples:**

```bash
cargo run --example simple-expression
```

Replace `simple-expression` with the name of any other snippet file to run it.

---

## Real-World Applications (`webapp/`)

It demonstrates RsHtml in more complex, real-world scenarios. It is a mini, standalone application that demonstrates how RsHtml integrates into a web environment.

| Project | Description |
|---------|-------------|
| `web_app/` | A simple web project that demonstrates the use of RsHtml.

**Run real-world applications:**

```bash
cd webapp
cargo run
```

---

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/rshtml/examples.git
cd examples
```

2. Run any instructional snippet:

```bash
cd snippets
cargo run --example simple-expression.rs.html
```

3. Or run a full web application:

```bash
cd webapp
cargo run
```

---
