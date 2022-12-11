# Elv

## Introuction

`elv` is a command line interface program that lets you interact with [Advent of Code](adventofcode.com) API.
Advent of Code is a yearly event that happens around the Christmas time. Eeach day of the event, one
algorithmic riddle becomes available on its site and everyone can join by solving it and submitting
their answers to it.

`elv` helps you interact with with Advent of Code via your terminal instead of the webpage. So far
`elv` supports:

- downloading a riddle's input for a given year and day
- submitting answers to a riddle

## Installation

## Uninstallation

## Examples

You need an Advent of Code session token to interact with its API. `elv` does not support authentication
to the API on its own, so you need to get your token beforehand.

### Getting the session token - **IMPORTANT**

You will need to [log into Advent of Code](https://adventofcode.com/2022/auth/login). AoC site sends
the session token back to you using cookies. You need to inspect the cookies and get the value of the
one named `session`. This is your session token you can use with `elv`. The session token is valid
for about a month, so remember to get another one, once the old one expires.

If you do not get the session token, you will not be able to interact with Advent of Code API using `elv`.

### Downloading the input

#### Getting today's riddle input

This works only while the event is being held, not all the time of the year.

```bash
elv -t <YOUR SESSION TOKEN> input
```

#### Getting input for a particular riddle

You specify the day and the year of the riddle.

```bash
elv -t <YOUR SESSION TOKEN> -y 2021 -d 1 input
# downloads the input for the riddle published on the 1st of December 2021
```

## FAQ

### How can I store the session token?

`elv` looks for your token in three places, starting from the first on the below list
and moving to the next one in case it did not found the token already.

1. Passed as an argument to `elv` with the `-t` parameter:

```bash
elv -t <YOUR TOKEN HERE> input
# or
elv --token <YOUR TOKEN HERE> input
```

As a live example:

```bash
elv -t 01234567890123456789abcdefghi input
```

2. As an environment variable. `elv` looks for an environmental variable `AOC_TOKEN`
   while searching for your session token. Example:

```bash
export AOC_TOKEN=0123456789abcdefghi
elv input
```

Despite the fact we have not provided the value for the `--token` parameter,
`elv` will pick the value of `AOC_TOKEN` and use it as a token.

3. In a configuration file. `elv` creates a configuration file in your
   home directory.

### How can I get the value of the session token?

The session token is sent to your http client (usually your browser) as a cookie,
when you log into the Advent of Code web page. The easiest way to get the value
of a cookie is using your browser's built-in inspection tools. Look for a way
to inspect the cookies specific to your browser.
