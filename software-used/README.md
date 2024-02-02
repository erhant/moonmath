# Chapter 2: Software Used in this Book

You need the following software for this book:

- [SageMath](https://www.sagemath.org/) for almost all chapters.
- [Circom](https://docs.circom.io/) for the last part.

## SageMath

I've used the [Conda installation](https://doc.sagemath.org/html/en/installation/conda.html) method for this one, which results in an environment that can be activated as follows:

```sh
conda activate sage
sage # launch sage
```

Within the Sage CLI, you can run files via: `load("./path-to-file.sage")`; or, within a Jupyter notebook, you can select the Sage kernel.

Alternatively, you may use the [SageMath Docker image](https://hub.docker.com/r/sagemath/sagemath) instead of installing it on your machine! To begin, pull the image:

```sh
docker pull --platform linux/amd64 sagemath/sagemath
```

At the root of this project, you can use the Sage CLI with all the folders mounted with the following command:

```sh
docker run -v $PWD:/home/sage/moonmath:ro -it sagemath/sagemath
```

You can even open a notebook using the Docker image:

```sh
docker run -v $PWD:/home/sage/moonmath:ro -p8888:8888 sagemath/sagemath sage-jupyter
```

## Circom

To install Circom, you need to have Rust installed first:

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then, you can install Circom by cloning its repository and then building its binary file.

```sh
git clone https://github.com/iden3/circom.git
cd circom
cargo build --release
cargo install --path circom
```
