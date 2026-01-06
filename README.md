# Algorithm Exercises (Rust 🦀)

[![Rust CI/CD Test](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/rust-test.yml/badge.svg)](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/rust-test.yml)
[![Rust CI/CD Lint](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/rust-clippy.yml)
[![Markdown Lint](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/markdown-lint.yml/badge.svg)](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/markdown-lint.yml)
[![YAML lint](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/yamllint.yml/badge.svg)](https://github.com/sir-gon/algorithm-exercises-rust/actions/workflows/yamllint.yml)

![GitHub](https://img.shields.io/github/license/sir-gon/algorithm-exercises-rust)
![GitHub language count](https://img.shields.io/github/languages/count/sir-gon/algorithm-exercises-rust)
![GitHub top language](https://img.shields.io/github/languages/top/sir-gon/algorithm-exercises-rust)
[![codecov](https://codecov.io/gh/sir-gon/algorithm-exercises-rust/branch/main/graph/badge.svg?token=YZ41BE67E4)](https://codecov.io/gh/sir-gon/algorithm-exercises-rust)

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=sir-gon_algorithm-exercises-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=sir-gon_algorithm-exercises-rust)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=sir-gon_algorithm-exercises-rust&metric=coverage)](https://sonarcloud.io/summary/new_code?id=sir-gon_algorithm-exercises-rust)
[![Bugs](https://sonarcloud.io/api/project_badges/measure?project=sir-gon_algorithm-exercises-rust&metric=bugs)](https://sonarcloud.io/summary/new_code?id=sir-gon_algorithm-exercises-rust)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=sir-gon_algorithm-exercises-rust&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=sir-gon_algorithm-exercises-rust)
[![Duplicated Lines (%)](https://sonarcloud.io/api/project_badges/measure?project=sir-gon_algorithm-exercises-rust&metric=duplicated_lines_density)](https://sonarcloud.io/summary/new_code?id=sir-gon_algorithm-exercises-rust)

## TL;DR

Algorithms Exercises solved in [Rust](https://www.rust-lang.org/)
Developed with TDD.

[![Rust](    https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://gcc.gnu.org/onlinedocs/gcc/Standards.html#C-Language)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

Go to [Install and run](#install-and-run)

## What is this?

This repository is part of a series that share and solve the same [objectives](#objetives),
with the difference that each one is based on a different software ecosystem,
depending on the chosen programming language:

- [Modern Javascript: algorithm-exercises-js](https://github.com/sir-gon/algorithm-exercises-js)
- [Typescript: algorithm-exercises-ts](https://github.com/sir-gon/algorithm-exercises-ts)

- [Python 3.x: algorithm-exercises-py](https://github.com/sir-gon/algorithm-exercises-py)
- [Go / Golang: algorithm-exercises-go](https://github.com/sir-gon/algorithm-exercises-go)

- [Java: algorithm-exercises-java](https://github.com/sir-gon/algorithm-exercises-java)
- [.NET / C#: algorithm-exercises-rustsharp](https://github.com/sir-gon/algorithm-exercises-rustsharp)
- [C++17: algorithm-exercises-rustpp](https://github.com/sir-gon/algorithm-exercises-rustsharp-cpp)

## Objetives

### Functional

- For academic purposes, it is an backup of some algorithm exercises
(with their solutions), proposed by various sources:
[leetcode, hackerrank, projecteuler](#algorithm-excersices-sources), ...

- The solutions must be written on "vanilla code", that is,
avoiding as much as possible the use of external libraries (in runtime).

- Adoption of methodology and good practices.
Each exercise is implemented as a unit test set,
using TDD (Test-driven Development) and Clean Code ideas.

### Technical

Foundation of a project that supports:

- Explicit **typing** when the language supports it, even when it is not mandatory.
- Static Code Analysis (**Lint**) of code, scripts and documentation.
- Uniform **Code Styling**.
- **Unit Test** framework.
- **Coverge** collection. High coverage percentage. Equal or close to 100%.
- **Pipeline** (Github Actions). Each command must take care of its
return status code.
- **Docker**-based workflow to replicate behavior in any environment.
- Other tools to support the reinforcement of software development **good practices**.

## Install and Run

- (⭐️) [Install and run directly with make](#install-and-run-directly-with-make)
require Rust tools installed in your SO.
<!--
- [Install and in Docker](#install-and-running-with-docker-) require Docker and
docker compose installed.
- [Install and in Docker with make](#install-and-running-with-docker--using-make)
require Docker, docker compose and GNU make installed. -->

<!-- ⭐️: Prefered way. -->

### Install and run directly with make

It is assumed that the following tools have already been installed:

- [**Install Rust**](https://www.rust-lang.org/tools/install)

Using Rust tools in your SO. You must install dependencies:

```bash
make dependencies
make lint # optional
make build
make test
```
<!--
#### Test run with alternative behaviors

Not implemented yet

#### Examples running tests with alternative behaviors

### Install and Running with Docker 🐳

Build an image of the test stage.
Then creates and ephemeral container an run tests.

```bash
docker compose build algorithm-exercises-rust
docker compose build algorithm-exercises-rust-lint
docker compose build algorithm-exercises-rust-test
docker compose build algorithm-exercises-rust-dev
```

```bash
docker compose --profile testing run --rm algorithm-exercises-rust-test
```

### Install and Running with Docker 🐳 using make

```bash
make compose/build
make compose/lint # optional
make compose/test
``` -->

## About development

```sh
rustc --version
```

```text
rustc 1.89.0 (29483883e 2025-08-04)
```

---

```sh
cargo --version
```

```text
cargo 1.89.0 (c24e10642 2025-06-23)
```

---

```sh
rustup --version
```

```text
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.89.0 (29483883e 2025-08-04)`
```

## Algorithm excersices sources

- [Leetcode](https://leetcode.com/) online platform for
coding interview preparation.
- [HackerRank](https://www.hackerrank.com/) competitive programming challenges
for both consumers and businesses.
- [Project Euler](https://projecteuler.net/) a series of computational problems
intended to be solved with computer programs.

Use these answers to learn some tip and tricks for algorithms tests.

### Disclaimer. Why I publish solutions?

As Project Euler says:

<https://projecteuler.net/about#publish>

```text
I learned so much solving problem XXX, so is it okay to publish my solution elsewhere?
It appears that you have answered your own question. There is  thing quite like that "Aha!" moment when you finally beat a problem which you have been working on for some time. It is often through the best of intentions in wishing to share our insights so that others can enjoy that moment too. Sadly, that will rarely be the case for your readers. Real learning is an active process and seeing how it is done is a long way from experiencing that epiphany of discovery. Please do not deny others what you have so richly valued yourself.

However, the rule about sharing solutions outside of Project Euler does not apply to the first one-hundred problems, as long as any discussion clearly aims to instruct methods, not just provide answers, and does not directly threaten to undermine the enjoyment of solving later problems. Problems 1 to 100 provide a wealth of helpful introductory teaching material and if you are able to respect our requirements, then we give permission for those problems and their solutions to be discussed elsewhere.
```

If you have better answers or optimal solutions, fork and PR-me

Enjoy 😁 !

## Status

### License

[LICENSE.md](LICENSE.md)

### Coverage

[![Coverage](https://codecov.io/gh/sir-gon/algorithm-exercises-rust/graphs/tree.svg?token=Q3B4ZT87O5)](https://codecov.io/gh/sir-gon/algorithm-exercises-rust)
