[![Build and
test](https://github.com/kpagacz/elv/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/kpagacz/elv/actions/workflows/rust.yml)

# Elv

## Table of Contents:

1.  [Introduction](#introduction)
2.  [Installation](#installation)
3.  [Uninstallation](#uninstallation)
4.  [Examples](#examples)
5.  [FAQ](#faq)
6.  [Configuration](#configuration)
7.  [Get Help](#help)

## Introduction

`elv` is a command line interface program that lets you interact with
[Advent of Code](https://adventofcode.com) API. Advent of Code is a yearly event
that happens around Christmas time. Each day of the event, one
algorithmic riddle becomes available on its site and everyone can join
by solving it and submitting their answers to it.

`elv` helps you interact with with Advent of Code via your terminal
instead of the webpage. So far `elv` supports:

- downloading riddles' description
- downloading a riddle's input for a given year and day
- submitting answers to a riddle
- automatically guessing what part of the riddle you want to submit
- getting the official leaderboards for a given year
- guessing the year and day of a riddle based on the current date
- caching `AoC` responses whenever possible, so you minimize your
  footprint on `AoC`'s servers

## Installation

### Using `cargo`

[`cargo`](https://doc.rust-lang.org/cargo/) is `Rust`'s package manager.
You can use it to install `elv` directly from
[`crates.io`](https://crates.io/). Once you install `cargo`, you can do
it by running the below command in your terminal:

```console
cargo install elv
```

After the installation, `elv` should be available from your CLI.

### Using `homebrew`

`Homebrew` is the self-described "Missing Package Manager for macOS (or
Linux)". If you want to install `elv` using `homebrew`; first you need
to [install `homebrew` itself](https://brew.sh/). Then, run the below in
your terminal:

```console
brew install kpagacz/elv/elv
```

`elv` is hosted on a private tap (if you are into `homebrew`'s
terminology), which is essentially a [GitHub repository](https://github.com/kpagacz/homebrew-elv).
By default, `homebrew` installs the latest version of the application available in
the repository. To install one of the previous versions, you must check
out a specific commit corresponding to that version.

### Downloading a binary

`elv` publishes several executables for different operating systems and
architectures. Head to the [releases subpage](https://github.com/kpagacz/elv/releases)
to check out the latest released version of `elv`.

#### Choose the binary matching your operating system and architecture

The archived binaries follow a simple naming scheme:
`elv-{version}-{target-os-and-arch}.zip`. Match your operating system
with the file name and the architecture:

- Windows: look for one of the Windows binaries.
- Apple: if you use one of the Silicon processors, download the
  `aarch64-apple-darwin` target; otherwise, download the other one.
- Linux: get one of the Linux distributions.

The choice between the `GNU` version and the other depends on whether
you have `GNU` installed. If yes, then go ahead and grab the appropriate
`GNU` version.

#### Run the binary

The archives in each release contain a single executable file. Unpack
the file. You need to put this executable file on your `PATH`, which
translates to either unpacking the `zip` file to one of the directories
already on `PATH` or anywhere you want and adding this location to
`PATH`. If inspecting or changing your `PATH` is unclear, I recommend:

- Windows:
  https://www.h3xed.com/windows/how-to-add-to-and-edit-windows-path-variable
- Linux/macOS: https://opensource.com/article/17/6/set-path-linux

After that, you should be able to call `elv` directly in your CLI:

```console
elv
```

### Installing from source

You can create your own executable from this repository if you possess a
`Rust` compiler and [`cargo`](https://doc.rust-lang.org/cargo/). The
steps are:

1.  Clone the repository. You can use a terminal to clone the
    repository, e.g.:

    ```console
    git clone git@github.com:kpagacz/elv.git
    # or
    https://github.com/kpagacz/elv.git
    ```

2.  Install `elv`. Navigate towards the `elv` directory and run:

    ```console
    cargo install --path .
    ```

## Uninstallation

### Removing configuration files and caches

`elv` uses a configuration file and caches when running. You can list
the directories `elv` uses by running:

```console
elv list-dirs
```

The output lists the directories. If you want your configuration file
gone and the cache gone as well, just remove these directories from your
system.

### Cargo

```console
cargo uninstall elv
```

### Installed from binary

Delete the binary from your system.

### Installed via brew

```console
brew uninstall kpagacz/elv/elv
brew autoremove
```

## Examples

You need an Advent of Code session token to interact with its API. `elv`
does not support authentication to the API on its own, so you need to
get your token beforehand.

### Getting and setting the session token - **IMPORTANT**

You will need to [log into Advent of
Code](https://adventofcode.com/2022/auth/login). AoC site sends the
session token back to you using cookies. So you need to inspect the
cookies and get the one named `session` value. This is your session
token you can use with `elv`. The session token is valid for about a
month, so remember to get another once the old one expires.

Once you get your session token, we recommend to set it in `elv`'s
configuration. You can do it by running:

```console
elv config set aoc.token <your-session-token>
```

See the [section about storing the session key](#how-can-i-store-the-session-token)
for other ways to store the session token.

If you do not get the session token, you will not be able to interact
with Advent of Code API using `elv`.

### Downloading the description

#### Getting today's riddle description

This works only while the event is being held, not all the time of the
year. While the event is not held, you need to specify the year and day
of the challenge explicitly using `-y` and `-d` parameters.

```console
elv desc
```

#### Getting a description of a particular riddle

You specify the day and the year of the riddle.

```console
elv desc -y 2021 -d 1
# Prints the description of the riddle published on the 1st of December 2021
```

### Downloading the input

#### Getting today's riddle input

This works only while the event is being held, not all the time of the
year. While the event is not held, you need to specify the year and day
of the challenge explicitly using `-y` and `-d` parameters.

```console
elv input
```

#### Getting input for a particular riddle

You specify the day and the year of the riddle.

```console
elv input -y 2021 -d 1
# downloads the input for the riddle published on the 1st of December 2021
```

### Submitting the solution

#### Submitting the solution for today's riddle

This works only while the event is being held, not all the time of the
year. While the event is not held, you need to specify the year and day
of the challenge explicitly using `-y` and `-d` parameters.

```console
elv submit <SOLUTION>
elv submit <SOLUTION>
```

#### Submitting the solution for a particular riddle

You specify the day and the year of the riddle.

```console
elv submit -y 2021 -d 1 <SOLUTION>
```

#### Specifying the part of the riddle

`elv` tries to guess the part of the riddle you are solving based on the
riddle description. If for some reason you want to override it, or the
application fails to guess the part, you can specify it explicitly:

```console
elv submit -y 2021 -d 1 <SOLUTION> one
elv submit -y 2021 -d 1 <SOLUTION> two
```

### Getting the leaderboard

#### Getting the leaderboard for this year

This works only while the event is being held, not all the time of the
year. While the event is not held, you need to specify the year
explicitly using `-y` parameter.

```console
elv leaderboard
```

#### Getting the leaderboard for a particular year

You specify the year of the leaderboard.

```console
elv leaderboard -y 2021
```

### Guessing the year and the day of the riddle

`elv` can guess the year and day of the riddle you are working on. It
does so by looking at the current date and arguments `year` and `day`
passed to `elv`. The application supports the following scenarios:

* if you do not pass the `year` and `day` arguments:
  * if it is December and the event is held, `elv` will guess you are
    working on the current day's riddle:
    ```console
    # Date is December, 6th
    elv desc
    # will download the description of the riddle published on the 6th of December
    ```
  * if it is not December, `elv` will not be able to guess the date and
    will ask you to provide the `year` and `day` arguments
    ```console
    # Date is November, 24th
    elv desc # will not produce any description
    ```
* if you pass only the `day` argument:
  * if it is December and the event is held, `elv` will guess you are
    working on the current year's riddle
    ```console
    # Date is December, 6th
    elv desc -d 4
    # will download the description of the riddle
    # published on the 4th of December the same year
    ```
  * if it is not December, `elv` will guess you meant the last year's
    event
    ```console
    # Date is November, 24th
    elv desc -d 4
    # will download the description of the riddle
    # published on the 4th of December last year
    ```
* if you pass only the `year` argument, `elv` will not be able to guess the
  date and will ask you to provide the `day` argument
* if you pass the `year` and `day` arguments, `elv` will use them regardless
  of the current date
  ```console
  elv desc -y 2021 -d 4
  # will download the description of the riddle
  # published on the 4th of December 2021
  # regardless of the current date
  ```
## FAQ

### How can I store the session token?

You can set the session token in `elv`'s configuration using multiple
ways, the most convenient being:

1. Set it in the configuration using the CLI:

    ```console
    elv config set aoc.token 01234567890123456789abcdefghi
    ```

1.  Passed as an argument to `elv` with the `-t` parameter:

    ```console
    elv input -t <YOUR TOKEN HERE>
    # or
    elv input --token <YOUR TOKEN HERE>
    ```

    As a live example:

    ```console
    elv input -t 01234567890123456789abcdefghi
    ```

1.  As an environment variable. `elv` looks for an environmental
    variable `AOC_TOKEN` while searching for your session token.
    Example:

    ```console
    export AOC_TOKEN=0123456789abcdefghi
    elv input
    ```

    Despite the fact we have not provided the value for the `--token`
    parameter, `elv` will pick the value of `AOC_TOKEN` and use it as a
    token.

1.  In a configuration file. `elv` creates a configuration file in your
    home directory. You can find the configuration file in a directory
    listed by running `elv list-dirs` in your terminal. You should store
    the token in the `[aoc]` section, under the `token` key:

    ```toml
    [aoc]
    token = "<YOUR TOKEN HERE>"
    ```

### What if I store the session token in the configuration file and pass it as an argument to `elv`?

`elv` looks for your token in three places, starting from the first on
the below list and moving to the next one if it did not find the token
already.

1. `elv`'s configuration file.
1. The `AOC_TOKEN` environment variable.
1. The `-t` parameter to the CLI command.

### How can I get the value of the session token to pass it to `elv`?

The session token is sent to your HTTP client (usually your browser) as
a cookie when you log into the Advent of Code web page. The easiest way
to get the value of a cookie is by using your browser's built-in
inspection tools. Look for a way to inspect the cookies specific to your
browser.

### Where is the configuration file?

All the directories `elv` uses can be listed by running:

```console
elv list-dirs
```

## Configuration

The application suppports a number of parameters in the configuration file.
You can find the configuration file by invoking:

### See the current configuration

```console
elv config list
# or
elv config l
```

### Set a configuration parameter

```console
elv config set <PARAMETER> <VALUE>
```

For example, changing the width of the output to 100 characters:
```console
elv config set cli.output_width 100
```

### Configuration file

```console
elv list-dirs
# cache: /Users/konradpagacz/Library/Caches/elv
# config: /Users/konradpagacz/Library/Application Support/elv
```

which prints the paths used by `elv` on your machine. Track down the one
named `config` and open the `.config` file inside the directory.

### Configuration parameters

The configuration file is written in `TOML`. You can set the following values

- `aoc.token` - the token used by the application to authenticate you while
  connecting to `AOC` servers
- `cli.output_width` - the column width of the output when calling
  `elv description`

## Help

If something is not working as expected, or you have an idea for an improvement,
feel free to contact me at: `konrad.pagacz@gmail.com`, open an issue or a pull
request.
