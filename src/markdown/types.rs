#[derive(Debug, PartialEq)]
pub struct MarkdownDocument {
    pub elements: Vec<MarkdownElement>
}

#[derive(Debug, PartialEq)]
pub enum MarkdownElement {
    Heading {
        level: u8,  // h1 = 1, h2 = 2, etc.
        content: String,
    },
    BulletList {
        items: Vec<String>,
    },
    Link {
        text: String,  // [[]]の中身
    },
    PlainText {
        content: String,
    },
}