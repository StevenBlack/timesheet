# 🕑 A daily timesheet aggregator for `git log` and MacJournal

A personal project, written in [rust](https://www.rust-lang.org/).
## This is an [aggregator](https://www.enterpriseintegrationpatterns.com/patterns/messaging/Aggregator.html)

<a href="https://www.enterpriseintegrationpatterns.com/patterns/messaging/Aggregator.html"><img src="./aggregator.gif"></a>

> The ***Aggregator*** is a special Filter that receives a stream of messages and identifies messages that are correlated. Once a complete set of messages has been received (more on how to decide when a set is 'complete' below), the Aggregator collects information from each correlated message and publishes a single, aggregated message to the output channel for further processing.

We aggregate records from different sources — [`git log`](https://git-scm.com/docs/git-log) and [MacJournal](https://danschimpf.com/) — into one literate timesheet paragraph per day.

## Background

I use `git log` and (the most excellent) [MacJournal](https://danschimpf.com/) for journalling.
I merge these two sources to create a one paragraph summary of my project day.

This is a tedious manual process.  Automate it!

<img width="378" alt="automate-all-the-things" src="https://user-images.githubusercontent.com/80144/145323062-b6fadc17-34ff-4369-baea-de9420f9f288.png">

## Raw input from `git log`

My `git log` command is shaped like this:

```
git log --pretty="format:%cd %s" --since="60 days ago" --date="short" >> "gitoutput.txt"
```
... which produces a log that like this:

```
2021-12-01 0.13.6
2021-12-01 Issue #2971 and #2951: assessing the sorry-ass state of our data pipeline.
2021-12-01 Issue #3341: check the weekend schedule, why is there is no Saturday schedule this week.
2021-12-01 Issue #404: fix — promote the Job screen to the main menu, and rename it "Upcoming Jobs".
2021-12-01 Issue curation.
2021-12-01 Afternoon coordination meeting with QA.
2021-12-01 Issue 405: refactor — reorganise this long query to make it easier to deconstruct.
2021-12-01 After hours — update test/dev data.
```
## Raw input from MacJournal export

Selecting a bunch of entries in MacJournal then right+click ➡️ Export gives me a bunch of dates, each day looks like this:

```
Date: 1 December 2021 at 12:15
Topic: Tasks today

Morning call with the Dev team.
Final testing of version 0.13.6.
Release version 0.13.6 to userland.
Issue #3138: adjust a part production report, and regenerate it.
Issue #3138: delete all the void entries from the production parts table.
Issue #3140: new issue about making reworked parts easy to move and merge.
Issue #3141: sales dept reports parts are missing in new estimates, so investigating that.
After hours — update test/dev data.
```
## What I need

Those inputs are aggregated into one line of literate text, on one line per day, that looks like this:

```
2021-12-01 Afternoon coordination meeting with QA.  etc...
```

## Details

The following apply:

### Git tags (status: 🟢)

**Input**: Lines with raw git tags look like this: "`0.13.6`".

**Output**: They should become the sentence "`Version 0.13.6 built, texted, and rolled out.`"

### Remove trivial commits (status: 🟢)

**Input**: Commits that say trivial things like "`Fix whitespace`" or just "`Whitespace`", or "`Fix typo`" or just "`Typo`", or the latin "[`Ibid.`](https://en.wikipedia.org/wiki/Ibid.)", are dropped.

**Output**: nothing.

### Repeated issue references (status: 🟢)

**Input**: Repeated commits referring to the same issue, originally this....
```
Issue #3138: fix — thing one.
Issue #3138: fix — thing two.
Issue #3138: fix — thing three.
```

**Output**: ... become (note the semicolon `;` delimiter and the period `.` at the end.)

```
Issue #3138: fix — thing one; fix — thing two; fix — thing three.
```

### Repeated version references (status: 🔴)

**Input**: Handwritten version number commits lke ...
```
2021-12-01 Something version 0.3.11
2021-12-01 Something version 0.3.12
2021-12-01 Something version 0.3.13
```

**Output**: ... become (note the semicolon `,` except with `and` as the last delimiter, and the period `.` at the end.)

```
Issue #3138: Something version 0.3.11, 0.3.12, and 0.3.13 built, tested, and rolled out.
```

## Usage

```
$ timesheet -h

timesheet 0.1.0
Timesheet from git log output and MaJournal export data.

USAGE:
    timesheet [FLAGS] [OPTIONS]

FLAGS:
    -d, --dryrun     Sets up a dry-run, does not timesheet create output 🔴
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Putput process information 🔴

OPTIONS:
    -g <gitlogfile>            The git log input file [default: ./commits.txt] 🟢
    -m <macjournalfile>        The MacJournal input file [default: ./macjournal.txt] 🟢
    -o <outfile>               Output file, stdout if not present 🔴
```

## For Example

Using this repository, for example:

```
$ git log --pretty="format:%cd %s" --since="60 days ago" --date="short" >> "gitoutput.txt"
$ timesheet -g gitoutput.txt > clean.txt
```

Yields this in `clean.txt`:

```
2021-12-06 First commit. Incremental improvement with a Commuitstruc
2021-12-07 Add to readme. Create readme.md. Improve the joining of elements, and handling of semver tags. Improvements on the MacJournal side. Minor tidy. Start processing the Macjournal export file.
2021-12-08 "Work in progress" Add feature status icons. Add structopt to dependencies. Better comment about these functions. Doesn't apply on the MacJournal side. More readme fixes. More readme refinements. More readme shenanegans. More readme tweaks. Remove commented code. Split everything into git and MacJournal modules. Test for the semver tag detector. Update the readme.
2021-12-09 Add another aspirational target feature to the readme. Add structop struct, and marshall the vecs from both git and macjournal modules. Cleanup in the macjournal module. Move git module functionality to main.rs. Refactor types and traits. Small reorg of the readme. Start roughing-in a basic config file named .timesheet.
2021-12-10 Don't need these tests. Fix test. New function commasand(), to be used for consolidated enumeration. Playing with structopt. Rename common.rs to utils.rs. Run everything through shared settings now.
2021-12-11 Better default file names. Dedup the git log and macjournal vecs as part of their processing. Implement structopt-toml to give us the ability to use a TOML config file anywhere up the parent folder tree. Improve help text. Introducing the .timesheet config file. Latest cargo.lock. Rename fn commasand to fn ommas_and(). Setting-up the dryrun option. Sort the vec. The config file is now named .timesheetrc and we have a .timesheetrc-sample file.
2021-12-12 Better comments for utils functions, now gradually moving to RustDoc compliance.
2021-12-13 Invoke commas_and() for semver commits.
2021-12-14 Add a test for semver commit processing. Better modularization for the semver consolidation test. Decent start on issue commit consolidation. Implement the parsing of issue commits. New trait on Commit: Isissueprefix. Optimize the semver regex definition so we don't build it with each function call. Remove test that doesn't belong here. Simplify naming to, simply, 'issue'.
```