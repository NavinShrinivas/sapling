# Forward index 

Forward indices are easy to understand. The tag `frontmatter:<value>` groups the current frontmatter with "value" as key. Something like this : 

`content/blog1.md`:
```
---
title : "First blog testing Markdown elements"
date : "01-20-2002"
link : "/blogs/blog1"
author : ["P K Navin Shrinivas"]
template : "blog.html"
forwardindex : blog
tags : ["test","deep data merge"]
---
```
`content/tags.md`:
```
---
title : "Tags"
template : "tags.html"
---
```
> Note the `forwardindex:blog`. To access all frontmatters that were forward merged under `blog`. You do something like this in the template : 

`templates/tags.html`:
```html
<div class="flexdiv">
   {% for i in forwardindex.blog %} 
      {% for j in i.tags %}
         {% set_global flatlist = flatlist | concat(with=j) %}
      {% endfor %}
   {% endfor %}
   {% for i in flatlist|unique %}
   <a href="/tags/{{i}}/">
      <button class="rounded btn bg-info b-info white">{{i}}</button>
   </a>
   {% endfor %}
</div>
```
> Note the `forwardindex.blog` that gives rise to an array of frontmatters.

> Also do note the "/" in the end of the link in a href tag.

## Multiple forward index mapping 

Right from version 1, sapling supports mappinn a given frontmatter to multiple forward index keys! 

Your frontmatter will look something like this : 
```
title : "First blog testing Markdown elements"
date : "01-20-2002"
link : "/blogs/blog1"
author : ["P K Navin Shrinivas"]
template : "blog.html"
forwardindex : ["blog","tags"]
tags : ["test","deep data merge"]
```
And is now accessible through any of the following ways : 
```html
<div class="flexdiv">
   {% for i in forwardindex.blog %} 
      {% for j in i.tags %}
         {% set_global flatlist = flatlist | concat(with=j) %}
      {% endfor %}
   {% endfor %}
   {% for i in flatlist|unique %}
   <a href="/tags/{{i}}/">
      <button class="rounded btn bg-info b-info white">{{i}}</button>
   </a>
   {% endfor %}
</div>
```
or : 
```html
<div class="flexdiv">
   {% for i in forwardindex.tags %} <!-- note the difference here -->
      {% for j in i.tags %}
         {% set_global flatlist = flatlist | concat(with=j) %}
      {% endfor %}
   {% endfor %}
   {% for i in flatlist|unique %}
   <a href="/tags/{{i}}/">
      <button class="rounded btn bg-info b-info white">{{i}}</button>
   </a>
   {% endfor %}
</div>
```

> This gives raise to some unique and powerful data collections to be created a fed into the templating engine!
