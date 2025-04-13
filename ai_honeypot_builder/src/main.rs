#![allow(unused)]
use anyhow::Result;
use dirs::document_dir;
use markov::Chain;
use std::path::PathBuf;

struct Page {
    file_name: String,
    title: String,
    paragraphs: Vec<String>,
}

impl Page {
    pub fn new(harvard_chain: &Chain<String>) -> Page {
        let page = Page {
            file_name: "some-title".to_string(),
            title: "Some Title".to_string(),
            paragraphs: vec![],
        };
        page
    }
}

fn main() -> Result<()> {
    let base_dir = document_dir().unwrap().join("ai-honeypots");
    build_site(&base_dir, 1)?;
    // println!("{}", harvard_chain.generate_str());
    Ok(())
}

fn build_site(base_dir: &PathBuf, id: usize) -> Result<()> {
    let output_root = base_dir.join(format!("{}", id));
    mkdir_p(&output_root)?;
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
