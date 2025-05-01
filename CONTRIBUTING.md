# Contributing to Downrust

Thank you for your interest in contributing to **Downrust** — a lightweight Markdown-to-HTML converter written in Rust! This project aims to support basic Markdown parsing and eventually provide a UI for rendering output in HTML.

Whether you're fixing bugs, adding features, improving documentation, or suggesting ideas — contributions of all kinds are welcome.

---

## 🚀 Getting Started

1. **Fork the repository** and clone it:
   ```bash
   git clone https://github.com/RKirlew/downrust.git
   cd downrust
   ```

2. **Set up your environment**  
   Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

3. **Build the project (debug mode):**
   ```bash
   cargo build
   ```

   This creates the binary at:
   ```
   ./target/debug/downrust
   ```

4. **Run the project:**
   ```bash
   cargo run
   ```

   Or run the compiled binary directly:
   ```bash
   ./target/debug/downrust
   ```

5. **Run tests (if any):**
   ```bash
   cargo test
   ```

---

## 📌 Todo List (Open Contributions Welcome)

Here are some areas where help is needed:

- [ ] Support for more headings (e.g. `###`)
- [ ] Parse unordered list items (`-`, `*`)
- [ ] Handle bold (`**text**` or `__text__`)
- [ ] Handle italics (`*text*` or `_text_`)
- [ ] Support reading input from `.md` files
- [ ] Build a simple HTML UI for previewing
- [ ] Refactor and clean up the file structure
- [ ] Parse inline and block code (e.g., ``code``, ` ``` `)
- [ ] Parse links (`[text](url)`)

Feel free to open issues or PRs for any of the above — or suggest your own improvements.

---

## 💡 How to Contribute

1. **Open an issue**  
   Suggest changes or improvements before starting major work.

2. **Create a new branch**  
   Use a descriptive name:
   ```bash
   git checkout -b feat/bold-support
   ```

3. **Commit your changes**
   Write clear and concise commit messages:
   ```bash
   git commit -m "Add bold text support"
   ```

4. **Push and create a PR**
   Push to your fork and open a pull request to `main`.

---

## 🧹 Code Style

- Follow Rust's best practices
- Keep code readable and well-commented
- Keep functions small and modular

---

## 📄 License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT).

---

## 🙌 Need Help?

If you get stuck, feel free to open a discussion or reach out via issues. We're happy to help.

---

Thanks again for helping improve Downrust!
