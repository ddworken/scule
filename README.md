# scule

```scule``` is a simple UTF-8 aware program to change the case of text. ```scule``` is UTF-8 aware and works correctly with UTF-8. ```scule``` is made to be used as part of a pipeline or as the start of a pipeline. When receiving input from stdin, case is a streaming program and will output text as it is received. 

# Usage

```
Usage: scule [options] [<file>]

Options:
    -h, --help  display this help and exit
    -u, --upper  convert to upper case 
    -l, --lower  convert to lower case 
    -v, --version  print the version
```

It can be used like: ```scule test.txt``` or ```echo test | scule```. 

If you do not specify ```--upper``` or ```--lower```, it will default to ```--lower```. 

# Installation

``` bash
git clone https://github.com/ddworken/scule.git
cd scule/
cargo build --release
mv target/release/scule /usr/bin/
```

# Why ```scule```?

The name scule was chosen because it is the suffix in majuscule and minuscule (meaning upper case and lower case respectively). 
