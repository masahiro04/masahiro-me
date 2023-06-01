use html2text::from_read;
use scraper::{Html, Selector};

pub fn html_to_text_converter(html: &str) -> String {
    // NOTE: htmlを扱うたえにHTMLを生成
    let document = Html::parse_document(html);
    let body_selector = Selector::parse("body").unwrap();
    if let Some(body_element) = document.select(&body_selector).next() {
        let body_html = body_element.inner_html();
        let mut text = from_read(body_html.as_bytes(), 80);
        text.retain(|c| c != '\n');
        text
    } else {
        String::new()
    }
}
