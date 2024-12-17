#[derive(Debug, PartialEq)]
pub struct MarkdownDocument {
    pub elements: Vec<MarkdownElement>,
}

#[derive(Debug, PartialEq)]
pub struct InlineText {
    pub content: String,
    pub links: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum MarkdownElement {
    Heading {
        level: u8, // h1 = 1, h2 = 2, etc.
        title: InlineText,
        children: Vec<MarkdownElement>,
    },
    BulletList {
        items: Vec<InlineText>,
    },
    InlineText(InlineText),
}
