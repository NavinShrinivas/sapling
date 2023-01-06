# Templating 

`sapling` uses [Tera](https://tera.netlify.app/) as it's templating engine. For a much more detailed look into templating, do look into Tera's [ documentation ](https://tera.netlify.app/docs/). This section of `sapling`'s docs only contains some important parts.

## Overriding Blocks 

`sapling` supports Inheritance. That is children templates can extend parent templates and override sections(Or addon to parent content). These sections are identified by `blocks`. The below sections are copied over from Tera docs as they paint a very clear picture : 
### Base template
A base template typically contains the basic document structure as well as several blocks that can have content.

For example, here's a base.html almost copied from the Jinja2 documentation:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    {% block head %}
    <link rel="stylesheet" href="style.css" />
    <title>{% block title %}{% endblock title %} - My Webpage</title>
    {% endblock head %}
</head>
<body>
    <div id="content">{% block content %}{% endblock content %}</div>
    <div id="footer">
        {% block footer %}
        &copy; Copyright 2008 by <a href="http://domain.invalid/">you</a>.
        {% endblock footer %}
    </div>
</body>
</html>
```
This base.html template defines 4 block tags that child templates can override. The head and footer block have some content already which will be rendered if they are not overridden.

### Child template

Again, straight from Jinja2 docs:
```html
{% extends "base.html" %}
{% block title %}Index{% endblock title %}
{% block head %}
    {{ super() }}
    <style type="text/css">
        .important { color: #336699; }
    </style>
{% endblock head %}
{% block content %}
    <h1>Index</h1>
    <p class="important">
      Welcome to my awesome homepage.
    </p>
{% endblock content %}
```

To indicate inheritance, you have to use the extends tag as the first thing in the file followed by the name of the template you want to extend. The {{ super() }} variable call tells Tera to render the parent block there.

Nested blocks also work in Tera. Consider the following templates:

```html
// grandparent
{% block hey %}hello{% endblock hey %}

// parent
{% extends "grandparent" %}
{% block hey %}hi and grandma says {{ super() }} {% block ending %}sincerely{% endblock ending %}{% endblock hey %}

// child
{% extends "parent" %}
{% block hey %}dad says {{ super() }}{% endblock hey %}
{% block ending %}{{ super() }} with love{% endblock ending %}
```
The block `ending` is nested in the `hey` block. Rendering the `child` template will do the following:

- Find the first base template: `grandparent`
- See `hey` block in it and check if it is in `child` and parent template
- It is in `child` so we render it, it contains a `super()` call so we render the hey block from `parent`, which also contains a `super()` so we render the `hey` block of the `grandparent` template as well
- See `ending` block in `child`, render it and also render the ending block of `parent` as there is a `super()`

The end result of that rendering (not counting whitespace) will be: "dad says hi and grandma says hello sincerely with love".

## Object types and Array type variables frontmatter
Say our frontmatter looks something like this : 
```
---
title : 
   main : hello
   alternate : hello_world 
authors : [Navin, Sapling]
---
```
You would access these variables in templates like so : 
```html
{% block head %}
<title>{{ frontmatter.title.alternate }}</title>
{% endblock head %}
<div>
   <h3 class="fufu"> Authors : </h3> 
   <ul> 
      {% for author in frontmatter.authors %}
      <li> {{ author }} </li>
      {% endfor%}
   </ul>
</div>
```
