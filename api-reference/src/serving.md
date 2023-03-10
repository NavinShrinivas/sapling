# Serving 
`sapling` uses rocket to serve the static files. What's different here is that you can control the url where a given markdown file gets served. This can be done using the `link` tag in the frontmatter. 

## Serving link handling using frontmatter 

> Note : If there is somehow 2 or more markdown files that are asking to be served on the same path, the second one is served. The warning `([WARN])` does show up during processing stage.

> Note : By default, the serving link for a given file is its file name, like : `/file_name/`

You would control the serving like so : 
```
---
link : "/blogs/blog2-hello-world" 
---
```
> Important note : the server that is being used can acccess the links only if it ends with a "/", do make sure of this.

## Reloading 

- `sapling` comes with a cool feature, live reload! 
- Any changes in `content`, `template` and `css` are auto detected and any browser tab that is open will be auto-reloaded! 
- Any additions of `assets` needs a manual restart, this will soon be changed!

## Other information 

- Rocket by default serves the sites on port `80`. This makes sudo permission must. Unless you want to serve on other ports : 
`sapling --serve-port 8000 run`

> It's better to always run sapling in sudo.
