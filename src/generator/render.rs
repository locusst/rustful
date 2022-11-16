use super::template::{ Templates };
use super::{ Site, Page };
use comrak::{ markdown_to_html, ComrakOptions };

struct FrontMatter {
    title: String,
    author: String,
    date: String,
}

impl FrontMatter {
    fn new() -> FrontMatter {
        FrontMatter {
            title: String::new(),
            author: String::new(),
            date: String::new(),
        }
    }
}

pub fn parse_post(raw: &str) -> Page {
    let mut page = Page::new();
    let mut lines = raw.lines();
    let front_matter = parse_front_matter(&mut lines);
    page.title = front_matter.title;
    page.author = front_matter.author;
    page.date = front_matter.date;
    page.content = lines.collect::<Vec<&str>>().join("\n");

    page
}

fn parse_front_matter(lines: &mut std::str::Lines) -> FrontMatter {
    let mut front_matter = FrontMatter::new();
    let mut in_front_matter = false;

    for line in lines {
        if line == "---" {
            if in_front_matter {
                break;
            } else {
                in_front_matter = true;
            }
        } else if in_front_matter && line != "---" && !line.is_empty() {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim();
            match key {
                "title" => {
                    front_matter.title = value.to_string();
                }
                "author" => {
                    front_matter.author = value.to_string();
                }
                "date" => {
                    front_matter.date = value.to_string();
                }
                _ => (),
            }
        } else if in_front_matter && line.is_empty() {
            continue;
        }
    }

    front_matter
}

impl Site {
    pub fn load_posts(&mut self, path: &str) {
        let paths = glob::glob(&format!("{}/posts/**/*.md", path)).unwrap();

        for path in paths {
            println!("Loading: {:?}", path);
            let path = path.unwrap();
            let markdown = std::fs::read_to_string(path).unwrap();
            let page = parse_post(&markdown);
            self.pages.push(page);
        }
    }

    pub fn render_markdown(&mut self) {
        for page in &mut self.pages {
            page.content = markdown_to_html(&page.content, &ComrakOptions::default());
        }
    }
}

impl Templates {
    pub fn render_index(&self, site: &Site) -> String {
        let mut context = tera::Context::new();
        context.insert("site", site);
        self.render("index.html", &context).unwrap()
    }

    pub fn render_site_pages(&self, site: &Site) -> Vec<Page> {
        let mut pages = Vec::new();
        let mut context = tera::Context::new();
        context.insert("site", site);
        for page in &site.pages {
            context.insert("page", page);
            let content = self.render("page.html", &context).unwrap();
            let mut new_page = page.clone();
            new_page.content = content;
            context.remove("page");
            pages.push(new_page);
        }
        pages
    }
}