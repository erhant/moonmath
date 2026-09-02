.PHONY: pull
pull:
	docker pull --platform linux/amd64 sagemath/sagemath

.PHONY: sage
sage:
	docker run -v ${PWD}:/home/sage/moonmath:ro -it sagemath/sagemath

.PHONY: notebook
notebook:
	docker run -v ${PWD}:/home/sage/moonmath:ro -p8888:8888 sagemath/sagemath sage-jupyter

.PHONY: markdown
markdown:
	@if command -v jupyter >/dev/null 2>&1; then \
		jupyter nbconvert --to markdown ./arithmetics/README.ipynb; \
		jupyter nbconvert --to markdown ./algebra/README.ipynb; \
		jupyter nbconvert --to markdown ./elliptic-curves/README.ipynb; \
		jupyter nbconvert --to markdown ./statements/README.ipynb; \
	else \
		echo "jupyter not found, falling back to Docker..."; \
		docker run --rm -v ${PWD}:/home/sage/moonmath sagemath/sagemath \
			jupyter nbconvert --to markdown \
			moonmath/arithmetics/README.ipynb \
			moonmath/algebra/README.ipynb \
			moonmath/elliptic-curves/README.ipynb \
			moonmath/statements/README.ipynb; \
	fi

.PHONY: book
book:
	@cp ./introduction/README.md ./.book/introduction.md
	@cp ./software-used/README.md ./.book/software-used.md
	@cp ./arithmetics/README.md ./.book/arithmetics.md
	@cp ./algebra/README.md ./.book/algebra.md
	@cp ./elliptic-curves/README.md ./.book/elliptic-curves.md
	@cp ./statements/README.md ./.book/statements.md
	@cp ./circuit-compilers/README.md ./.book/circuit-compilers.md
	@cp ./zero-knowledge/README.md ./.book/zero-knowledge.md
	mdbook build ./.book -d ./book-build --open
