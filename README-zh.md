# git鞭策

git-鞭策是一个用rust写的，用来可视化队友干了多少活的小程序。

<div align="center">⚠️请*不要*用这个来压力同事，开开玩笑就得了⚠️</div>

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

## 集成

### GitLab集成

用以下流程可以实现合并请求中自动鞭策的效果，需要成员添加名为CI_AUTOCOMMENTER_API_KEY的变量，值为个人授权码：

```yaml
# 别忘了把gitlab示例的链接改了
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

![效果](https://github.com/user-attachments/assets/ea9ae5a6-d51d-441d-9cc0-f484f34ad614)

## 常见问题

> 有些人的名字出现了好几次是咋回事？

`git鞭策`用邮箱来区分用户，但是打印的是用户名。最可能的是你的队友没用ssh，所以每次推送邮件都不一样。你可以用[git mailmap](https://git-scm.com/docs/gitmailmap)来解决这个问题，运行以下命令获取所有推送过的人的用户名和邮件：

```shell
git log | grep Author | sort | uniq
```
