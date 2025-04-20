# RSS - Really Simple Syndication 

Is a standardised way that websites can provide their content. Given that a large number of sites provide RSS feeds, a single app should be enough for users to read content from multiple sources as long as each of the source provides a RSS feed. 

> To know how to enable RSS in your sapling site, check the [Settings](/settings.html) page.

Once RSS is enabled in settings, sapling will generate one feed for each of the defined groups in the settings (as long as there is at least one markdown content file tagged to that group). 

## Tagging content to RSS groups :
A single markdown content file can be tagged to multiple groups. To do so, you can add the following to the frontmatter of your markdown content file, your frontmattrer will look something like this (look at the `rss_group` key) : 
```yaml
---
title : "Second blog testing Markdown elements"
date : "01-20-2002"
author : "P K Navin Shrinivas"
template : "blog.html"
forwardindex : ["blog","tags"]
tags : ["test","deep data merge"]
rss_group: ["all","completed"]
---
```

For tagging it to a single group, you can do so like this : 
```yaml
---
title : "Second blog testing Markdown elements"
date : "01-20-2002"
author : "P K Navin Shrinivas"
template : "blog.html"
forwardindex : ["blog","tags"]
tags : ["test","deep data merge"]
rss_group: "all"
---
```
