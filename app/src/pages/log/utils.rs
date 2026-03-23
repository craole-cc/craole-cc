#[must_use]
pub fn render_markdown(md : &str,) -> String {
  use pulldown_cmark::{
    Options,
    Parser,
    html,
  };

  let mut opts = Options::empty();
  opts.insert(Options::ENABLE_TABLES,);
  opts.insert(Options::ENABLE_STRIKETHROUGH,);
  opts.insert(Options::ENABLE_TASKLISTS,);
  opts.insert(Options::ENABLE_HEADING_ATTRIBUTES,);

  let parser = Parser::new_ext(md, opts,);

  let mut out = String::new();
  html::push_html(&mut out, parser,);

  ammonia::Builder::default()
    .add_tags(["img", "figure", "figcaption"])
    .add_tag_attributes("img", ["src", "alt", "title", "loading", "decoding", "width", "height"])
    .add_tag_attributes("a", ["href", "title"])
    .url_schemes(["http", "https"].into())
    // Allow relative image paths like /images/foo.jpg to work:
    .url_relative(ammonia::UrlRelative::PassThrough)
    .clean(&out)
    .to_string()
}
