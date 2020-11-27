import setuptools

with open("README.md") as f:
	long_description = f.read()

setuptools.setup(
	name="bree",
	version="1.0",
	author="Ender",
	description="Determines whether I am bree on a given date",
	long_description=long_description,
	long_description_content_type="text/markdown",
	url="https://github.com/Brecert/is_bree",
	license="MIT",
	packages=("bree",),
	classifiers=[
		"Programming Language :: Python :: 3",
		"License :: OSI Approved :: MIT License",
		"Operating System :: OS Independent",
	],
	entry_points={
		"console_scripts": ['is_bree=bree.is_bree:main'],
	},
	python_requires=">=3.2",
)