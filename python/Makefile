ifeq ($(OS),Windows_NT)
	PYTHON=python
	RMTREE=rd /q /s
else
	PYTHON=python3
	RMTREE=rm -r
endif

.DEFAULT_GOAL := dist/bree-1.0-py3-none-any.whl

dist/bree-1.0-py3-none-any.whl : bree/__init__.py bree/is_bree.py README.md
	$(PYTHON) setup.py bdist_wheel

.PHONY: clean
clean :
	$(RMTREE) build
	$(RMTREE) bree.egg-info