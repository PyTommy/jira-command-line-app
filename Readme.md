## 概要

## 使い方

1. [github releases](https://github.com/PyTommy/jira-command-line-app/releases)から、自分 local 環境にあった os の binaryfile をダウンロードして、適当な path に配置する。

```
j-<version>-osx // mac
j-<version>-linux // linux
j-<version>-windows // windows
```

2. binary ファイルを実行できるようにする

```sh
chmod +x /Users/taroyamada/dev/jira-command-line-app/local/j-1.0.0-osx // 例
```

3. zsh 修正(Mac)

```
# open ~/.zshrc
export JIRA_BASE_URL="https://xxxxx.atlassian.net" # JiraのURL
export JIRA_AUTH=<email>:<API_token> # API Token発行はこちら: https://id.atlassian.com/manage-profile/security/api-tokens
alias j=/Users/taroyamada/dev/jira-command-line-app/local/j-1.0.0-osx # binaryファイルのパス
```

```sh
source ~/.zshrc
```

4. 実行

```sh
j --help

# 担当者=me & inアクティブスプリントのIssueを複数選択し、markdownで吐き出す
j issues

# 担当者=me & inアクティブスプリントのIssueを選択し、issue番号とともに`git commit`
j commit
```

## 開発の仕方

0. 環境設定(zsh 修正)

```sh
export JIRA_BASE_URL="https://mycompany.atlassian.net"
export JIRA_AUTH=<email>:<API_token> # API Token発行はこちら: https://id.atlassian.com/manage-profile/security/api-tokens
```

1. rust をインストールする
2. `src/main.rs`を修正する
3. `cargo run <subcommand> <options>`を実行する
