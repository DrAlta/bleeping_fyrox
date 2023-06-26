
use fyrox::gui::{
    core::{algebra::Vector2, math::Rect, pool::Handle, scope_profile},
    draw::{CommandTexture, Draw, DrawingContext, SharedTexture},
    message::UiMessage,
    widget::{Widget, WidgetBuilder},
    BuildContext, Control, UiNode, UserInterface,
};
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SizeMode {
    Strict(u32),
    Auto,
}




/// Automatically arranges children by rows and columns
#[derive(Clone)]
pub struct NinePatch {
    pub widget: Widget,
    texture: Option<SharedTexture>,
    pub x_size_mode: SizeMode,
    pub y_size_mode: SizeMode,
    pub x_fence_post1_pixel: u32,
    pub x_fence_post2_pixel: u32,
    pub y_fence_post1_pixel: u32,
    pub y_fence_post2_pixel: u32,
    pub image_width: u32,
    pub image_height: u32,

}

fyrox::gui::define_widget_deref!(NinePatch);


impl Control for NinePatch {
    fn query_component(&self, type_id: TypeId) -> Option<&dyn Any> {
        if type_id == TypeId::of::<Self>() {
            Some(self)
        } else {
            None
        }
    }

    fn measure_override(&self, ui: &UserInterface, available_size: Vector2<f32>) -> Vector2<f32> {
        scope_profile!();

        let column1_width_pixels = self.x_fence_post1_pixel as f32;
        let column3_width_pixels = (self.image_width - self.x_fence_post2_pixel) as f32;
    
        let row1_height_pixels = self.y_fence_post1_pixel as f32;
        let row3_height_pixels = (self.image_height - self.y_fence_post2_pixel) as f32;

        let x_overflow = column1_width_pixels + column3_width_pixels;
        let y_overflow = row1_height_pixels as f32 + row3_height_pixels;

        let mut size: Vector2<f32> = Vector2::new(self.image_width as f32, self.image_height as f32);

        

        let mut center_size = Vector2::new(available_size.x - x_overflow, available_size.y - y_overflow);

        if let SizeMode::Strict(strict_x) = self.x_size_mode {
            center_size.x = center_size.x.min(strict_x as f32 - x_overflow);
        }

        if let SizeMode::Strict(strict_y) = self.y_size_mode {
            center_size.y = center_size.y.min(strict_y as f32 - y_overflow);
        }

        for &child in self.children.iter() {
            ui.measure_node(child, center_size);
            let desired_size = ui.node(child).desired_size();
            size.x = size.x.max(desired_size.x.ceil());
            size.y = size.y.max(desired_size.y.ceil());
        }
        //println!("patch measure:{size:?}");
        size

    }


    fn arrange_override(&self, ui: &UserInterface, final_size: Vector2<f32>) -> Vector2<f32> {
        scope_profile!();
        let column1_width_pixels = self.x_fence_post1_pixel as f32;
        let column3_width_pixels = (self.image_width - self.x_fence_post2_pixel) as f32;
    
        let row1_height_pixels = self.y_fence_post1_pixel as f32;
        let row3_height_pixels = (self.image_height - self.y_fence_post2_pixel) as f32;

        let x_overflow = column1_width_pixels + column3_width_pixels;
        let y_overflow = row1_height_pixels as f32 + row3_height_pixels;

        let final_rect = Rect::new(column1_width_pixels, row1_height_pixels , final_size.x - x_overflow, final_size.y - y_overflow);

        for &child in self.children.iter() {
            ui.arrange_node(child, &final_rect);
        }

        final_size
    }
 
    fn draw(&self, drawing_context: &mut DrawingContext) {
        if self.texture.is_some() {
            let patch_bounds = self.widget.bounding_rect();


            let column1_width_pixels = self.x_fence_post1_pixel as f32;
            let column2_width_pixels = (self.x_fence_post2_pixel - self.x_fence_post1_pixel) as f32;
            let column3_width_pixels = (self.image_width - self.x_fence_post2_pixel) as f32;
        
            let row1_height_pixels = self.y_fence_post1_pixel as f32;
            let row2_height_pixels = (self.y_fence_post2_pixel - self.y_fence_post1_pixel) as f32;
            let row3_height_pixels = (self.image_height - self.y_fence_post2_pixel) as f32;
        
            let column1_width_uv = column1_width_pixels as f32 / self.image_width as f32;
            let column2_width_uv = column2_width_pixels as f32 / self.image_width as f32;
            let column3_width_uv = column3_width_pixels as f32 / self.image_width as f32;
        
            let row1_height_uv = row1_height_pixels as f32 / self.image_height as f32;
            let row2_height_uv = row2_height_pixels as f32 / self.image_height as f32;
            let row3_height_uv = row3_height_pixels as f32 / self.image_height as f32;
        
            let x_fence_post1_uv = self.x_fence_post1_pixel as f32 / self.image_width as f32;
            let x_fence_post2_uv = self.x_fence_post2_pixel as f32 / self.image_width as f32;
            let y_fence_post1_uv = self.y_fence_post1_pixel as f32 / self.image_height as f32;
            let y_fence_post2_uv = self.y_fence_post2_pixel as f32 / self.image_height as f32;

            let x_overflow = column1_width_pixels + column3_width_pixels;
            let y_overlfow = row1_height_pixels + row3_height_pixels;

            //top left
            let bounds = Rect { 
                position: patch_bounds.position, 
                size: Vector2::new(
                    column1_width_pixels, 
                    row1_height_pixels
                ) 
            };
            let tex_coords= [
                Vector2::<f32>::new(0.0, 0.0),
                Vector2::new(x_fence_post1_uv, 0.0),
                Vector2::new(x_fence_post1_uv, y_fence_post1_uv),
                Vector2::new(0.0, y_fence_post1_uv),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context
            );

            //top center
            let bounds = Rect { 
                position: Vector2::new(
                    patch_bounds.position.x + column1_width_pixels,
                    patch_bounds.position.y
                ), 
                size: Vector2::new(
                    patch_bounds.size.x - x_overflow, 
                    row1_height_pixels
                )
            };
            let tex_coords= [
                Vector2::<f32>::new(x_fence_post1_uv, 0.0),
                Vector2::new(x_fence_post2_uv, 0.0),
                Vector2::new(x_fence_post2_uv, y_fence_post1_uv),
                Vector2::new(x_fence_post1_uv, y_fence_post1_uv),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context);

            //top right
            let bounds = Rect { 
                position: Vector2::new(
                    (patch_bounds.position.x + patch_bounds.size.x) - column3_width_pixels,
                    patch_bounds.position.y
                ), 
                size: Vector2::new(
                    column3_width_pixels, 
                    row1_height_pixels
                )
            };
            let tex_coords= [
                Vector2::<f32>::new(x_fence_post2_uv, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(1.0, y_fence_post1_uv),
                Vector2::new(x_fence_post2_uv, y_fence_post1_uv),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context);
            ////////////////////////////////////////////////////////////////////////////////
            //middle left
            let bounds = Rect { 
                position: Vector2::new(
                    patch_bounds.position.x, 
                    patch_bounds.position.y + row1_height_pixels
                ),
                size: Vector2::new(
                    column1_width_pixels, 
                    patch_bounds.size.y - y_overlfow
                ) 
            };
            let tex_coords= [
                Vector2::<f32>::new(0.0, y_fence_post1_uv),
                Vector2::new(x_fence_post1_uv, y_fence_post1_uv),
                Vector2::new(x_fence_post1_uv, y_fence_post2_uv),
                Vector2::new(0.0, y_fence_post2_uv),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context
            );

            //middle center
            let bounds = Rect { 
                position: Vector2::new(
                    patch_bounds.position.x + column1_width_pixels,
                    patch_bounds.position.y + row1_height_pixels
                ), 
                size: Vector2::new(
                    patch_bounds.size.x - x_overflow, 
                    patch_bounds.size.y - y_overlfow
                )
            };
            let tex_coords= [
                Vector2::<f32>::new(x_fence_post1_uv, y_fence_post1_uv),
                Vector2::new(x_fence_post2_uv, y_fence_post1_uv),
                Vector2::new(x_fence_post2_uv, y_fence_post2_uv),
                Vector2::new(x_fence_post1_uv, y_fence_post2_uv),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context);

            //middle right
            let bounds = Rect { 
                position: Vector2::new(
                    (patch_bounds.position.x + patch_bounds.size.x) - column3_width_pixels,
                    patch_bounds.position.y + row1_height_pixels
                ), 
                size: Vector2::new(
                    column3_width_pixels, 
                    patch_bounds.size.y - y_overlfow
                )
            };
            let tex_coords= [
                Vector2::<f32>::new(x_fence_post2_uv, y_fence_post1_uv),
                Vector2::new(1.0, y_fence_post1_uv),
                Vector2::new(1.0, y_fence_post2_uv),
                Vector2::new(x_fence_post2_uv, y_fence_post2_uv),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context);

            ////////////////////////////////////////////////////////////////////////////////
            //bottom left
            let bounds = Rect { 
                position: Vector2::new(
                    patch_bounds.position.x, 
                    (patch_bounds.position.y + patch_bounds.size.y) - row3_height_pixels
                ),
                size: Vector2::new(
                    column1_width_pixels, 
                    row3_height_pixels
                ) 
            };
            let tex_coords= [
                Vector2::<f32>::new(0.0, y_fence_post2_uv),
                Vector2::new(x_fence_post1_uv, y_fence_post2_uv),
                Vector2::new(x_fence_post1_uv, 1.0),
                Vector2::new(0.0, 1.0),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context
            );

            //bottom center
            let bounds = Rect { 
                position: Vector2::new(
                    patch_bounds.position.x + column1_width_pixels,
                    (patch_bounds.position.y + patch_bounds.size.y) - row3_height_pixels
                ), 
                size: Vector2::new(
                    patch_bounds.size.x - x_overflow, 
                    row3_height_pixels
                )
            };
            let tex_coords= [
                Vector2::<f32>::new(x_fence_post1_uv, y_fence_post2_uv),
                Vector2::new(x_fence_post2_uv, y_fence_post2_uv),
                Vector2::new(x_fence_post2_uv, 1.0),
                Vector2::new(x_fence_post1_uv, 1.0),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context);

            //bottom right
            let bounds = Rect { 
                position: Vector2::new(
                    (patch_bounds.position.x + patch_bounds.size.x) - column3_width_pixels,
                    (patch_bounds.position.y + patch_bounds.size.y) - row3_height_pixels
                ), 
                size: Vector2::new(
                    column3_width_pixels, 
                    row3_height_pixels
                )
            };
            let tex_coords= [
                Vector2::<f32>::new(x_fence_post2_uv, y_fence_post2_uv),
                Vector2::new(1.0, y_fence_post2_uv),
                Vector2::new(1.0, 1.0),
                Vector2::new(x_fence_post2_uv, 1.0),
            ];
            draw_image(
                &self.texture.as_ref().unwrap(), 
                bounds, 
                &tex_coords, 
                self.clip_bounds(), 
                self.widget.background(), 
                drawing_context);

            //end drawing
        }
    }

    fn handle_routed_message(&mut self, ui: &mut UserInterface, message: &mut UiMessage) {
        self.widget.handle_routed_message(ui, message);
    }
}

pub struct NinePatchBuilder {
    widget_builder: WidgetBuilder,
    texture: Option<SharedTexture>,
    pub x_size_mode: SizeMode,
    pub y_size_mode: SizeMode,
    pub x_fence_post1_pixel: u32,
    pub x_fence_post2_pixel: u32,
    pub y_fence_post1_pixel: u32,
    pub y_fence_post2_pixel: u32,
    pub image_width: u32,
    pub image_height: u32,
}

impl NinePatchBuilder {
    pub fn new(widget_builder: WidgetBuilder) -> Self {
        Self {
            widget_builder,
            texture: None,
            x_size_mode: SizeMode::Auto,
            y_size_mode: SizeMode::Auto,
            x_fence_post1_pixel: 40,
            x_fence_post2_pixel: 41,
            y_fence_post1_pixel: 40,
            y_fence_post2_pixel: 41,
            image_width: 81,
            image_height: 81,
        }
    }

    pub fn with_texture(mut self, texture: SharedTexture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn build(self, ui: &mut BuildContext) -> Handle<UiNode> {
        let grid = NinePatch {
            widget: self.widget_builder.build(),
            texture: self.texture,
            x_size_mode: self.x_size_mode,
            y_size_mode: self.y_size_mode,
            x_fence_post1_pixel: self.x_fence_post1_pixel,
            x_fence_post2_pixel: self.x_fence_post2_pixel,
            y_fence_post1_pixel: self.y_fence_post1_pixel,
            y_fence_post2_pixel: self.y_fence_post2_pixel,
            image_width: self.image_width,
            image_height: self.image_height,
            
        };
        ui.add_node(UiNode::new(grid))
    }
}
fn draw_image(image:&SharedTexture, bounds:Rect<f32>, tex_coords: &[Vector2<f32>; 4], clip_bounds: Rect<f32>, background: fyrox::gui::brush::Brush, drawing_context: &mut DrawingContext) {
    drawing_context.push_rect_filled(&bounds, Some(tex_coords));
    let texture = CommandTexture::Texture(image.clone());
    drawing_context.commit(clip_bounds, background, texture, None);
}

//draw_image(self.texture.unwrap(), bounds, tex_coords, self.clip_bounds(), self.widget.background, drawing_context)