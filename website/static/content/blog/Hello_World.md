# Hello, World!

I have been thinking about creating a website for many years and finally found the a moment to create it.

The final drop of motivation came through a discussion with a web developer, whom I told about my work with the [Rust programming language](https://www.rust-lang.org/), and that it can be used to write front-end applications as well.
I would've liked to explain to him much more on how to do so but I had never done it, so that conversation came to an end quickly.

That was going to change the very next day, so here we go!

## What will be on this website?
I'm not sure about the content I want to publish here, as my initial focus is just to play around with the technology.

## First version
The implementation will likely change over time and the following explanation might become outdated fast, nonetheless, let's see.

### 2020-02-08
First version with the goal of being served statically via GitHub pages.

The client bundle is built using [yew](https://github.com/yewstack/yew), making use of [GitHub Actions](https://travis-ci.org) for building and deployment.

To display the content, the client downloads a known index file which is regenerated on CI as well.
This index file contains metadata about the content to be displayed.

The first media-type to be supported is Markdown.
