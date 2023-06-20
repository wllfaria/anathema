use anathema_render::Size;

use super::{Constraints, Layout, Padding};
use crate::contexts::{LayoutCtx, PositionCtx};
use crate::error::{Error, Result};
use crate::gen::generator::Generator;
use crate::{Axis, WidgetContainer};

pub struct Single;

impl Layout for Single {
    fn layout<'widget, 'tpl, 'parent>(
        &mut self,
        ctx: &mut LayoutCtx<'widget, 'tpl, 'parent>,
        size: &mut Size,
    ) -> Result<()> {
        let constraints = ctx.padded_constraints();
        let mut values = ctx.values.next();
        let mut gen = Generator::new(ctx.templates, ctx.lookup, &mut values);

        if let Some(mut widget) = gen.next(&mut values).transpose()? {
            ctx.children.push(widget);
            *size = match ctx.children[0].layout(constraints, &values, ctx.lookup) {
                Ok(s) => s,
                Err(Error::InsufficientSpaceAvailble) => return Ok(()),
                err @ Err(_) => err?,
            };
        }

        Ok(())
    }
}
