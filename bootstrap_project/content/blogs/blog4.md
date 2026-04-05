---
title: "Completed Posts - RSS Feed Test"
date: "03-02-2026"
link: "/blogs/blog4"
author: ["Test Author"]
template: "blog.html"
forwardindex: ["blog", "tags"]
reverseindex: ["author", "tags"]
tags: ["rss", "completed"]
rss_group: 
    - "all"
    - "completed"
---

# Fourth Blog Post - Completed Status

This post is added to both the "all" and "completed" RSS groups to test RSS group filtering.

## RSS Group Testing

This post should appear in:
1. The "All Posts" RSS feed
2. The "Completed Posts" RSS feed

## Content Verification

Here's some content to ensure RSS parsing includes:

- Post title: "Completed Posts - RSS Feed Test"
- Publication date: 03-02-2026
- Multiple RSS groups assignment

This helps verify that posts with multiple `rss_group` entries are handled correctly in the RSS generation.
