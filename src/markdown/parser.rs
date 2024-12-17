use crate::markdown::types::{MarkdownDocument, MarkdownElement};

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
        
        // Anything else is plain text
        if !trimmed.is_empty() {
            elements.push(MarkdownElement::PlainText { 
                content: trimmed.to_string() 
            });
        }
    }
    
    MarkdownDocument { elements }
}

fn parse_heading(line: &str) -> Option<MarkdownElement> {
    let heading_chars: Vec<char> = line.chars().take_while(|&c| c == '#').collect();
    let level = heading_chars.len();
    
    if level > 0 {
        let content = line[level..].trim().to_string();
        Some(MarkdownElement::Heading { level: level as u8, content })
    } else {
        None
    }
}

fn parse_bullet_list(line: &str) -> Option<MarkdownElement> {
    if line.starts_with('-') {
        let content = line[1..].trim().to_string();
        Some(MarkdownElement::BulletList { 
            items: vec![content] 
        })
    } else {
        None
    }
}

fn parse_wiki_link(line: &str) -> Option<MarkdownElement> {
    if line.starts_with("[[") && line.ends_with("]]") {
        let content = line[2..line.len()-2].trim().to_string();
        Some(MarkdownElement::Link { text: content })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let input = "# Heading 1\n- List item\n[[Link]]";
        let doc = parse_markdown(input);
        
        assert_eq!(doc.elements.len(), 3);
        assert_eq!(doc.elements[0], MarkdownElement::Heading { 
            level: 1, 
            content: "Heading 1".to_string() 
        });
        assert_eq!(doc.elements[1], MarkdownElement::BulletList { 
            items: vec!["List item".to_string()] 
        });
        assert_eq!(doc.elements[2], MarkdownElement::Link { 
            text: "Link".to_string() 
        });
    }
}
