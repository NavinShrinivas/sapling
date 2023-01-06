# CSS : Cascading Style Sheets 

`sapling` uses [lightningcss](https://lightningcss.dev/) and supports bundling and a lot more. 

## Imports in css : 

A given css files can import another css files using relative paths : 
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
In the above path, if `css2.css` wants to import `css1.css`, it would look something like this : 
```css
@import "../css1.css"

...
```
> Note : conflicting cases and selectors in bundled are not caught by css bundler just yet.
