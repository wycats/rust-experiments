use std::error::Error;

use crate::{emit::buf::Buf, string::copy_string::SimpleContext, EmitBackend};
use crate::{prelude::*, string::copy_string::StringContext};

use super::{renderer::StyledRenderer, Structure};

pub fn render<'a>(
    structure: &Structure<SimpleContext>,
    backend: impl Into<EmitBackend<'a>>,
    width: usize,
) -> Result<String, Box<dyn Error>> {
    let pretty = structure.clone().render();

    Ok(Buf::collect_string(|write| {
        let ctx = &mut SimpleContext;
        let mut renderer = StyledRenderer::new(write, backend.into(), ctx);
        pretty
            .render_raw(width, &mut renderer)
            .map_err(|_| std::fmt::Error)
    })?)
}

pub fn render_with<'a, 'b, C: StringContext + 'static>(
    structure: &Structure<C>,
    backend: impl Into<EmitBackend<'a>>,
    width: usize,
    ctx: &mut C,
) -> Result<String, Box<dyn Error>> {
    let pretty = structure.clone().render_with(ctx);

    Ok(Buf::collect_string(|write| {
        let mut renderer: StyledRenderer<C> = StyledRenderer::new(write, backend.into(), &ctx);
        pretty
            .render_raw(width, &mut renderer)
            .map_err(|_| std::fmt::Error)
    })?)
}
