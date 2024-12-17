use crate::markdown::types::{MarkdownDocument, MarkdownElement, InlineText};

pub fn parse_markdown(input: &str) -> MarkdownDocument {
    let mut elements = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    
    for line in lines {
        let trimmed = line.trim();
        
        // Parse headings
        if let Some(heading) = parse_heading(trimmed) {
            elements.push(heading);
            continue;
        }
        
        // Parse bullet lists
        if let Some(bullet_list) = parse_bullet_list(trimmed) {
            elements.push(bullet_list);
            continue;
        }
        
        // Parse wiki-style links
        if let Some(link) = parse_wiki_link(trimmed) {
            elements.push(link);
            continue;
        }
        
        // Anything else is inline text
        if !trimmed.is_empty() {
            elements.push(MarkdownElement::InlineText(InlineText { 
                content: trimmed.to_string(),
                links: Vec::new()
            }));
        }
    }
    
    MarkdownDocument { elements }
}

fn parse_heading(line: &str) -> Option<MarkdownElement> {
    let heading_chars: Vec<char> = line.chars().take_while(|&c| c == '#').collect();
    let level = heading_chars.len();
    
    if level > 0 {
        let title_text = line[level..].trim().to_string();
        let links = parse_wiki_links(line);
        
        Some(MarkdownElement::Heading { 
            level: level as u8, 
            title: InlineText {
                content: title_text,
                links
            },
            content: Vec::new()
        })
    } else {
        None
    }
}

fn parse_bullet_list(line: &str) -> Option<MarkdownElement> {
    if line.starts_with('-') {
        let content = line[1..].trim().to_string();
        let links = parse_wiki_links(line);
        
        Some(MarkdownElement::BulletList { 
            items: vec![InlineText {
                content,
                links
            }]
        })
    } else {
        None
    }
}

fn parse_wiki_links(line: &str) -> Vec<String> {
    let link_pattern = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    let mut links = Vec::new();

    for capture in link_pattern.captures_iter(line) {
        let link_text = capture.get(1).map_or("", |m| m.as_str()).trim().to_string();
        links.push(link_text);
    }

    links
}

fn parse_wiki_link(line: &str) -> Option<MarkdownElement> {
    let link_pattern = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    let mut links = Vec::new();

    for capture in link_pattern.captures_iter(line) {
        let link_text = capture.get(1).map_or("", |m| m.as_str()).trim().to_string();
        links.push(link_text.clone());
    }

    if !links.is_empty() {
        Some(MarkdownElement::InlineText(InlineText { 
            content: line.to_string(), 
            links 
        }))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let input = "# Heading 1\n- List item\n[[Link]] with text\nText with [[Another Link]]";
        let doc = parse_markdown(input);
        
        assert_eq!(doc.elements.len(), 4);
        assert_eq!(doc.elements[0], MarkdownElement::Heading { 
            level: 1, 
            title: InlineText {
                content: "Heading 1".to_string(),
                links: Vec::new()
            },
            content: Vec::new()
        });
        assert_eq!(doc.elements[1], MarkdownElement::BulletList { 
            items: vec![InlineText {
                content: "List item".to_string(),
                links: Vec::new()
            }]
        });
        assert_eq!(doc.elements[2], MarkdownElement::InlineText(InlineText { 
            content: "[[Link]] with text".to_string(), 
            links: vec!["Link".to_string()] 
        }));
        assert_eq!(doc.elements[3], MarkdownElement::InlineText(InlineText { 
            content: "Text with [[Another Link]]".to_string(), 
            links: vec!["Another Link".to_string()] 
        }));
    }

    #[test]
    fn test_parse_markdown_with_links() {
        let input = "# Heading with [[Link1]] and [[Link2]]\n- List item with [[Link3]]\nText with [[Link4]]";
        let doc = parse_markdown(input);
        
        assert_eq!(doc.elements.len(), 3);
        assert_eq!(doc.elements[0], MarkdownElement::Heading { 
            level: 1, 
            title: InlineText {
                content: "Heading with [[Link1]] and [[Link2]]".to_string(),
                links: vec!["Link1".to_string(), "Link2".to_string()]
            },
            content: Vec::new()
        });
        assert_eq!(doc.elements[1], MarkdownElement::BulletList { 
            items: vec![InlineText {
                content: "List item with [[Link3]]".to_string(),
                links: vec!["Link3".to_string()]
            }]
        });
        assert_eq!(doc.elements[2], MarkdownElement::InlineText(InlineText { 
            content: "Text with [[Link4]]".to_string(), 
            links: vec!["Link4".to_string()] 
        }));
    }

    #[test]
    fn test_multi_level_headings() {
        let input = "# Top Level Heading\n## Second Level Heading\n### Third Level Heading with [[Link]]";
        let doc = parse_markdown(input);
        
        assert_eq!(doc.elements.len(), 3);
        assert_eq!(doc.elements[0], MarkdownElement::Heading { 
            level: 1, 
            title: InlineText {
                content: "Top Level Heading".to_string(),
                links: Vec::new()
            },
            content: Vec::new()
        });
        assert_eq!(doc.elements[1], MarkdownElement::Heading { 
            level: 2, 
            title: InlineText {
                content: "Second Level Heading".to_string(),
                links: Vec::new()
            },
            content: Vec::new()
        });
        assert_eq!(doc.elements[2], MarkdownElement::Heading { 
            level: 3, 
            title: InlineText {
                content: "Third Level Heading with [[Link]]".to_string(),
                links: vec!["Link".to_string()]
            },
            content: Vec::new()
        });
    }
}
