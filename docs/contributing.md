---
title: Contributing
---

# Contributing

## contribute workflow

1. report a issue
2. assign issue to you
3. create a draft pull request with the issue number included
4. pass CI checking
5. change the status of PR to "Ready for review"
5. more than 3 members review and approve
6. merge PR

## code style 

Please read [contributing.md](/docs/code-style.md)

## PR branch naming style

branch naming is kebab-case and require prefix in one of follow type

- feature: this PR introduces a new feature to the codebase
- fix: this PR patches a bug in codebase
- refactor: this PR changes the code base without new features or bugfix
- ci: this PR changes build/testing/ci steps
- docs: this PR changes the documents
- chore: this PR only has small changes that no need to record, like coding styles

git branch name example: chore/ci-add-typo-checking

## commit message style

`[branch_name](#issue_number): commit message`

git commit message example: `[chore/ci-add-typo-checking](#1): CI add typos cli tools to check typo`
