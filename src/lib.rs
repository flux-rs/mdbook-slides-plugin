use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;

const PREFIX: &str = r#"<div class="slides">"#;
const SUFFIX: &str = r#"</div>"#;

pub struct SlidesPreprocessor;

impl SlidesPreprocessor {
    pub fn new() -> Self {
        SlidesPreprocessor
    }
}

fn push_block(wrapped_chunks: &mut Vec<String>, cur: &Vec<&str>) {
    let block_str = cur.join("\n");
    let block_str = block_str.trim();
    if block_str != "" {
        let wrapped = [PREFIX, "", &block_str, "", SUFFIX, ""].join("\n");
        wrapped_chunks.push(wrapped);
    }
}

fn wrap(input: &String) -> String {
    let mut wrapped_chunks = vec![];
    let mut cur = vec![];
    for l in input.lines() {
        if l.starts_with("#") {
            push_block(&mut wrapped_chunks, &cur);
            cur = vec![l];
        } else {
            cur.push(l);
        }
    }
    push_block(&mut wrapped_chunks, &cur);
    wrapped_chunks.join("\n")
}

impl Preprocessor for SlidesPreprocessor {
    fn name(&self) -> &str {
        "slides"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        let use_plugin = std::env::var("MDBOOK_SLIDES_PLUGIN").unwrap_or_default() == "true";
        if use_plugin {
            book.for_each_mut(|book_item| {
                if let BookItem::Chapter(ch) = book_item {
                    ch.content = wrap(&ch.content);
                }
            });
        }
        return Ok(book);
    }
}

#[test]
fn test_wrap() {
    let input = r#"

piglets

and

dogs

# Slide 1
bing
bong

## Slide 1.1
bigly
poggers

# Slide 2
frinkglea

## Slide 2.1
zig zag
zinker

# Slide 3
zong

"#;
    let input = String::from(input);
    println!("{}", wrap(&input));
}
