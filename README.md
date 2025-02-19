# git-biance

[中文文档](./README-zh.md)

biance（鞭策，biān cè，spur）is a small rust program that shows and visualizes code contributions in a git repository.

<div align="center">⚠️*Do not* use it to harass your teammates⚠️</div>

|                                `git biance --commits --plot`                                |                                `git biance --stat --plot`                                 |
| :-----------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------: |
| ![commits](https://github.com/user-attachments/assets/6fdcb9cd-44aa-4918-b8dd-6d3a27b850bd) | ![stats](https://github.com/user-attachments/assets/23c69509-fd12-42bc-8b46-9aa9ffe08543) |

## Installation

```shell
cargo install git-biance
```

## Usage

```
Usage: git-biance [OPTIONS] [AUTHOR]

Arguments:
  [AUTHOR]  Specify certain author

Options:
  -s, --stat            Show total insertions and deletions
  -c, --commits         Show total commits
  -p, --plot            Visualize contributions with a graph
  -f, --file <FILE>...  Show insertions and deletions on single file
  -h, --help            Print help
  -V, --version         Print version
```

## FAQ

> I see some names are repeated, what is going on?

`git-biance` use email to identify users, but shows them by user name. It's likely that some contributors are using git with http, which generates a new email each time they push a commit. One can get the correct output by using [git mailmap](https://git-scm.com/docs/gitmailmap). To get a list of authors, run:

```shell
git log | grep Author | sort | uniq
```
