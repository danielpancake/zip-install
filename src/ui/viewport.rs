use eframe::egui::{Context, ViewportBuilder, ViewportCommand};

pub fn apply_viewport_field<T>(ctx: &Context, field: Option<T>, cmd: fn(T) -> ViewportCommand) {
    if let Some(value) = field {
        ctx.send_viewport_cmd(cmd(value));
    }
}

pub fn apply_viewport_builder(ctx: &Context, builder: ViewportBuilder) {
    use ViewportCommand::*;

    apply_viewport_field(ctx, builder.title, Title);
    apply_viewport_field(ctx, builder.inner_size, InnerSize);
    apply_viewport_field(ctx, builder.min_inner_size, MinInnerSize);
    apply_viewport_field(ctx, builder.max_inner_size, MaxInnerSize);
    apply_viewport_field(ctx, builder.position, OuterPosition);
    apply_viewport_field(ctx, builder.fullscreen, Fullscreen);
    apply_viewport_field(ctx, builder.maximized, Maximized);
    apply_viewport_field(ctx, builder.resizable, Resizable);
}
