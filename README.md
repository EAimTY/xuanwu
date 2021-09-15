# Xuanwu

Xuanwu is a simple CLI front end of [pangu](https://github.com/airt/pangu-rs) with no dependency, which can automatically insert whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).

Xuanwu (玄武) is a powerful deity in Chinese religion, one of the higher-ranking deities in Taoism. In Chinese mythology, Xuanwu is the son of Pangu (盘古; 盤古), who separated heaven and earth and became geographic features such as mountains and rivers.

## Usage

    Usage: xuanwu [options] [text]

    Options:
        -f FILENAME         read from file
        -h, --help          print this help menu

### Input from argument

```console
$ xuanwu "當你凝視著bug，bug也凝視著你"
當你凝視著 bug，bug 也凝視著你
```

### Input from file

```console
$ cat /PATH/TO/FILE
當你凝視著bug，bug也凝視著你

$ xuanwu -f /PATH/TO/FILE
當你凝視著 bug，bug 也凝視著你
```

### Output to file

```console
$ xuanwu "當你凝視著bug，bug也凝視著你" > /PATH/TO/FILE

$ cat /PATH/TO/FILE
當你凝視著 bug，bug 也凝視著你
```

## Installation

### Compile

```bash
$ git clone https://github.com/EAimTY/xuanwu && cd xuanwu
$ cargo build --release
```

You can find the compiled file at `target/release/xuanwu`

## License

GNU General Public License v3.0