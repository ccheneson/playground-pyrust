Playing around with Rust and Python. 

This is also to compare speeds

# Set up

Go to the project folder.

From the [Py03 page](https://github.com/PyO3/pyo3)

```
$ python -m venv .env
$ source .env/bin/activate
$ pip install maturin
```

From the [Maturin page](https://github.com/PyO3/maturin)
> Build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages.
> 
> There are three main commands:

>   * `maturin publish` builds the crate into python packages and publishes them to pypi.
>   * `maturin build` builds the wheels and stores them in a folder (target/wheels by default), but doesn't upload them. It's possible to upload those with twine.
>   * `maturin develop` builds the crate and installs it as a python module directly in the current virtualenv. Note that while maturin develop is faster, it doesn't support all the feature that running pip install after maturin build supports.



Now build and execute the module:

```
$ maturin develop
$ python
>>> import rustils
>>> rustils.multi_and_sum(range(10000), range(10000))
2499500025000000
>>> 
```


# Tests

In order to run the test, you need additional configuration to work around an issue - See more at [here](https://pyo3.rs/v0.7.0-alpha.1/advanced.html#testing)


To run the test,
```
cargo test --no-default-features
```

# Test files

You can generate an executable that calls our `multi_and_sum` function

```
cargo build --release --no-default-features
```

This will result in an executable called `prog01-rust`

At the root of the project you have 
* `prog01-py.py` : A simple python script that **does exactly** what our rust function does
* `prog01-rs.py` : A simple python script that **calls** our rust function
* `prog01-np.py` : A simple python script that **does exactly** what our rust function does with the help of the `numpy` lib.
* `prog01-rust`  : A rust main function that **calls** our rust function `multi_and_sum` 

```
$ time ./prog01-py.py 
2499500025000000

real	0m6,732s
user	0m6,708s
sys	0m0,016s
```

```
$ time ./prog01-rs.py 
2499500025000000

real	0m2,546s
user	0m2,533s
sys	0m0,012s
```

```
$ time ./prog01-np.py 
2499500025000000

real	0m0,350s
user	0m0,226s
sys	0m0,121s
```

```
$ time ./target/release/prog01-rust 
2499500025000000

real	0m0,066s
user	0m0,061s
sys	0m0,004s
```

# Note

The above commands must be run where you created the python environment.

If you want to use the module outside of this env, copy the file at `target/release/librustils.so` to `the/path/here/rustils.so` where `./prog01-rs.py` (python script that uses this module) is in `the/path/here`

See https://pyo3.rs/v0.15.0/building_and_distribution.html#manual-builds
> on Linux, rename `libyour_module.so` to `your_module.so`.


# Reference:
1) https://github.com/PyO3/pyo3
2) https://pyo3.rs/v0.7.0-alpha.1/advanced.html#testing
