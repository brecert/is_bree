# bree

*Am I bree today? Find out, with this handy package!*

Python implementation of the famous `is_bree` algorithm.

## Building & Installation

```bash
make # to build
pip install dist/bree-1.0-py3-none-any.whl # to install
```

## Usage

```
> is_bree
Usage: is_bree <yyyy> <mm> <dd>
> is_bree 2020 12 11
is_bree(datetime.date(2020, 12, 11)) = True
```

## Tests

```
python -m unittest -v
```



## License

[MIT License](https://choosealicense.com/licenses/mit/)