## Specification

### Project structure 

- This is one place that StaticType is highly Opinionated and expect a rigid structure.
- A folder strcture as such is expected : 

- [TODO] Allow changing default bases in folder structure

### Serving path : 
- The order of respecting serving paths are : 
   - `link` tag in frontmatter
   - file name of the markdown file in content [Note : You can have a name tag in your frontmatter]

### CSS files : 

- All css files need to be present in a "css" folder in the project root.
- refrencing these css files from templates is like so : 
```
<link rel="stylesheet" href="/css/index.css" />
```
- We minify and bundle css using [lightningcss](https://lightningcss.dev/), so anything it supports. StaticType also does!
> Note : The current way to handling css files will work only with the given rocket server or any other server that is designed to serve static files. Future releases may cover css injections.

### Fontmatter 
- Frontmatter is found at the starting of the file and can contain any field you wish [It's completely optional too!]. Some of the common fields are : 
- [TODO] To allow for changes to defaults in frontmatter
```
---
title : [Does not have a default value]
template : [default is `index.html`] specifies what template to render a given markdown file using
link : [default is the `name` tag in frontmatter] specifies where this given page must be served
name : [default is the files name] Is related with the `link` tag
---
```
