// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

// Code from: https://github.com/robinst/linkify/blob/main/demo/src/lib.rs

use linkify::{LinkFinder, LinkKind};

pub fn linkify(text: &str) -> String {
    let mut link_finder = LinkFinder::new();

    link_finder.url_must_have_scheme(true);

    let mut bytes = Vec::new();

    for span in link_finder.spans(text) {
        match span.kind() {
            Some(LinkKind::Url) => {
                let mut url = span.as_str().to_string();

                if !url.contains(":") {
                    url.insert_str(0, "https://");
                }

                bytes.extend_from_slice(b"<a href=\"");
                escape(&url, &mut bytes);

                bytes.extend_from_slice(b"\">");
                escape(span.as_str(), &mut bytes);

                bytes.extend_from_slice(b"</a>");
            }
            Some(LinkKind::Email) => {
                bytes.extend_from_slice(b"<a href=\"mailto:");
                escape(span.as_str(), &mut bytes);

                bytes.extend_from_slice(b"\">");
                escape(span.as_str(), &mut bytes);

                bytes.extend_from_slice(b"</a>");
            }
            _ => {
                escape(span.as_str(), &mut bytes);
            }
        }
    }
    String::from_utf8(bytes).expect("Added bytes are all ASCII")
}

fn escape(text: &str, output: &mut Vec<u8>) {
    for character in text.bytes() {
        match character {
            b'&' => output.extend_from_slice(b"&amp;"),
            b'<' => output.extend_from_slice(b"&lt;"),
            b'>' => output.extend_from_slice(b"&gt;"),
            b'"' => output.extend_from_slice(b"&quot;"),
            b'\'' => output.extend_from_slice(b"&#39;"),
            _ => output.push(character),
        }
    }
}
