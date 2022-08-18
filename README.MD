SAMPLE scraped HTML inside <body>

<header>
    <div class="inner">
        <a href="https://iodapson.github.io/">
            <h1>The Book Debrief</h1>
        </a>
        <h2>Rust drills, all based on 'The Book'. Check the Rust Official page to read 'The Book'</h2>
        <a href="https://github.com/iodapson" class="button"><small>Follow me</small>Github</a>
    </div>
</header>
<div id="content-wrapper">
    <div class="inner clearfix">
        <section id="main-content">
            <h2 id="rust-drills">RUST DRILLS</h2>
            ...
        </section>
    </div>
</div>

FEATURES (Vision of what the app should be able to do)

TREES - Sample HTML above should be formatted into a tree that looks as follows:

-header
  |
    -div
      |
        -a
          |
            -h1
        -h2
-div
  |
    -div
      |
        -section
          |
            -h2


QUERY - App should listen for input, or maybe, allow command-line argument keywords like "show-class-and-id-count", which shpuld then print out:

-header (0 class, 0 id)
  |
    -div (1 class, 0 id)
      |
        -a (0 class, 0 id)
          |
            -h1 (0 class, 0 id)
        h2 (0 class, 0 id)
-div (0 class, 1 id)
  |
    -div (1 class, 0 id)
      |
        -section (1 class, 0 id)
          |
            -h2


QUERY - Here is another query, this time for the content of each HTML element, using command-line argument "content":

-header (div)
  |
    -div (a)
      |
        -a (h1)
          |
            -h1 (The Book Debrief)
        h2 (Rust drills, all based on 'The Book'. Check the Rust Official page to read 'The Book')
-div (div)
  |
    -div (div)
      |
        -section (h2, ...)
          |
            -h2 (RUST DRILLS)


QUERY - Here is yet another query. This one has command-line argument "with-indices" (a better arg name can come up later). Output could look as follows:

-header (0)
  |
    -div (0-0)
      |
        -a (0-0-0)
          |
            -h1 (0-0-0-0)
        h2 (0-0-1)
-div (1)
  |
    -div (1-0)
      |
        -section (1-0-0)
          |
            -h2 (1-0-0-0)


N.B - I hope I have been able to successfully hint at the direction which I am heading with this app. Think Rust-analyzer for HTML. It is my hope that these four starting features can be implemented and this project be made ready to be cargo installed or something similar so people can use it in their web-dev process. More queries can then be supported much later after this project has proved itself useful for public useage.

Thanks for reading.