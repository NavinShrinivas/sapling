# Settings 

Sapling has a bunch of defaults it usually runs in, obviously these defaults are considered `sane` for majority of the people making a need for intentional settings file `optional`. In this file, we see what all settings options sapling has and what they do. 

> `settings.yaml` is the file where all settings go, it is located in the root of your project, obviously, this is completely optional. On not finding settings.yaml, sapling throws an error, but continues to operate with defaults.

### Logging 

Log level can be defined inside settings file, like so : 

```yaml
logging: 
    level : "DEBUG"
```
> Note : The default log level is `INFO`. 

Possible `level`s are :
- `TRACE` : All logs are shown
- `DEBUG` : Debug logs are shown
- `INFO` : Info logs are shown
- `WARN` : Warning logs are shown
- `ERROR` : Error logs are shown

### RSS 

Sapling has built in support to generate RSS feeds for your sites. It's structure agnostic, meaning, it doesn't care where your markdown content are present. To enable said RSS, do so in settings : 
```yaml
rss:
  enable: true
  date_format: '%m-%d-%Y' #Optional - default is %m-%d-Y | This is used to parse the dates found in frontmatter of each markdown
  rss_groups: 
    - name: "All Posts" # Compulsory - this is the name of the group, used to identify the group
      group_id: "all" # Compulsory - this is the id of the group, used to identify the group
      link: "rss/all.xml" # Compulsory - this is the path where the rss will be generated
      content_url: "https://sapling.navinshrinivas.com/" # Compulsory - this is the url of the content, used to generate the rss
      description: "All completed and incomplete posts" #Optional
    - name: "Completed Posts"
      group_id: "completed"
      link: "rss/completed.xml"
      description: "All completed posts"
      content_url: "https://sapling.navinshrinivas.com/"
```
You should find the RSS feeds in the following links for the above example :
- /rss/all.xml 
- /rss/completed.xml

To see how to include markdown `content` files into RSS, check [RSS](/RSS.html) page.

### Serving 
<!-- sapling tries to be out of your way as much as possible, sapling can be used for solely generating your static sites. But if you do please, you can have sapling serve these state files as well. This can be done in two ways : -->
<!-- - By providing some command line arguments [See [here](serving.md) for command line args] -->
<!-- - Defining some settings in the `settings.yaml` file -->
<!-- > In the case where both are defined, command line arguments take precedence over the settings file. -->
<!---->
<!-- To define serving in the settings file, you can do so like this :  -->
<!-- ```yaml -->
<!-- serve:  -->
<!--     enable: true # Can be false -->
<!--     port : 8000 # Has to be valid port, else errors out -->
<!--     live-reload: true # Can be false -->
<!-- ``` -->
[TODO: Feature Under Construction]

### Fully featured example 

Here you can find a fully featured example of a `settings.yaml` file, the same in used in the bootstrap starter site as well : 

```yaml
rss:
  enable: true
  rss_groups: 
    - name: "All Posts"
      url: "all.xml"
      description: "All completed and incomplete posts"
    - name: "Completed Posts"
      url: "completed.xml"
      description: "All completed posts"
logging: 
  level : "TRACE"
```
