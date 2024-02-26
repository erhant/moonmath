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
	@jupyter nbconvert --to markdown ./arithmetics/README.ipynb
	@jupyter nbconvert --to markdown ./algebra/README.ipynb
	@jupyter nbconvert --to markdown ./elliptic-curves/README.ipynb
	@jupyter nbconvert --to markdown ./statements/README.ipynb
