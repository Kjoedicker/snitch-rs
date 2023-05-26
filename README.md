<br />
<div align="center">
<h3 align="center">SnitchRs</h3>

  <p align="center">
    A tool for tracking issues within source code. Inspired <a href="https://github.com/tsoding/snitch#example-1">by</a>
    <br />
    <br />
    <br />
    <!-- <a href="https://github.com/github_username/repo_name">View Demo</a> -->
    ·
    <a href="https://github.com/kjoedicker/snitch-rs/issues">Report Issue</a>
    ·
    <a href="https://github.com/kjoedicker/snitch-rs/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
# About The Project

It's common practice to leave a quick `// TODO: come back and fix this or add that` type line when working through a build.

`SnitchRs` provides a way to find and track these lines


## Usage:
  * `snitch` - Find and report untracked issues
  * `peek`   - List existing issues
  * `audit`  - Audit and remove completed issues

## Snitch:
Snitch finds lines that match the following pattern: `// TODO: some thing`  and does two things:

1. Reports the line through the issues tracker
2. Ties a issue number to the line: `// TODO(#1): some thing`

Example:

**Before:**
```javascript
function returnUndefined() {
    return;
}

// TODO: return something
function returnSomething() {
    return "something"
}
```

**After:**
```javascript
function returnUndefined() {
    return;
}

// TODO(#1): return something
function returnSomething() {
    return "something"
}
```

## Peek
Peek grabs the first 10 issues in reported in the issue tracker. In this case github.

```
+-----+-----------------------------------------------------+---------------------+
| Id  | Url                                                 | Title               |
+=================================================================================+
| 100 | https://github.com/Kjoedicker/snitch-lab/issues/100 | Do the thing        |
|-----+-----------------------------------------------------+---------------------|
| 101 | https://github.com/Kjoedicker/snitch-lab/issues/101 | Fix the thing       |
|-----+-----------------------------------------------------+---------------------|
| 102 | https://github.com/Kjoedicker/snitch-lab/issues/102 | Implement the thing |
|-----+-----------------------------------------------------+---------------------|
| 103 | https://github.com/Kjoedicker/snitch-lab/issues/103 | Refactor the thing  |
|-----+-----------------------------------------------------+---------------------|
| 104 | https://github.com/Kjoedicker/snitch-lab/issues/104 | Rinse and repeat    |
+-----+-----------------------------------------------------+---------------------+
```

NOTE: for projects with more than 10 issues this is limiting. There is an active issue to add some form of pagination: https://github.com/Kjoedicker/snitch-rs/issues/126

## Audit
Runs through the project and validates if a issue line is still in issue, removing the issue if needed.

For example, there is a todo that is lingering around that has been closed out in `github`:

**Before:**
```javascript
function returnUndefined() {
    return;
}

// TODO(#1): return something
function returnSomething() {
    return "something"
}
```

**After:**
```javascript
function returnUndefined() {
    return;
}

function returnSomething() {
    return "something"
}
```
