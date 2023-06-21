#![allow(dead_code)]
use crate::grid::{GridBuilder, GridDimension};
use fyrox::{
    asset::manager::ResourceManager,
    core::{math::Rect, pool::Handle},
    gui::{
        image::ImageBuilder,
        widget::{WidgetBuilder},
        BuildContext,
        HorizontalAlignment,
        UiNode,
    },
    resource::texture::Texture,
    utils::into_gui_texture,
};


pub fn create_nine_box(
    ctx: &mut BuildContext,
    resource_manager: &ResourceManager,
    image: &str,
    x_fence_post1_pixel: u32,
    x_fence_post2_pixel: u32,
    y_fence_post1_pixel: u32,
    y_fence_post2_pixel: u32,
    image_width: u32,
    image_height: u32,
    center:Option<Handle<UiNode>>,
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

    //let test = resource_manager.request::<Texture, _>(image);
   // let test2 = test.data_ref().kind().rectangle_size().unwrap();

    let mut wb = 
        WidgetBuilder::new()
        .with_desired_position(fyrox::core::algebra::Vector2::new(50.0, 50.0))
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
            );
    if let Some(handle) = center{
        wb = wb.with_child(
        //middle cent
        handle
        );
    }
    GridBuilder::new(wb
    )
    .add_row(GridDimension::strict(40.0))
    .add_row(GridDimension::auto())
    .add_row(GridDimension::strict(40.0))
    .add_column(GridDimension::strict(40.0))
    .add_column(GridDimension::auto())
    .add_column(GridDimension::strict(40.0))

    .build(ctx)
}


pub struct NinePatchBuilder<'a> {
    resource_manager: &'a ResourceManager,
    image_path: &'a str,
    x_fence_post1_pixel: u32,
    x_fence_post2_pixel: u32,
    y_fence_post1_pixel: u32,
    y_fence_post2_pixel: u32,
    image_width: u32,
    image_height: u32,
    center: Option<Handle<UiNode>>
}
impl NinePatchBuilder<'_> {
    pub fn with_center(mut self, center:Handle<UiNode>) -> Self {
        self.center = Some(center);
        self
    }
    pub fn build(self, ui:&mut BuildContext) -> Handle<UiNode> {
        create_nine_box(ui, self.resource_manager, self.image_path, self.x_fence_post1_pixel, self.x_fence_post2_pixel, self.y_fence_post1_pixel, self.y_fence_post2_pixel, self.image_width, self.image_height, self.center)
    } 
    pub fn new<'a>(
        resource_manager: &'a ResourceManager,
        image_path: &'a str,
        x_fence_post1_pixel: u32,
        x_fence_post2_pixel: u32,
        y_fence_post1_pixel: u32,
        y_fence_post2_pixel: u32,
        image_width: u32,
        image_height: u32,
    ) -> NinePatchBuilder::<'a> {
        NinePatchBuilder{        
            resource_manager,
            image_path,
            x_fence_post1_pixel,
            x_fence_post2_pixel,
            y_fence_post1_pixel,
            y_fence_post2_pixel,
            image_width,
            image_height,
            center: None
    }
    }


}


pub fn center_widget_builder() -> WidgetBuilder {
    WidgetBuilder::new()
    .on_row(1)
    .on_column(1)
}