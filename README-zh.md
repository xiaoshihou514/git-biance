# git鞭策

git-鞭策是一个用rust写的，用来可视化队友干了多少活的小程序。

<div align="center">⚠️请*不要*用这个来压力队友⚠️</div>

|                                `git biance --commits --plot`                                |                                `git biance --stat --plot`                                 |
| :-----------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------: |
| ![commits](https://github.com/user-attachments/assets/6fdcb9cd-44aa-4918-b8dd-6d3a27b850bd) | ![stats](https://github.com/user-attachments/assets/23c69509-fd12-42bc-8b46-9aa9ffe08543) |

## 安装

```shell
cargo install git-biance
```

## 用法

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

## 常见问题

> 有些人的名字出现了好几次是咋回事？

`git鞭策`用邮箱来区分用户，但是打印的是用户名。最可能的是你的队友没用ssh，所以每次推送邮件都不一样。你可以用[git mailmap](https://git-scm.com/docs/gitmailmap)来解决这个问题，运行以下命令获取所有推送过的人的用户名和邮件：

```shell
git log | grep Author | sort | uniq
```
