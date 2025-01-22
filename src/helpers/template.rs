// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext, RenderError,
    RenderErrorReason,
};

use super::formatter;

pub fn format_line(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    output: &mut dyn Output,
) -> HelperResult {
    if let Some(parameter) = helper.param(0) {
        let html = formatter::linkify(parameter.value().render().as_ref());

        output.write(&html)?;

        Ok(())
    } else {
        Err(RenderError::from(RenderErrorReason::Other(
            "Missing line text".to_string(),
        )))
    }
}
