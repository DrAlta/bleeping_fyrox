//! Game project.
use fyrox::{
    core::{algebra::Vector2, pool::Handle, rand::Rng, color::Color},
    engine::{ GraphicsContext, resource_manager::ResourceManager},
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    dpi::PhysicalSize,
    gui::{
        formatted_text::WrapMode,
        text::TextBuilder,
        button::{ButtonBuilder, ButtonMessage},
        message::{MessageDirection, UiMessage},
        widget::{WidgetBuilder, WidgetMessage},
        UiNode,
    },
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    rand::thread_rng,
    scene::Scene, utils::into_gui_texture,
};
use fyrox_ui::{BuildContext, image::ImageBuilder, grid::{GridBuilder, GridDimension}};
pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, #[allow(unused_variables)] context: PluginRegistrationContext) {
        
    }
    fn create_instance(
        &self,
        _override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        let resource_manager = context.resource_manager;
        let window_size =context.user_interface.screen_size();
        println!("screen_size:{}", window_size);
        let ctx = &mut context.user_interface.build_ctx();
        let button = ButtonBuilder::new(WidgetBuilder::new())
            .with_text("Click me!")
            .build(ctx);
        //let text = create_text(ctx, window_size);
        let text = create_nine_box(ctx, resource_manager);
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
                    ui
                        .send_message(WidgetMessage::desired_position(
                            self.button,
                            MessageDirection::ToWidget,
                            new_position,
                        ));
                    // "Tell" the text to adjust to window size.
                    ui
                        .send_message(WidgetMessage::width(
                            self.text,
                            MessageDirection::ToWidget,
                            base_x * 4.0,
                        ));
                    ui
                        .send_message(WidgetMessage::height(
                            self.text,
                            MessageDirection::ToWidget,
                            base_y * 4.0,
                        ));
                }
            }
        }
    }
}


fn create_text(ctx: &mut BuildContext, window_size: Vector2<f32>) -> Handle<UiNode> {
    let base_x = window_size.x / 6.0;
    let base_y = window_size.y / 6.0;
    TextBuilder::new(WidgetBuilder::new()
        .with_width(base_x * 4.0)
        .with_height(base_y * 4.0)
        .with_desired_position(Vector2::new(base_x, base_y))
    )
        .with_wrap(WrapMode::Word)
        .with_text("It feels right to notice all the shiny things about you there is nothing I wouldn’t want to know of you nothing is simple yet nothing is simpler")
        .build(ctx)
}

fn foobar(text: Handle<UiNode>, context: PluginContext, size: &PhysicalSize<u32>){
  
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

fn create_image(ctx: &mut BuildContext, resource_manager: &ResourceManager, path: &str) -> Handle<UiNode> {
    // You must explicitly set width and height of the image, otherwise it will collapse to a
    // point and you won't see anything.
    let width = 100.0;
    let height = 100.0;
    ImageBuilder::new(WidgetBuilder::new().with_width(width).with_height(height))        
        .with_texture(into_gui_texture(
            // Ask resource manager to load a texture.
            resource_manager.request_texture(path),//"path/to/your/texture.png"),
        ))
        .build(ctx)
}

fn create_nine_box(ctx: &mut BuildContext, resource_manager: &ResourceManager) -> Handle<UiNode> {

        GridBuilder::new(
            WidgetBuilder::new()
                .with_child(//top left
                    ImageBuilder::new(WidgetBuilder::new().with_width(40.0).with_height(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxTopLeft.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//top cent
                    ImageBuilder::new(WidgetBuilder::new().on_column(1).with_height(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxTopCenter.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//top right
                    ImageBuilder::new(WidgetBuilder::new().on_column(2).with_width(40.0).with_height(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxTopRight.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//middle left
                    ImageBuilder::new(WidgetBuilder::new().on_row(1).with_width(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxMiddleLeft.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//middle cent
                    ImageBuilder::new(WidgetBuilder::new().on_row(1).on_column(1))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxMiddleCenter.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//middle right
                    ImageBuilder::new(WidgetBuilder::new().on_row(1).on_column(2).with_width(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxMiddleRight.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//bottom left
                    ImageBuilder::new(WidgetBuilder::new().on_row(2).with_width(40.0).with_height(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxBottomLeft.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//bottom cent
                    ImageBuilder::new(WidgetBuilder::new().on_row(2).on_column(1).with_height(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxBottomCenter.png"),
                    ))
                    .build(ctx)
                )
                .with_child(//bottom right
                    ImageBuilder::new(WidgetBuilder::new().on_row(2).on_column(2).with_width(40.0).with_height(40.0))        
                    .with_texture(into_gui_texture(
                        resource_manager.request_texture("data/9boxBottomRight.png"),
                    ))
                    .build(ctx)
                ))
                .add_row(GridDimension::strict(40.0))
                .add_row(GridDimension::stretch() )
                .add_row(GridDimension::strict(40.0))
                .add_column(GridDimension::strict(40.0))
                .add_column(GridDimension::stretch() )
                .add_column(GridDimension::strict(40.0))
                .build(ctx)
       
}