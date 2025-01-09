use egui::RawInput;

fn egui_handling() {
    let ctx = egui::Context::default();
    let raw_input = gather_input();

    let full_output = ctx.run(raw_input, |ctx| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Hello world!");
            if ui.button("Click me").clicked() {
                // take some action here
            }
        });
    });
    handle_platform_output(full_output.platform_output);
    let clipped_primitives = ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
    paint(full_output.textures_delta, clipped_primitives);
}

fn paint(textures_delta: egui::TexturesDelta, clipped_primitives: Vec<egui::ClippedPrimitive>) {
    todo!()
}

fn handle_platform_output(platform_output: egui::PlatformOutput) {
    todo!()
}

fn gather_input() -> RawInput {
    todo!()
}
