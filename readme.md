# Structured Editing of Stream Data

This rust package is an experimental library that combines a parser and a (text)
editor buffer. While the primary use is a text editor, the library is supposed
to be flexible enough to operate on any kind of symbol.

## Requirements

The library shall fulfil the following tasks:

* [X] Provide the basic operations of a text editor buffer.
* [X] Parse the input in parallel to editing, based on an arbitrary, user provided grammar.
* Parsing shall be able to happen
  * [X] either synchronously on every edit operation or
  * [ ] asynchronously in a separate thread.
  * [ ] Configuring this at compile time is sufficient.
* [X] Provide access to the parse tree of the current input.
* Support for Marpa's *ruby slippers* parsing where the parser fills in missing data.
* Support for
  * [X] rendering and
  * [ ] editing the stream based on the grammar

The following features (mostly based on Marps's features) are not in the scope:

* Support for multiple, parallel token interpretations.
* Leo's extensions for right recursive grammars.
* Aycock, Horspool's extensions for better performance.

While Marpa provides a much better parser, it does not provide the
synchronisation between parser and editor buffer. It might be possible to add
that on top of Marpa and will be considered as a future direction once the
basic proof-of-concept has been implemented.

* [X] A small editor (text mode only) will be added to demonstrate these features.

## How to build this library

Please understand this software is in a very early stage. Many features are
simply not developed. The architecture and the API of all modules are subject
to (sometimes drastic) changes from version to version.

You are welcome to try it out. This section will give you an overview on how to
download and build it.

See the next section for the features currently being implemented.

## Prerequisites

* Linux (tested on an 64 bit Intel CPU)
* Rust 1.37
* Cargo 1.37

Other systems might work too (rust supports quite a number of
systems), but have not yet been tested.

Building on **Microsoft Windows** might work.

## Download this repository

If you read this readme on github, you should see a *clone or download* button.
Click it and follow the instructions. Alternatively, you can copy the follow
command into your terminal:

```sh
git clone https://github.com/LarsEKrueger/sesd.git
```

For the following steps, it is assumed that you did that.

## Build sesd

The following command performs all the steps:
```sh
cd sesd
cargo build
```

It should produce a binary at `./target/debug/sesd` which can be started.

If you want to install the release binary somewhere else, do this:

```sh
cargo install --root $HOME/somewhere/else
```

## Reporting bugs

I'd be grateful for any reported bug. Please navigate to [sesd's issue
tracker](https://github.com/LarsEKrueger/sesd/issues) and follow the procedure
outlined below. It will ensure that your bug can be reproduced and addressed.

* Is there a similar bug already reported? If so, add any missing specifics of
  your system / situation to the discussion.
* Create a new issue.
* Describe the difference between expected and experienced behaviour.
* Add any error or warning messages that the compilation process generated.
* If you encounter a build error, add the output of the following commands:
  ```sh
  cargo clean
  cargo build -vv
  ```
* Add your rust version (*rustc --version*).
* Add your cargo version (*cargo --version*).
* Add you gcc version (*gcc --version*).
* Add your linux version (*uname -a*). You can censor the hostname and the date of build if you like.
* Add the SHA1 of the version you checked out or downloaded.
    * If you downloaded the ZIP, run
      ```sh
      unzip -v sesd-master.zip
      ```

      and report the string of numbers and letters in the second line (just above the file table).
    * If you cloned the repository, run
      ```sh
      git rev-parse HEAD
      ```

      and report it's output.

# Roadmap

The roadmap is defined in terms of features of the [demo
program](src/bin/sesd/readme.md). This directly corresponds to the library
features that implement them.

* [X] 0.1 - Simple syntax directed editor
    * One, compiled-in language
* [ ] 0.2 - General syntax directed editor
    * Dynamically loaded grammars and templates
    * Compile library to webassembly, provide interactive grammar debugger

# TODO

* [ ] Consistent and unambigous names for the various types of indices.
* [ ] BUG: Wrapping long lines doesn't work correctly.
* [ ] Compile grammar tables using macros (no dynamic memory)
* [ ] Join style sheet and parser
* [ ] Reduce number of allocations

# LICENSE issues

This crate is MIT licensed. Keep that in mind when you add e.g. grammars. The
BASH grammar, for instance, is licensed under GPL v3. Thus, adapting the
grammar to SESD would most likely constitute 'creating a derived work' and the
resulting grammar is therefore licensed as GPL v3 too. Thus, it cannot compiled
into SESD without changing the license of SESD to GPL v3 too.

As soon as SESD can load grammars from separate files, the BASH grammar (and
all other GPL licensed grammars) can be loaded. Thus, only the grammar
definition file is licensed under GPL v3 and subject to free redistribution.

*If you plan to use SESD in a commerical product, using a GPL v3 licensed
grammar, check these issues with a lawyer.*

As such, PRs for grammars need to be checked for the license of the grammar.
