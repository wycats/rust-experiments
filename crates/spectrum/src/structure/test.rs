use std::error::Error;

use crate::prelude::*;
use crate::{emit::buf::Buf, EmitBackend};

use super::{renderer::StyledRenderer, Structure};

pub fn render<'a>(
    structure: &Structure<()>,
    backend: impl Into<EmitBackend<'a>>,
    width: usize,
) -> Result<String, Box<dyn Error>> {
    let pretty = structure.clone().render();

    Ok(Buf::collect_string(|write| {
        let mut renderer = StyledRenderer::new(write, backend.into(), ());
        pretty
            .render_raw(width, &mut renderer)
            .map_err(|_| std::fmt::Error)
    })?)
}
