# mzio_pyo3

This is a Python wrapper for the mzio library. It is written in Rust and uses the PyO3 library to interface with Python.

## Installation

To install the package, run the following command:

```bash
git clone https://github.com/oolonek/mzio_pyo3.git
cd mzio_pyo3
```

Install [maturin](https://www.maturin.rs/)

```bash
pip install maturin
```

## Build the package:

```bash
maturin develop --release
```

## Usage

```python
python tests/io.py
```


