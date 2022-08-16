use std::io;
fn main() {
    // Obtain user input as a website url to scrape
    println!("Please input a valid website here: ");
    let mut website_url = String::new();
    io::stdin()
        .read_line(&mut website_url)
        .expect("Not a valid web url");

    website_url = website_url.trim_start_matches("http://").to_string();

    if !website_url.starts_with("https://") {
        let https_base = "https://";
        website_url = format!("{}{}", https_base, website_url);
    }

    website_url = website_url.trim().to_string();

    println!("Web url:: {:?}", website_url);

    let response = reqwest::blocking::get(
        website_url,
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let mut body_string = String::new();

    // target received html "body" contents and assign to body_string as a string
    let body = scraper::Selector::parse("body").unwrap();
    let elements = document.select(&body).map(|x| x.inner_html());
    elements
        .zip(1..101)
        .for_each(|(item, _)| {
            body_string = format!("{}{}", body_string, item).to_string();
            ()
        });

    //let body_raw_string = r#body_string; works too!
    let body_raw_string = to_raw_string(&body_string);
    println!("{}", body_raw_string);

    show_tree();

}

fn to_raw_string(str_arg: &str) -> &str {
    r#str_arg
}

fn show_tree() {
    let sample_html_body = r#"
        <header>
          <div class="inner">
            ioa href="https://iodapson.github.io/">
              <h1>The Book Debrief</h1>
            </a>
            <h2>Rust drills, all based on 'The Book'. Check the Rust Official page to read 'The Book'</h2>
            <a href="https://github.com/iodapson" class="button"><small>Follow me</small>Github</a>
          </div>
        </header>
    "#;
    
    let elements: Vec<_> = sample_html_body.split(['<', '>'])
        .filter(|item| item.trim() != "")
        .collect();
    println!("{:?}", elements);

    for element in elements {
        format!("<{}>", element);
    }
    println!("");
}

// FEATURES::
/*
    - Scrape machine-local webpages
    - Print out DOM tree in a glance
    - Keep std input open listening for input
    - Add syntax highlighting of elements
    - Allow querying of DOM for elements; its classes and even content
      -- Add tests
      -- Integrate modules and lib.rs
    - Convert to crate or installable Cargo binary
*/