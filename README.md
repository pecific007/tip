# Tip:
![License](https://img.shields.io/badge/license-BSD_2_Clause-blue.svg) \
TIP: TODOs-Issues-Progress \
It is a todo/issue tracker. It is inspired by [tatr](https://github.com/tsoding/tatr). \

For this project a `tip` is used to refer to either the program itself, or it to issue/todo depending on the context.

### Usage:

```console
$ tip
Error parsing arguments: Not enough arguments provided

Usage: tip <project> [command] (function)
    -- project:
          <project>        The name of the project
    -- commands:
           create          to create project|tip
           update          to update project|tip
           delete          to delete project|tip
           show            to search project|tip
    -- functions:
           @project        create new project
           @tip            create new tip
           @where          specify a condition
Not implemented: use `help command|function <name>` to learn more about them.
```
```bash
$ tip imgv create @project
Created: new Project: imgv
@path: /home/pecific007/Projects/c-lang/imgv

$ tip imgv create @tip 4 Add file explorer
Created: new Tip: Add file explorer
@id: 001
@critical: 4
@done: false
```
Not yet implimented: it's the general idea of how these functions would work:
```console
$ tip imgv show @all
@id: 001 @title: Add file explorer @critical 4 @done: false

$ tip imgv show @where done
Tip: No data found.
$ tip imgv update @set id is 001 @done:true
Tip: Marked @id: 001 @title: Add file explorer @critical 4 as done.

$ tip imgv update @set title like 'file explorer' @critical:8
@id: 001 @title: Add **file explorer** @critical 8 @done: true

$ tip imgv show @where title like 'Add' and id like 001
@id: **001** @title: **Add** file explorer @critical: 4 @done: true
```

This will create a directory `$HOME/.tip/` (`tip_dir`) where `$HOME` is an environment variable (in UNIX /home/&lt;username&gt;). \
Creating a new project it will create a new directory as the same name as the project in the `tip_dir` so for this example it will create a directory `tip_dir/imgv`. And it will also create a `000.tip` file that will contain the meta-data about the project like the location of the project etc.
\

- The `000.tip` file will have this data:
  ```tip
  @project:imgv
  @path:/home/pecific007/Projects/imgv/
  @count:2
  ```
Adding a new `tip` will create a file in the project file (`tip_dir/imgv`) with it's name being the `id` like `001.tip`.

- The `001.tip` file will have this data:
  ```tip
  @title:Add file explorer
  @critical:4
  @done:false
  ```
  will be adding more data like date-time, etc.

## TODOs:
- [ ] implement commands
  - [x] create
  - [ ] delete
  - [ ] update
  - [ ] show
- [ ] implement functions
  - [x] project
  - [x] tip
  - [ ] where
  - [ ] set
- [x] separate into crates and modules
- [ ] decide on a good structure for providing command lines arguments

### Examples:
1) This will create a new project called imgv
    ```bash
    $ tip imgv create @project
    ```

2) This will create a new tip with title = 'fix bug in main.c at line 30' and critical = 10
    ```bash
    $ tip imgv create @tip 10 'fix bug in main.c at line 30'
