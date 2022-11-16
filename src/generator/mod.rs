pub mod render;
pub mod template;

use serde::{Deserialize, Serialize};

use self::template::Templates;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub title: String,
    pub description: String,
    pub pages: Vec<Page>,
}

impl Site {
    pub fn new() -> Site {
        Site {
            title: String::new(),
            description: String::new(),
            pages: Vec::new(),
        }
    }

    pub fn load_config(&mut self, config: &str) {
        let config: Config = toml::from_str(config).unwrap();
        self.title = config.title;
        self.description = config.description;
    }

    pub fn generate(&mut self, source: &str, output: &str) {
        let config = std::fs::read_to_string(format!("{}/config.toml", source)).unwrap();
        let template_path = format!("{}/templates/**/*", source);
        self.load_config(&config);
        self.load_posts(source);
        self.render_markdown();
        let mut templates = Templates::new();
        templates.load_templates(&template_path).unwrap();

        if std::path::Path::new(output).exists() {
            std::fs::remove_dir_all(output).unwrap();
        }
        std::fs::create_dir_all(format!("{}/assets", output)).unwrap();

        let assets = glob::glob(&format!("{}/assets/**/*", source)).unwrap();
        for path in assets {
            let path = path.unwrap();
            let path_str = path.to_str().unwrap();
            let path_str =
                path_str.replace(&format!("{}/assets", source), &format!("{}/assets", output));
            if path.is_dir() {
                std::fs::create_dir_all(path_str).unwrap();
            } else {
                std::fs::copy(path, path_str).unwrap();
            }
        }

        let index = templates.render_index(&self);
        std::fs::write(format!("{}/index.html", output), index).unwrap();

        self.pages = templates.render_site_pages(self);
        for page in &self.pages {
            std::fs::write(format!("{}/{}.html", output, page.title), &page.content).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub title: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

impl Page {
    pub fn new() -> Page {
        Page {
            title: String::new(),
            date: String::new(),
            author: String::new(),
            content: String::new(),
        }
    }
}
