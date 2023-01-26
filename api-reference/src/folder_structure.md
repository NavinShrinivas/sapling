# Folder structure 

This is one of those places where `sapling` is very rigid and opinionated. As of now, the folder names are rigid, in the upcoming releases this is bound to change.

## Basic structure

The basic structure of a project should be like this:
```
.
├── content
│   └── blog1.md
├── css
│   ├── css1.css
│   └── csssubdir
│       └── css2.css
└── template
    ├── home.html
    └── subfolder
        └── home2.html
```
In the upcoming releases we will be adding a bootstrapper which will help you init your project.[THIS IS DONE NOW] 

## Accessing template 
The folder that the runtime searches for when mentioning a template in frontmatter is by default `template` folder. So the way you would use say `home2.html` in the above project structure will be : 
```
---
template : subfolder/home2.html
---
```
> Help : If you don't understand frontmatter, look into [frontmatter](./frontmatter.md) to learn about it.
---
and for `home.html` :
```
---
template : home.html 
---
```
> Note : using relative or absolute paths may render the runtime unable to generate anything.

## Accessing css 
The css is served automatically by the server runtime. In the given example including `css2.css` in the template will be something like this : 
```html
<link rel="stylesheet" href="/css/csssubdir/css2.css" />
```
and accessing `css1.css` :
```html
<link rel="stylesheet" href="/css/css1.css" />
```
`sapling` uses [lightningcss](https://lightningcss.dev/) and supports bundling and a lot more. Look into the [ css ](./css.md) section to learn more about writing css files for `sapling`

## Bootstrapping 

To use the tool to create the above mentioned folder structure see `Bootstrapping` in [ Usage ](./Usage.md) section.

