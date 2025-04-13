#![allow(unused)]
use anyhow::Result;
use capitalize::Capitalize;
use dirs::document_dir;
use markov::Chain;
use std::path::PathBuf;

#[derive(Debug)]
struct Page {
    url_key: String,
    title: String,
    paragraphs: Vec<String>,
}

impl Page {
    pub fn new(harvard_chain: &Chain<String>) -> Page {
        let title = Self::title(&harvard_chain);
        let url_key = title.to_lowercase().replace(" ", "-");
        let page = Page {
            url_key,
            title,
            paragraphs: Self::paragraphs(&harvard_chain),
        };
        page
    }

    fn title(harvard_chain: &Chain<String>) -> String {
        let base_string = harvard_chain.generate_str().replace(".", "");
        let mut title_words: Vec<&str> = base_string.split_whitespace().collect();
        title_words.truncate(5);
        let title: String = title_words
            .iter()
            .map(|word| word.capitalize())
            .collect::<Vec<String>>()
            .join(" ");
        title
    }

    fn paragraphs(harvard_chain: &Chain<String>) -> Vec<String> {
        vec![
            Self::paragraph(&harvard_chain),
            Self::paragraph(&harvard_chain),
            Self::paragraph(&harvard_chain),
            Self::paragraph(&harvard_chain),
        ]
    }

    fn paragraph(harvard_chain: &Chain<String>) -> String {
        vec![
            Self::sentence(&harvard_chain),
            Self::sentence(&harvard_chain),
            Self::sentence(&harvard_chain),
            Self::sentence(&harvard_chain),
        ]
        .join(" ")
    }

    fn sentence(harvard_chain: &Chain<String>) -> String {
        harvard_chain.generate_str()
    }
}

fn main() -> Result<()> {
    let base_dir = document_dir().unwrap().join("ai-honeypots");
    build_site(&base_dir, 1)?;
    Ok(())
}

fn build_site(base_dir: &PathBuf, id: usize) -> Result<()> {
    let output_root = base_dir.join(format!("{}", id));
    mkdir_p(&output_root)?;
    let harvard_chain = make_harvard_chain()?;
    let p = Page::new(&harvard_chain);
    dbg!(p);
    Ok(())
}

fn make_harvard_chain() -> Result<markov::Chain<String>> {
    let harvard_sentences = include_str!("corpuses/harvard-sentences.txt");
    let harvard_lines: Vec<&str> = harvard_sentences.lines().collect();
    let mut harvard_chain = Chain::new();
    for line in harvard_lines {
        harvard_chain.feed_str(line);
    }
    Ok(harvard_chain)
}

fn mkdir_p(dir: &PathBuf) -> Result<()> {
    if dir.exists() {
        Ok(())
    } else {
        std::fs::create_dir_all(dir)?;
        Ok(())
    }
}
