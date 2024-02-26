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
3. [**Arithmetics**](./arithmetics/README.ipynb)
4. [**Algebra**](./algebra/README.ipynb)
5. [**Elliptic Curves**](./elliptic-curves/README.ipynb)
6. [**Statements**](./statements/README.ipynb)
7. [**Circuit Compilers**](./circuit-compilers/)
8. [**Zero-Knowledge Protocols**](./zero-knowledge/)

<h3 align="center">
    Usage
</h3>

See [Chapter 2](./software-used/) for the required software. If you don't have Sage installed, you can use [Docker](https://www.docker.com/) mounted over the repository. We have scripts for that:

```sh
make pull     # pull the SageMath image
make sage     # open Sage CLI
make notebook # open Jupyter Notebook with Sage
```

We also have a script that converts all the notebooks into Markdown format so that it is easier to read on the web:

```sh
make markdown
```
