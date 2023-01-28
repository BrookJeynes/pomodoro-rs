
# Pomodoro-rs - Command-line Pomodoro timer
A simple pomodoro timer within your terminal. Track your tasks progress while you study


## Screenshots

![App Screenshot](https://via.placeholder.com/468x300?text=App+Screenshot+Here)


## Features

- Custom Pomodoro timer lengths
- Track your current tasks


## Run Locally

Clone the project

```bash
  git clone https://github.com/BrookJeynes/pomodoro-rs
```

Go to the project directory

```bash
  cd pomodoro-rs
```

Install dependencies

```bash
  cargo install
```

Run the application

```bash
  cargo run -- --help
```


## Documentation

### Writing tasks
By default, Pomodoro-rs will pull tasks in from a `tasks` file within the directory. You can specify a file directory else where with the `-t <file_path>` command.

Tasks are written in the following format:

```
---
title: Task 1
pomodoros_expected: 2
pomodoros_completed: 1
completed: false
---
title: Task 2
pomodoros_expected: 3
pomodoros_completed: 0
completed: false
---
```
