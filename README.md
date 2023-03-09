# teach-me-through-git
tmtg is a tool that navigates you through the git history so you can learn from it

## What is the idea?
I try to learn rust at the moment which brings me to two problems.

First: I need experimental projects which I can use to do so.
Second: The docs are very limited. They are good but I need practical advice how other programmers use the language and it seems that there are not many teachers out there. 

So I got an idea: 
What if you would be able to clone a github repository and then "step through" every committed set. You could walk through a project this way and try to interpret the changes, and maybe try to write an improved version in another CLion/VSCode instance at the same time. 

## Usage

Clone a git repository to your disk, then run inside:
```
tmtg
```

- tmtg will show you what position you are currently in (e.g. 1/200 commits).
- It will also ensure you have a clean working directory each time you press + or - (which will move you up or down the commits).
- It will show the current commit message as well as a list of changed files.


