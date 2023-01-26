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
   <a href="/tags/{{i}}">
      <button class="rounded btn bg-info b-info white">{{i}}</button>
   </a>
   {% endfor %}
</div>
```
> Note the `forwardindex.blog` that gives rise to an array of frontmatters.
