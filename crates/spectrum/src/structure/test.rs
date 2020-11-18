use std::error::Error;

use crate::{emit::buf::Buf, string::copy_string::SimpleContext, EmitBackendTrait};
use crate::{prelude::*, string::copy_string::StringContext};

use super::{renderer::StyledRenderer, Structure};

pub fn render<'borrow, 'ctx>(
    structure: &'borrow Structure<'ctx, SimpleContext>,
    backend: &'borrow dyn EmitBackendTrait,
    width: usize,
) -> Result<String, Box<dyn Error + 'static>>
where
    'ctx: 'borrow,
{
    let structure = structure.clone();

    Ok(Buf::collect_string(move |write| {
        let ctx = &mut SimpleContext;
        let pretty = structure.render();

        let mut renderer = StyledRenderer::new(write, backend, ctx);
        pretty
            .render_raw(width, &mut renderer)
            .map_err(|_| std::fmt::Error)
    })?)
}

pub fn render_with<'borrow, 'ctx, C: StringContext<'ctx> + 'ctx>(
    structure: &'borrow Structure<'ctx, C>,
    backend: &'borrow dyn EmitBackendTrait,
    width: usize,
    ctx: &'borrow mut C,
) -> Result<String, Box<dyn Error + 'static>> {
    let pretty = structure.clone().render_with(ctx);

    Ok(Buf::collect_string(|write| {
        let mut renderer: StyledRenderer<C> = StyledRenderer::new(write, backend, ctx);
        pretty
            .render_raw(width, &mut renderer)
            .map_err(|_| std::fmt::Error)
    })?)
}
