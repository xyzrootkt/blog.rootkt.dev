use pulldown_cmark::{Event, Options, Parser, html};

pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        Event::Html(value) | Event::InlineHtml(value) => Event::Text(value),
        other => other,
    });

    let mut rendered = String::new();
    html::push_html(&mut rendered, parser);

    rendered
}
