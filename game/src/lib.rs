//! Game project.
mod grid;
use grid::{GridBuilder, GridDimension};

use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector2, color::Color, math::Rect, pool::Handle, rand::Rng},
    dpi::PhysicalSize,
    engine::GraphicsContext,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        formatted_text::WrapMode,
        //grid::{GridBuilder, GridDimension},
        image::ImageBuilder,
        message::{MessageDirection, UiMessage},
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        widget::{WidgetBuilder, WidgetMessage},
        BuildContext,
        HorizontalAlignment,
        UiNode,
    },
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    rand::thread_rng,
    resource::texture::Texture,
    scene::Scene,
    utils::into_gui_texture,
};
pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, #[allow(unused_variables)] context: PluginRegistrationContext) {}
    fn create_instance(
        &self,
        _override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        let resource_manager = context.resource_manager;
        let window_size = context.user_interface.screen_size();
        println!("screen_size:{}", window_size);
        let ctx = &mut context.user_interface.build_ctx();
        let button = ButtonBuilder::new(WidgetBuilder::new())
            .with_text("Click me!")
            .build(ctx);
        //let text = create_text(ctx, window_size);
        let text = create_stack_panel(ctx, resource_manager);
        Box::new(Game { button, text })
    }
}

struct Game {
    button: Handle<UiNode>,
    text: Handle<UiNode>,
}

impl Plugin for Game {
    fn on_graphics_context_initialized(
        &mut self,
        context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
        context
            .graphics_context
            .as_initialized_mut()
            .renderer
            .set_backbuffer_clear_color(Color::GREEN);
    }
    fn on_os_event(
        &mut self,
        event: &fyrox::event::Event<()>,
        context: PluginContext,
        control_flow: &mut ControlFlow,
    ) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    foobar(self.text, context, size);
                }
                _ => {}
            }
        }
    }
    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
        // Simple example of message system. We'll catch "Click" messages from the button
        // and send new message to the button that will contain new position for it.
        if let Some(ButtonMessage::Click) = message.data::<ButtonMessage>() {
            if message.destination() == self.button {
                // Generate random position in the window.
                if let GraphicsContext::Initialized(ref graphics_context) = context.graphics_context
                {
                    let client_size = graphics_context.window.inner_size();
                    let base_x = client_size.width as f32 / 6.0;
                    let base_y = client_size.height as f32 / 6.0;

                    let mut rng = thread_rng();

                    let new_position = Vector2::new(
                        rng.gen_range(0.0..(client_size.width as f32 - 100.0)),
                        rng.gen_range(0.0..(client_size.height as f32 - 100.0)),
                    );

                    let ui = &context.user_interface;
                    // "Tell" the button to "teleport" in the new location.
                    ui.send_message(WidgetMessage::desired_position(
                        self.button,
                        MessageDirection::ToWidget,
                        new_position,
                    ));
                    // "Tell" the text to adjust to window size.
                    ui.send_message(WidgetMessage::width(
                        self.text,
                        MessageDirection::ToWidget,
                        base_x * 4.0,
                    ));
                    ui.send_message(WidgetMessage::height(
                        self.text,
                        MessageDirection::ToWidget,
                        base_y * 4.0,
                    ));
                }
            }
        }
    }
}

/*
fn create_text(ctx: &mut BuildContext, window_size: Vector2<f32>) -> Handle<UiNode> {
    let base_x = window_size.x / 6.0;
    let base_y = window_size.y / 6.0;
    TextBuilder::new(WidgetBuilder::new()
        .with_width(base_x * 4.0)
        .with_height(base_y * 4.0)
        .with_desired_position(Vector2::new(base_x, base_y))
    )
        .with_wrap(WrapMode::Word)
        .with_text("It feels"
    )
        .build(ctx)
}


fn create_image(ctx: &mut BuildContext, resource_manager: &ResourceManager, path: &str) -> Handle<UiNode> {
    // You must explicitly set width and height of the image, otherwise it will collapse to a
    // point and you won't see anything.
    let width = 100.0;
    let height = 100.0;
    ImageBuilder::new(WidgetBuilder::new().with_width(width).with_height(height))
        .with_texture(into_gui_texture(
            // Ask resource manager to load a texture.
            resource_manager.request::<Texture, _>(path),//"path/to/your/texture.png"),
        ))
        .build(ctx)
}
*/
fn foobar(text: Handle<UiNode>, context: PluginContext, size: &PhysicalSize<u32>) {
    let base_x = (size.width as f32 / 6.0).floor();
    let base_y = (size.height as f32 / 6.0).floor();

    let ui = &context.user_interface;

    ui.send_message(WidgetMessage::desired_position(
        text,
        MessageDirection::ToWidget,
        Vector2::new(base_x, base_y),
    ));
    ui.send_message(WidgetMessage::width(
        text,
        MessageDirection::ToWidget,
        base_x * 4.0,
    ));
    ui.send_message(WidgetMessage::height(
        text,
        MessageDirection::ToWidget,
        base_y * 4.0,
    ));
    println!("size=X:{} Y:{}", base_x * 4.0, base_y * 4.0);
}

fn create_nine_box(
    ctx: &mut BuildContext,
    resource_manager: &ResourceManager,
    image: &str,
    x_fence_post1_pixel: u32,
    x_fence_post2_pixel: u32,
    y_fence_post1_pixel: u32,
    y_fence_post2_pixel: u32,
    image_width: u32,
    image_height: u32,
    text: &str,
) -> Handle<UiNode> {
    let column1_width_pixels = x_fence_post1_pixel;
    let column2_width_pixels = x_fence_post2_pixel - x_fence_post1_pixel;
    let column3_width_pixels = image_width - x_fence_post2_pixel;

    let row1_height_pixels = y_fence_post1_pixel;
    let row2_height_pixels = y_fence_post2_pixel - y_fence_post1_pixel;
    let row3_height_pixels = image_height - y_fence_post2_pixel;

    let column1_width_uv = column1_width_pixels as f32 / image_width as f32;
    let column2_width_uv = column2_width_pixels as f32 / image_width as f32;
    let column3_width_uv = column3_width_pixels as f32 / image_width as f32;

    let row1_height_uv = row1_height_pixels as f32 / image_height as f32;
    let row2_height_uv = row2_height_pixels as f32 / image_height as f32;
    let row3_height_uv = row3_height_pixels as f32 / image_height as f32;

    let x_fence_post1_uv = x_fence_post1_pixel as f32 / image_width as f32;
    let x_fence_post2_uv = x_fence_post2_pixel as f32 / image_width as f32;
    let y_fence_post1_uv = y_fence_post1_pixel as f32 / image_height as f32;
    let y_fence_post2_uv = y_fence_post2_pixel as f32 / image_height as f32;
    GridBuilder::new(
        WidgetBuilder::new()
            .with_horizontal_alignment(HorizontalAlignment::Center)
            .with_child(
                //top left
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .with_width(column1_width_pixels as f32)
                        .with_height(row1_height_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(0.0, 0.0, column1_width_uv, row1_height_uv))
                .build(ctx),
            )
            .with_child(
                //top cent
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_column(1)
                        .with_height(row1_height_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    x_fence_post1_uv,
                    0.0,
                    column2_width_uv,
                    row1_height_uv,
                ))
                .build(ctx),
            )
            .with_child(
                //top right
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_column(2)
                        .with_width(column3_width_pixels as f32)
                        .with_height(row1_height_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    x_fence_post2_uv,
                    0.0,
                    column3_width_uv,
                    row1_height_uv,
                ))
                .build(ctx),
            )
            .with_child(
                //middle left
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(1)
                        .with_width(column1_width_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    0.0,
                    y_fence_post1_uv,
                    column1_width_uv,
                    row2_height_uv,
                ))
                .build(ctx),
            )
            .with_child(
                //middle cent
                ImageBuilder::new(WidgetBuilder::new().on_row(1).on_column(1))
                    .with_texture(into_gui_texture(
                        resource_manager.request::<Texture, _>(image),
                    ))
                    .with_uv_rect(Rect::new(
                        x_fence_post1_uv,
                        y_fence_post1_uv,
                        column2_width_uv,
                        row2_height_uv,
                    ))
                    .build(ctx),
            )
            .with_child(
                //middle cent
                TextBuilder::new(
                    WidgetBuilder::new()
                        .with_max_size(Vector2::new(200.0, f32::INFINITY))
                        .on_row(1)
                        .on_column(1),
                )
                .with_wrap(WrapMode::Word)
                .with_text(text)
                .build(ctx),
            )
            .with_child(
                //middle right
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(1)
                        .on_column(2)
                        .with_width(column3_width_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    x_fence_post2_uv,
                    y_fence_post1_uv,
                    column3_width_uv,
                    row2_height_uv,
                ))
                .build(ctx),
            )
            .with_child(
                //bottom left
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(2)
                        .with_width(column1_width_pixels as f32)
                        .with_height(row3_height_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    0.0,
                    y_fence_post2_uv,
                    column1_width_uv,
                    row3_height_uv,
                ))
                .build(ctx),
            )
            .with_child(
                //bottom cent
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(2)
                        .on_column(1)
                        .with_height(row3_height_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    x_fence_post1_uv,
                    y_fence_post2_uv,
                    column2_width_uv,
                    row3_height_uv,
                ))
                .build(ctx),
            )
            .with_child(
                //bottom right
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(2)
                        .on_column(2)
                        .with_width(column3_width_pixels as f32)
                        .with_height(row3_height_pixels as f32),
                )
                .with_texture(into_gui_texture(
                    resource_manager.request::<Texture, _>(image),
                ))
                .with_uv_rect(Rect::new(
                    x_fence_post2_uv,
                    y_fence_post2_uv,
                    column3_width_uv,
                    row3_height_uv,
                ))
                .build(ctx),
            ),
    )
    .add_row(GridDimension::strict(40.0))
    .add_row(GridDimension::auto())
    .add_row(GridDimension::strict(40.0))
    .add_column(GridDimension::strict(40.0))
    .add_column(GridDimension::auto())
    .add_column(GridDimension::strict(40.0))
    .build(ctx)
}

fn create_stack_panel(
    ctx: &mut BuildContext,
    resource_manager: &ResourceManager,
) -> fyrox::core::pool::Handle<UiNode> {
    StackPanelBuilder::new(
        WidgetBuilder::new()
            /*
            .with_child(
                create_nine_box(
                    ctx,
                    resource_manager,
                    "data/9boxblur.png",
                    40,
                    41,
                    40,
                    41,
                    81,
                    81,
                    "It feels right to notice all the shiny things about you, about you there is nothing I wouldn’t want to know, to know you nothing is simple, nothing is simple yet nothing is simpler"
                )
            )
            .with_child(
                create_nine_box(
                    ctx,
                    resource_manager,
                    "data/9boxblur.png",
                    40,
                    41,
                    40,
                    41,
                    81,
                    81,
                    "It feels right to notice all the shiny things about you"
                )
            )
            */
            .with_child(create_nine_box(
                ctx,
                resource_manager,
                "data/9boxblur.png",
                40,
                41,
                40,
                41,
                81,
                81,
                "about you",
            ))
            .with_child(create_nine_box(
                ctx,
                resource_manager,
                "data/9boxblur.png",
                40,
                41,
                40,
                41,
                81,
                81,
                "to know you nothing is simple",
            ))
            .with_child(create_nine_box(
                ctx,
                resource_manager,
                "data/9boxblur.png",
                40,
                41,
                40,
                41,
                81,
                81,
                "nothing is simple yet nothing is simpler",
            )),
    )
    .build(ctx)
}
