# Usage 

## Boostrapping 

The `sapling` binary can create the project structure and a new simple project. It does not serve the site by default. To bootstrap : 
```bash
sapling bootstrap project_name
```
> If a given folder with same name as project exists, sapling will initialise the project within that folder (Only if it empty). Else it create a new one. You can use the current directory by giving project_name as `.`

## Running 

`sapling` serve the project in the current folder, it cannot serve projects in other folders. To serve projects : 
```bash
sapling --serve run # Not mentioning --serve only builds your project
```
> Note : there are a lot of other options and optional command line arugment, so `sapling help` to see them.
