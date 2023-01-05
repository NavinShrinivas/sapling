# Sapling | A static site generator and server

```
░██████╗░█████╗░██████╗░██╗░░░░░██╗███╗░░██╗░██████╗░
██╔════╝██╔══██╗██╔══██╗██║░░░░░██║████╗░██║██╔════╝░
╚█████╗░███████║██████╔╝██║░░░░░██║██╔██╗██║██║░░██╗░
░╚═══██╗██╔══██║██╔═══╝░██║░░░░░██║██║╚████║██║░░╚██╗
██████╔╝██║░░██║██║░░░░░███████╗██║██║░╚███║╚██████╔╝
╚═════╝░╚═╝░░╚═╝╚═╝░░░░░╚══════╝╚═╝╚═╝░░╚══╝░╚═════╝░

Fast light weight static site framework written in rust
```

If you are familiar with a static site generator like 11ty, `sapling` is nothing new to you. If not here is a quick intro to the world of static site frameworks. 

## Static Site Framework :

Often times, website do not have to change once served. Websites like these are considered "static". Even to write those static site we often want to do little dynamic stuff, like : 
- get a page with list of all sites in a given directory or topic
- use same structure for all blogs but different content

or some times we want to do things like : 
- want to write slightly complex websites without javascript or much html 

Static site frameworks serve exactly these purposes. 

## What does sapling do?

Sapling is in it's very early stages at the moment. But current sapling can :
- render html from markdown files using template
- also handles css 
- supports entire of gfm syntax 

Some of the upcoming features in sampling are : 
- deep data merge 
- project management system
