use derive_new::new;

#[derive(Debug, Copy, Clone)]
pub enum Nesting {
    Exact(isize),
    Configured(isize),
}

#[derive(Debug, Copy, Clone, new)]
pub struct RenderConfig {
    pub indent_size: isize,
    pub column_size: usize,
}

impl RenderConfig {
    pub fn width(page_size: usize) -> RenderConfig {
        RenderConfig {
            indent_size: 2,
            column_size: page_size,
        }
    }

    pub fn indent_size(mut self, size: isize) -> RenderConfig {
        self.indent_size = size;
        self
    }

    pub fn size(&self, nesting: Nesting) -> isize {
        match nesting {
            Nesting::Exact(size) => size,
            Nesting::Configured(size) => size * self.indent_size,
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            indent_size: 2,
            column_size: 80,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RenderState {
    nesting: isize,
    config: RenderConfig,
}

impl Default for RenderState {
    fn default() -> Self {
        RenderState {
            nesting: 0,
            config: RenderConfig::default(),
        }
    }
}

impl RenderState {
    pub fn top(config: RenderConfig) -> RenderState {
        RenderState { nesting: 0, config }
    }

    pub fn indentation(&self) -> isize {
        self.nesting * self.config.indent_size
    }

    pub fn size(&self, nesting: Nesting) -> isize {
        self.config.size(nesting)
    }

    pub fn indent(&self, indent: Nesting) -> RenderState {
        RenderState {
            config: self.config,
            nesting: self.nesting + self.config.size(indent),
        }
    }

    pub fn nest(&self) -> RenderState {
        RenderState {
            config: self.config,
            nesting: self.nesting + 1,
        }
    }
}
