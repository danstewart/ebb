# ebb

The easy blog builder.  

---

## What is it?

ebb is a small command line tool for managing a static blog using markdown and AWS S3.  

---

## Install

Just grab the right file from the releases section.  

--

## Usage

### Managing blog posts
```
# Start your blog
ebb init
# or
ebb init --name=my-blog --surround=wrapper.html --backend=s3 --format=md

# Add a new blog post
ebb new my-post.md

# Publish the post
ebb publish my-post.md

# Edit a post
ebb edit my-post.md
```

### Managing your blog style
```
# Edit your page surround
ebb 
```

---

## Features

- Easy to use
- Zero maintenance
- Extendable

---

## Extensions

ebb has been designed to extend pretty much every element.  
Want to use Digital Ocean spaces? Sure, just add it to `/src/backends/` and run `ebb config backend=dospaces`.  
What about using LaTeX instead of MD? Yepp, just add the logic to `/src/formatters/` and run `ebb config format=latex`.  
