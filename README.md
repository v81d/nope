<p align="center">
  <img src="media/logo.png" alt="Logo" width="400" />
</p>

<p align="center">
  <img src="https://img.shields.io/github/languages/top/v81d/nope?style=for-the-badge" />
  <img src="https://img.shields.io/github/contributors/v81d/nope?style=for-the-badge" />
  <img src="https://img.shields.io/github/issues/v81d/nope?style=for-the-badge" />
  <img src="https://img.shields.io/github/downloads/v81d/nope/total?style=for-the-badge">
  <img src="https://img.shields.io/github/v/release/v81d/nope?style=for-the-badge" />
</p>

---

**nope** is a lightweight but powerful command-line tool for keeping track of commands you regret running. 

## Features

- Record regrets for commands along with a reason.
- Add, view, and remove entries quickly from the command line.
- Warns you when you run a regretted command!
- Multi-shell support (WIP).

## Supported Shells

At the moment, nope only supports **bash** and **zsh**, but support for more shells is being worked on!

## Quick Start

The following guide provides instructions on how to build and run nope.

### Prerequisites

You must have Rust and Cargo installed. If you don't already have them, you can do so by following the [official install guide](https://rust-lang.org/tools/install).

### Build Manually

After installing the requirements, you can build the project. To do so, follow the steps:

1. Clone the repository:

```bash
git clone https://github.com/v81d/nope.git
cd nope
```

2. Build the project:

```bash
cargo build --release
```

Now, the project should be built and compiled into an executable in `target/release/nope`.

3. Install the project (optional):

```bash
cargo install --path .
```

Make sure Cargo is in your `$PATH`. If it's not, nope will not be in your `$PATH` either.

### Running nope

To run nope, first make sure the project has already been built successfully and is in your `$PATH`. Then, launch the help page:

```bash
nope --help
```

This will display a help page describing nope and its command usage.

## Usage

nope is a command-line tool, meaning it is designed to be run from your terminal.

### Initializing the Shell (IMPORTANT)

To initialize nope for your shell, follow the examples below.

1. For Bash, put this in your `.bashrc`:

```bash
eval "$(nope init bash)"
```

2. For Zsh, put this in your `.zshrc`:

```zsh
eval "$(nope init zsh)"
```

### Adding a Regret

To add a new regret, type `nope add <COMMAND> <REASON>` with the command and reason in your terminal.

### Removing a Regret

To remove an existing regret, type `nope remove <COMMAND>` with the command in your terminal.

### Listing Regrets

To list your existing regrets, type `nope list` in your terminal.

You should see a table displaying the command, reason, and timestamp of each regret.

### Checking a Command

To check a command against your regrets list, type `nope check <COMMAND>` with the command in your terminal.

If you see an output, the command is in your regrets list. Otherwise, the command is not in your list.

### Clearing All Regrets

> [!CAUTION]
> This is a highly destructive action.

To clear ALL regrets from your entire regrets list, type `nope clear` in your terminal.

This will reset your entire configuration and remove every single regret you ever added.

## Contributing

### Reporting Issues

To report an issue or bug, visit nope's [issue tracker](https://github.com/v81d/nope/issues) on GitHub.

### Pull Requests

To push your features or fixes into this official repository:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/my-feature`) or a fix branch (`git checkout -b fix/my-fix`).
3. Commit your changes (`git commit -m "feat: add new feature"`). **Please follow the [Conventional Commits](https://www.conventionalcommits.org) guideline when doing so!**
4. Push the branch (`git push origin feature/my-feature`).
5. Open a pull request with `contrib` as the base branch. Make sure to create a detailed title and description of your change.

Please follow the [GitHub flow](https://guides.github.com/introduction/flow) and nope's [Code of Conduct](CODE_OF_CONDUCT.md) when submitting a pull request.

## License

nope is free software distributed under the **GNU General Public License, version 3.0 or later (GPL-3.0+).**

You are free to use, modify, and share the software under the terms of the GPL.
For full details, see the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html).
