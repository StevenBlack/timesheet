# 🕑 `git log` and MacJournal parser

A personal project, written in rust.

This repo consolidates daily activity from two different sources and produces one consolidated line for each work day.

## Background

When I track time on software projects, I use `git log` reports, and ad-hoc daily notes kept in (the most excellent) [MacJournal](https://danschimpf.com/). I merge these two sources to create one spreadsheet cell per day. This is a tedious manual process.

<img width="378" alt="automate-all-the-things" src="https://user-images.githubusercontent.com/80144/145323062-b6fadc17-34ff-4369-baea-de9420f9f288.png">


### Output from `git log`

The `git` command looks like this...

```
git log --pretty="format:%cd %s" --since="60 days ago" --date="short" >> "gitoutput.txt"
```
... which produces output like this:

```
2021-12-01 Version 0.10.27 built, tested, and rolled out.
2021-12-01 After hours — update test/dev data.
2021-12-01 Issue #2971 and #2951: assessing the sorry-ass state of our data pipeline.
2021-12-01 Issue #3341: check the weekend schedule, why is there is no Saturday schedule this week.
2021-12-01 Issue #404: fix — promote the Job screen to the main menu, and rename it "Upcoming Jobs".
2021-12-01 Issue curation.
2021-12-01 Afternoon coordination meeting with QA.
2021-12-01 Issue 405: refactor — reorganise this long query to make it easier to deconstruct.
```
## Output fro MacJournal export

```
Date: 1 December 2021 at 12:15
Topic: Tasks today

Morning call with the Dev team.
Final testing of version 0.13.6.
Release version 0.13.6 to userland.
0.13.6
Issue #3138: adjust a part production report, and regenerate it.
Issue #3138: delete all the void entries from the production parts table.
Issue #3140: new issue about making reworked parts easy to move and merge.
Issue #3141: sales dept reports parts are missing in new estimates, so investigating that.
After hours — update test/dev data. 
```

## What I need

All the above input on one line per day that looks like this:

```
2021-12-01 Version 0.10.27 built, tested, and rolled out. After hours ... etx...
```
One consolidated line, per day.

## Refinements

The following refinements should apply

### Git tags

Git tags are embellished, like this: "`0.13.6`" becomee the sentence "`Version 0.13.6 built, texted, and rolled out.`"

### Repeated issue references

This....
```
Issue #3138: fix — thing one.
Issue #3138: fix — thing two.
Issue #3138: fix — thing three.
```

... becomes (note the semicolon `;` delimiter and the period `.` at the end.)

```
Issue #3138: fix — thing one; fix — thing two; fix — thing three.
```
