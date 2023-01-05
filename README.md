## Specification 

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
