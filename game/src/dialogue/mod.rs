use std::collections::HashMap;

use fyrox::{gui::{text::{TextMessage, TextBuilder}, message::MessageDirection, UiNode, stack_panel::StackPanelBuilder, widget::{WidgetBuilder, WidgetMessage}, BuildContext, button::ButtonBuilder, HorizontalAlignment}, core::{pool::Handle, algebra::Vector2}, asset::manager::ResourceManager};

use crate::{create_nine_box, nine_patch::center_widget_builder};
pub fn show_dialogue(ui: &&mut fyrox::gui::UserInterface, dialogue_box:Handle<UiNode>, msg: String) {
    //ui.send_message(WidgetMessage::remove(dialogue_box,MessageDirection::ToWidget));
    ui.send_message(WidgetMessage::visibility(dialogue_box, MessageDirection::ToWidget, true));
    ui.send_message(TextMessage::text(
        dialogue_box,
        MessageDirection::ToWidget,
        msg,
    ));
}

pub fn show_choices(
    ctx: &mut BuildContext,
    resource_manager: &ResourceManager,
    choices:HashMap::<String, String>
) -> ChoiceRet {
    let mut buttons = HashMap::<Handle<UiNode>, String>::new();
    let mut wb = WidgetBuilder::new();
    for (choice_name, choice_jump) in choices{
        let center = TextBuilder::new(
            center_widget_builder()
                .with_max_size(Vector2::new(200.0, f32::INFINITY))
        )
        .with_text(choice_name)
        .build(ctx);
        let nine = create_nine_box(
            ctx,
            resource_manager,
            "data/choice9boxblur.png",
            40,
            41,
            40,
            41,
            81,
            81,
            Some(center)
        );
        let button = ButtonBuilder::new(WidgetBuilder::new()
            .with_horizontal_alignment(HorizontalAlignment::Center)
            .with_child(nine)
        )
        .build(ctx);
        wb = wb.with_child(button.clone());
        buttons.insert(button, choice_jump);
    }
    ChoiceRet {
        choice_container: StackPanelBuilder::new(wb).build(ctx),
        choices: buttons
    }


}

pub struct ChoiceRet {
    pub choice_container:Handle<UiNode>,
    pub choices: HashMap<Handle<UiNode>, String>
}