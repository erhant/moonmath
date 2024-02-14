.PHONY: pull
pull:
	docker pull --platform linux/amd64 sagemath/sagemath

.PHONY: sage
sage:
	docker run -v ${PWD}:/home/sage/moonmath:ro -it sagemath/sagemath

.PHONY: notebook
notebook:
	docker run -v ${PWD}:/home/sage/moonmath:ro -p8888:8888 sagemath/sagemath sage-jupyter
