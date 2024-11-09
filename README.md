# cpgrader-rs

## Finish

sorted csv
docker

### Commands

- grade  Automatically grade students by dependcies
- score  Input students score for one problem
- build  Clean, unzip, make, copy depends to grader dir
- clean  Clean the grader dir
- log    Print the log
- csv    Create CSV of grade
- help   Print this message or the help of the given subcommand(s)

## docker usage

please copy all the student file into ./example/
Define a ./comfig.toml before build

```bash
docker build --rm -t cpgrader-rs:1.0 .
docker run -it -v $(pwd)/status:/usr/src/app/status cpgrader-rs:1.0
```

### toml example

```config.toml
[global]
homework = "MID"
testcase = 0
dependent = []

# example: dependent = ["mid03.c", "test.png",]   

[testcase01]
testcase = 0
#diffcommand = "colordiff -w -B -q"

[testcase02]
testcase = 0

[testcase03]
testcase = 20

[testcase04]
testcase = 20

[testcase05]
testcase = 20

```
