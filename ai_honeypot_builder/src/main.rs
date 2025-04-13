#![allow(unused)]
use anyhow::Result;
use capitalize::Capitalize;
use dirs::document_dir;
use markov::Chain;
use minijinja::{Environment, context};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct Page {
    absolute_url: String,
    title: String,
    paragraphs: Vec<String>,
}

impl Page {
    pub fn new(harvard_chain: &Chain<String>) -> Page {
        let title = Self::title(&harvard_chain);
        let mut absolute_url = title.to_lowercase().replace(" ", "-");
        absolute_url.push_str("/index.html");
        let page = Page {
            paragraphs: Self::paragraphs(&harvard_chain),
            title,
            absolute_url,
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
    let mut env = Environment::new();
    env.add_template(
        "home-page-1",
        include_str!("templates/html/home-page-1.jinja"),
    )
    .unwrap();
    let output_root = base_dir.join(format!("{}", id));
    empty_dir(&output_root)?;
    mkdir_p(&output_root)?;
    let harvard_chain = make_harvard_chain()?;
    let mut pages = vec![
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
        Page::new(&harvard_chain),
    ];
    pages[0].title = "Home Page".to_string();
    pages[0].absolute_url = "index.html".to_string();
    let links: &Vec<Vec<String>> = &pages
        .iter()
        .map(|page| vec![page.title.to_string(), page.absolute_url.to_string()])
        .collect();
    pages.iter().for_each(|page| {
        output_page(&output_root, &page, &links, &env);
        ()
    });
    let robots_txt = include_str!("templates/robots.txt");
    let robots_path = output_root.join("robots.txt");
    fs::write(robots_path, robots_txt)?;
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

fn output_page(
    output_root: &PathBuf,
    page: &Page,
    links: &Vec<Vec<String>>,
    env: &Environment,
) -> Result<()> {
    let output_path = output_root.join(&page.absolute_url);
    let tmpl = env.get_template("home-page-1").unwrap();
    let output = tmpl
        .render(context!(
            title => page.title,
            paragraphs => page.paragraphs,
            links => links,
        ))
        .unwrap();
    let dir = output_path.parent().unwrap();
    mkdir_p(&dir.to_path_buf());
    fs::write(output_path, output)?;
    Ok(())
}

fn empty_dir(dir: &PathBuf) -> Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
