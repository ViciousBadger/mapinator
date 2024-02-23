# The Mapinator

It's a stupidly simple tool that converts valid Markdown files into markers on a
map. The map is just a HTML file you can view in your browser.

The markdown files have to look like this:

``` md
---
title: "The British Museum"
lat: 51.51925449251278
lng: -0.12706067024482415
---

# The British Museum

(Some relevant text about the British Museum (I know absolutely nothing about it))
```

By default the tool will read all .md files recursively from the current working
directory and collect them in the file map.html. Files that do not contain a
valid front-matter (title, lat, lng) will be ignored.
