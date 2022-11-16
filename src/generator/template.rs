use tera::{ Tera, Context, Result };

pub struct Templates {
    tera: Tera,
}

impl Templates {
    pub fn new() -> Templates {
        Templates {
            tera: Tera::default(),
        }
    }

    pub fn load_templates(&mut self, path: &str) -> Result<()> {
        self.tera = Tera::new(path)?;
        Ok(())
    }

    pub fn render(&self, name: &str, context: &Context) -> Result<String> {
        self.tera.render(name, context)
    }
}