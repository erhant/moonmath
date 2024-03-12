<p align="center">
  <h1 align="center">
    🌑 Moon Math 🌕
  </h1>
  <p align="center">
    <i>Exercises & codes from the "<a href="https://leastauthority.com/community-matters/moonmath-manual/">MoonMath Manual to zkSNARKs</a>" by Least Authority.</i>
  </p>
</p>

<h3 align="center">
    Chapters
</h3>

1. [**Introduction**](./introduction/)
2. [**Software Used in This Book**](./software-used/)
3. [**Arithmetics**](./arithmetics/)
4. [**Algebra**](./algebra/)
5. [**Elliptic Curves**](./elliptic-curves/)
6. [**Statements**](./statements/)
7. [**Circuit Compilers**](./circuit-compilers/)
8. [**Zero-Knowledge Protocols**](./zero-knowledge/)

<h3 align="center">
    Usage
</h3>

See [Software Used](./software-used/) for the required software. If you don't have [Sage](https://www.sagemath.org/) installed, you can use [Docker](https://www.docker.com/) mounted over the repository. We have scripts for that:

```sh
make pull     # pull the SageMath image
make sage     # open Sage CLI
make notebook # open Jupyter Notebook with Sage
```

We also have a script that converts all the notebooks into Markdown format via `jupyter nbconvert` so that they are easier to read on the web from the READMEs alone:

```sh
make markdown
```

<h3 align="center">
    Book
</h3>

A [mdBook](https://github.com/rust-lang/mdBook) build can be created & opened with:

```sh
make book
```

The README file are copied under the [`.book`](./.book/) directory and a build is created from them. Note that it is not hot-reloaded (i.e. `mdbook serve`). You will also need [KaTeX](https://github.com/lzanini/mdbook-katex) and [Mermaid](https://github.com/badboy/mdbook-mermaid) plugins.

<h3 align="center">
    Contributions
</h3>

Please feel free to open an issue or create a pull-request if something is not clear, could have been better, or is missing references. For the chapters with notebooks, please write the changes in the notebook and then generate the README files with `make markdown`.
