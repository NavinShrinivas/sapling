## Architechture

### Bootstrap mode
[TODO]

### Run mode

The rendering process happens in many phases. 
- First, we use Tera's in-built functions to parse all the templates.
- Discovery stage: 
   - Reads all frontmatter and content and stores it as a `Discovered` struct, which in turn stores a Hashmap of `ContentDocument` as value and path as key.
   - ContentDocument is got from the markdownParser, it passes all the file reads into the markdownParser. This uses `comrak` to parse all markdown generate html and also handle frontmatter, It simply returns a ContentDocument.
   - In discovery we check for forwardindex and reverseindex tags.
   - The forwardindex data finally collected is filled inot every ContentDocument in the Discovered struct.
   - The reverseindex data is return back to main function.
   - This is because forwardindex is available to every post. reverseindex is only used to render the needed pages.
- We then start rendering markdown files, this is as simple as iterating on the Discovered hashmap and using Tera, we carefully feed the ContentDocument to every post respectively.
- After this, we start rendering the pages for Reverseindex.
- After this, we use Rocket to serve the sites.

> Note : The above is a very high level overview, the code is def not 1 on 1 to what I've mentioned above.

This document and code can be much better understood by also reading the api-reference.

## Testing

- The respective feature/change must be introduced in the bootstrap_project and do `cargo run -- --serve --serve-port 8000 run` to see if everything works!
