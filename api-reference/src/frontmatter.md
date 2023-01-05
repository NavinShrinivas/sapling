# Frontmatter 
Frontmatter is found at the starting of the file and can contain any field you wish. 
> Note : the frontmatter is completely optional, but do look at the defaults before skipping out of frontmatter all together.

## Common fields and some defaults
Some common frontmatter fields are : 
```
---
title: can be used to store the title of the site 
template: specifies what template to render a given markdown file using
link: specifies where this given page must be served
name:  Is related with the `link` tag
---
```
And some default values for important fields : 
- template: default is `index.html` 
- link: default is the `name` tag in frontmatter 
- name: default is the files name 

An example frontmatter : 
```
---
template: "blogs_templates/blog.html"
title: "Blog1-OwO"
link: "/blogs/blog1-OwO"
data_merge : "blog"
authors : [navin,navin_clone]
---
```
## Accessing frontmatter variables
To access these variables in your templates you have to use `{{frontmatter.x}}`, something like this : 
```html
<html>
   <head>
      {% block head %}
      <title>{{ frontmatter.title.main }}</title>
      {% endblock head %}
   </head>
   ...
</html>

```
