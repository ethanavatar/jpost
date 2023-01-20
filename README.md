# jpost

A command line tool to create new posts in a Jekyll project 

## Contents

- [Usage](#usage)
- [Installation](#installation)
- [To Do](#to-do)

## Usage

```bash
$ jpost --help
Command line arguments

Usage: jpost.exe [POST_TITLE]

Arguments:
  [POST_TITLE]  Title of the post [default: "New Post"]

Options:
  -h, --help  Print help
```

While in the root directory of your Jekyll project, run the following command:

```bash
$ jpost "My New Post"
Post created: 2023-1-20-my-new-post.md
```

This will create a new post in the `_posts` directory with Jekyll's preferred naming convention, `YYYY-MM-DD-title-of-post.md`.

The contents of the post will be a basic front matter template with the title and date filled in:

### 2023-1-20-my-new-post.md
- - -

```markdown
---
layout: post
title: "My New Post"
date: 2023-1-20
---

```

## Installation

Install using cargo:

```bash
$ git clone https://github.com/ethanavatar/jpost
$ cd jpost
$ cargo install --path .
```

## To Do

- [ ] Publish as a crate
- [ ] Add support for custom post templates
