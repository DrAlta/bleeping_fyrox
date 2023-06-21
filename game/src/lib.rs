//! Game project.

pub mod topic_list;

pub mod script;

/*
mod script_resource;
use script_resource::{ScriptResource, ScriptResourceLoader};
*/
pub mod grid;

mod nine_patch;
use nine_patch::{create_nine_box, center_widget_builder, NinePatchBuilder};

use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector2, color::Color, pool::Handle, rand::Rng},
    dpi::PhysicalSize,
    engine::GraphicsContext,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        message::{MessageDirection, UiMessage},
        stack_panel::StackPanelBuilder,
        widget::{WidgetBuilder, WidgetMessage},
        BuildContext,
        UiNode, text::{TextBuilder, TextMessage}, formatted_text::WrapMode,
    },
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    rand::thread_rng,
    scene::Scene,
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
        
        let text = TextBuilder::new(
            center_widget_builder()
                .with_max_size(Vector2::new(200.0, f32::INFINITY))
        )
        .with_wrap(WrapMode::Word)
        .with_text("about you")
        .build(ctx);
    
    


        let _nine = create_nine_box(
            ctx,
            resource_manager,
            "data/9boxblur.png",
            40,
            41,
            40,
            41,
            81,
            81,
            Some(text)
        );
        let scripts = script::load_from_file("data/scripts.json").unwrap();
        /*
        println!("\n\n");
        match script {
            Ok(value)=> println!("{:?}\n\n", value.keys()),
            Err(err) => println!("error!{:?}\n\n", err)
        }
        */

    

        Box::new(Game { button, text, scripts, current_script_pos: None })
    }
}

struct ScriptPos {
    pub script: String,
    pub index: usize,
}

struct Game {
    button: Handle<UiNode>,
    text: Handle<UiNode>,
    
    scripts: script::Scripts,
    current_script_pos: Option<ScriptPos>
    

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
                println!("pressed!");
                // Generate random position in the window.
                if let GraphicsContext::Initialized(ref graphics_context) = context.graphics_context
                {   let start = ScriptPos { script: "Pharaoh1".to_string(), index: 0 };
                    let ui = &context.user_interface;
                    let ScriptPos { script, index }= self.current_script_pos.as_ref().unwrap_or(&start);
                    let blurp = match self.scripts.get_blurp(script, index.clone()).unwrap() {
                        script::ScriptItem::Action(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::AddQuest(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::Animation(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::Blurp(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::Choice(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::Cue(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::End(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::Jump(blurp) => {
                            format!("{blurp:?}")
                        }
                        script::ScriptItem::OfferTopics(blurp) => {
                            format!("{blurp:?}")
                        }
                    };
                    ui.send_message(TextMessage::text(
                        self.text,
                        MessageDirection::ToWidget,
                        blurp,
                    ));
                    self.current_script_pos= Some(ScriptPos { script:script.to_string(), index: index.clone() + 1 })
                }
            }
        }
    }
}

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
}


fn create_stack_panel(
    ctx: &mut BuildContext,
    resource_manager: &ResourceManager,
) -> fyrox::core::pool::Handle<UiNode> {
    let center1= TextBuilder::new(
        center_widget_builder()
            .with_max_size(Vector2::new(200.0, f32::INFINITY))
    )
    .with_wrap(WrapMode::Word)
    .with_text("about you")
    .build(ctx);
    let center2= TextBuilder::new(
        center_widget_builder()
            .with_max_size(Vector2::new(200.0, f32::INFINITY))
    )
    .with_wrap(WrapMode::Word)
    .with_text("about you")
    .build(ctx);




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
                Some(center1)
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
                Some(center2)
            ))
            .with_child(
                NinePatchBuilder::new(
                    resource_manager,
                    "data/9boxblur.png",
                    40,
                    41,
                    40,
                    41,
                    81,
                    81,
                )
                .with_center(
                    TextBuilder::new(
                        center_widget_builder()
                            .with_max_size(Vector2::new(200.0, f32::INFINITY))
                    )
                    .with_wrap(WrapMode::Word)
                    .with_text("about you")
                    .build(ctx)
                )
                .build(ctx)
            ),
    )
    .build(ctx)
}

