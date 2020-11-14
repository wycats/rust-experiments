use derive_new::new;

use crate::{Primitive, StyledDoc};

#[derive(Debug, Copy, Clone)]
pub enum Nesting {
    Exact(isize),
    Configured(isize),
}

#[derive(Debug, Copy, Clone, new)]
pub struct RenderConfig {
    indent_size: isize,
}

impl RenderConfig {
    pub fn size(&self, nesting: Nesting) -> isize {
        match nesting {
            Nesting::Exact(size) => size,
            Nesting::Configured(size) => size * self.indent_size,
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig { indent_size: 2 }
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

pub trait Render: Sized {
    fn render(self) -> StyledDoc {
        self.render_with_state(&RenderState::default())
    }

    fn render_with_config(self, config: RenderConfig) -> StyledDoc {
        self.render_with_state(&RenderState::top(config))
    }

    fn render_with_state(self, state: &RenderState) -> StyledDoc {
        self.into_primitive(true).render_with_state(state)
    }

    fn into_primitive(self, recursive: bool) -> Primitive;
}
