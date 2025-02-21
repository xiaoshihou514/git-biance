# git-biance

![Crates.io Version](https://img.shields.io/crates/v/git-biance)

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

## Integrations

### GitLab Integration

The following workflow can be used to achieve automatic spurring effect in merge requests, requires environment variable named
CI_AUTOCOMMENTER_API_KEY containing a personal access code:

```yaml
# Don't forget to change the gitlab instance
biance:
  stage: deploy
  variables:
    GIT_DEPTH: 0
  script:
    - BIANCE_COMMIT=$(git biance -c | sed ':a; N; $!ba; s/ /%20/g; s/\n/%0A/g')
    - BIANCE_STATS=$(find src/main/ -type f | xargs git biance -f | sed ':a; N; $!ba; s/ /%20/g; s/\n/%0A/g')
    - BIANCE_MSG=$(echo "Beep%20boop%0A%0A$BIANCE_COMMIT%0A%0A$BIANCE_STATS")
    - 'curl --request POST --header "PRIVATE-TOKEN: $CI_AUTOCOMMENTER_API_KEY" "https://your.gitlab.instance.com/api/v4/projects/$CI_PROJECT_ID/merge_requests/$CI_MERGE_REQUEST_IID/notes?body=$BIANCE_MSG"'
  rules:
    - if: $CI_PIPELINE_SOURCE == 'merge_request_event'
```

![Effect](https://github.com/user-attachments/assets/ea9ae5a6-d51d-441d-9cc0-f484f34ad614)

## FAQ

> I see some names are repeated, what is going on?

`git-biance` use email to identify users, but shows them by user name. It's likely that some contributors are using git with http, which generates a new email each time they push a commit. One can get the correct output by using [git mailmap](https://git-scm.com/docs/gitmailmap). To get a list of authors, run:

```shell
git log | grep Author | sort | uniq
```
