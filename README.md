# rustful

rewrite of [rusty](https://github.com/locusst/rusty) but with a different approach to templating

## Usage

```bash
$ rustful [options]
```

## Options

* `-h`, `--help` - Show this help message
* `-v`, `--version` - Show the version
* `-s`, `--source` - The source directory
* `-o`, `--output` - The output directory

## Example

```bash
$ rustful -s source -o output
```

## Config

The config file is located at `source/config.toml` and has the following structure:

```toml
title = "My Blog"
description = "A blog about stuff"
```

## Posts

Posts are located in source/posts and have the following structure:

```md
---
title: My Post
date: 2014-01-01
author: John Doe
---

# My Post
```