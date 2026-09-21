### qf
`qf` is an opinionated portfolio static stie generator that compiles custom markdown-like syntax into different document types. Export options currently include HTML and TXT, with more to come.

#### Motivation
A portfolio is meant to be a demonstration of your skills and capabilities. In my opinion, this means that you either use a portfolio to list out and showcase projects that you have worked on, experience you have gained, awards you have obtained, and so on; or you build the portfolio to itself showcase your skills. I am not a graphic designer nor better than anyone else at making websites - so I fall into the former category.

The problem I have is that most portfolios are AI-generated React or Next.JS slop-lag-fests. I am not a recruiter, but whenever I try interacting with these types of portfolios, I always cringe and subconsciously close the tab. If I were a recruiter, that wouldn't be a great sign. There is absolutely no reason that these sites require JavaScript. There is absolutely no reason to ship hundreds of megabytes of code just to list out your name, some projects, self-proclaimed skills, and absolutely random quotes that you definitely live by.

Thus, I wanted to build a system to make it easy to create static HTML + CSS portfolios with no bullshit. There is no framework, no fancy components, no animations, and no JavaScript. Sites load immediately, and don't stutter on the worstest of machines. Most importantly, it provides quick, easy, and distraction-less access to information that you wish to share.

#### Goals
The goals are quite simple: digest a markdown-like language, transform into HTML, TXT, and more, and deploy. Provide a limited set of primitives for page types and custom components, and require developers to bring their own CSS. Handle everything up until deployment, including linking documents together, indexing lists of documents, and ensuring maximum accessibility within the output formats.

#### Non-Goals
A portfolio is not a place to self-proclaim that you know 100% of JavaScript (high chance you don't) or Vite (what does this even mean?). There will be no custom components to create any of the following:
- Skill lists, trees, or whatever
- Quotes
- Interactive contact forms
- Advanced HTML layering or complex layouts
- Animations

If this tool doesn't have what you want, then use something else, it is not my problem.

### Features
`qf` supports exporting to accessible `.html` and plain text `.txt` files. It currently does both simultaneously and without configuration. Pages that are tagged with the same type are indexed together, allowing for pages to link to the previous and/or next entry - great for blog posts and other articles - and this happens automatically, updating all relevant pages when you create a new page.

Static files are copied from a configurable directory, default `static`, merged directly into the output `dist` directory. This is where you will include your custom `style.css` for styling HTML output, `favicon`, images, and more.

#### Planned Features
- Index Pages: dedicated pages that list out all pages available of a specific type. This way people can search all of your articles at once.
- Linked Page Content: copy content from specific pages, such as project pages, to reference in other pages. Simply create unique pages for each project you've worked on, and link content such as the name, description, image, and more, through an index or list on your root page.
- RSS XML Output: specifically for indexes, so people can subscribe through an RSS application to your articles, projects, and more.

### Language
The language is intentionally similar to markdown, however it _does not implement markdown_, and implements custom blocks to handle custom components and enforced accessibility.

~~~
@type page
@title Some Page Title
@summary This is an example page.
@date Sunday, September 20th

# This is a header

## This is a smaller header

1. This
2. is an
3. ordered list
4. that contains **strong** words

- This is an
- unordered list
- that contains *emphasis*

@image example.png
alt="An image that reads 'example'."
caption="An optional caption that described the image."

@image arrow_down.png
decorative

Built with [qf](https://github.com/waqfs/qf).

> A quote I definitely live by

```js
console.log("Hello, code block!");
```
~~~

This language format is used for all types of pages, regardless of export format. Accessibility properties are required, even if certain output formats never use them.

### Usage
While this is still in development, compile from source and point to your root directory:
```zsh
cargo run build --root /path/to/portfolio
```

### License
This project is licensed under GNU GPLv3, so you can use this for whatever, and forks must disclose source and be under the same license. Not that anybody actually gives a shit about code licensing.
