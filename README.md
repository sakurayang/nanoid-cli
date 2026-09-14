# nanoid-cli

[![Package version](https://img.shields.io/crates/v/nanoid.svg)](https://crates.io/crates/nanoid)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/sakurayang/nanoid-cli/blob/main/LICENSE)

> simple warper for [mrdimidium/nanoid](https://github.com/mrdimidium/nanoid).

## Download & Install

download at [release](https://github.com/sakurayang/nanoid-cli/release),
and put it on some where you like, then you can run it.

## Usage

> you can type `nanoid-cli --help` to get full doc

```bash
$ nanoid-cli
ujA69eUKXItjI8bxM6WZb

# customize size
$ nanoid-cli -s 10
BguSR9_FZM

# customize alphabet
$ nanoid-cli -a aSd
aaadSSaaaSSaSdaSddddS

# all upper alphabet
$ nanoid-cli --upper
F25B5AECDEA35B1F89EA0

# ...or lowwer. can't use with customize alphabet
$ nanoid-cli --lowwer
a9eda0efd07fe2e808f23

# and use seed
$ nanoid-cli --seed 12333
V82FlD85nfYK5YNNVdobm
```

## Other Programming Languages

Reference implementation on [JavaScript](https://github.com/ai/nanoid).

Nano ID was ported to many languages. You can use these ports to have
the same ID generator on the client and server side.

- [C](https://github.com/lukateras/nanoid.h)
- [C#](https://github.com/codeyu/nanoid-net)
- [C++](https://github.com/mcmikecreations/nanoid_cpp)
- [Clojure and ClojureScript](https://github.com/zelark/nano-id)
- [ColdFusion/CFML](https://github.com/JamoCA/cfml-nanoid)
- [Crystal](https://github.com/mamantoha/nanoid.cr)
- [Dart & Flutter](https://github.com/pd4d10/nanoid-dart)
- [Elixir](https://github.com/railsmechanic/nanoid)
- [Gleam](https://github.com/0xca551e/glanoid)
- [Go](https://github.com/matoous/go-nanoid)
- [Haskell](https://github.com/MichelBoucey/NanoID)
- [Haxe](https://github.com/flashultra/uuid)
- [Janet](https://sr.ht/~statianzo/janet-nanoid/)
- [Java](https://github.com/wosherco/jnanoid-enhanced)
- [Kotlin](https://github.com/viascom/nanoid-kotlin)
- [MySQL/MariaDB](https://github.com/viascom/nanoid-mysql-mariadb)
- [Nim](https://github.com/icyphox/nanoid.nim)
- [OCaml](https://github.com/routineco/ocaml-nanoid)
- [Perl](https://github.com/tkzwtks/Nanoid-perl)
- [PHP](https://github.com/hidehalo/nanoid-php)
- Python [native](https://github.com/puyuan/py-nanoid) implementation
  with [dictionaries](https://pypi.org/project/nanoid-dictionary)
  and [fast](https://github.com/oliverlambson/fastnanoid) implementation (written in Rust)
- Postgres [Extension](https://github.com/spa5k/uids-postgres)
  and [Native Function](https://github.com/viascom/nanoid-postgres)
- [R](https://github.com/hrbrmstr/nanoid) (with dictionaries)
- [Ruby](https://github.com/radeno/nanoid.rb)
- [Rust](https://github.com/nikolay-govorov/nanoid)
- [Swift](https://github.com/ShivaHuang/swift-nanoid)
- [Unison](https://share.unison-lang.org/latest/namespaces/hojberg/nanoid)
- [V](https://github.com/invipal/nanoid)
- [Zig](https://github.com/SasLuca/zig-nanoid)
