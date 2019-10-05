use crate::prelude::*;

use presentation::prelude::*;

use super::super::gui::texturedvertex::TexturedVertex;

use cgmath::ortho;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Default, Debug)]
pub struct GuiHandlerCreateInfo {
    // default button textures
    pub menu_button: String,
    pub menu_button_selected: String,

    // path to the alphabet image
    pub font_path: String,
}

struct GuiSeparator {
    _descriptor_layout: Arc<DescriptorSetLayout>,
    _pipeline_layout: Arc<PipelineLayout>,

    _pipeline: Arc<Pipeline>,
}

struct DisplayableTexture {
    _descriptor_pool: Arc<DescriptorPool>,
    _descriptor_set: Arc<DescriptorSet>,

    _texture: Arc<Image>,
}

struct TextableColor {
    _descriptor_pool: Arc<DescriptorPool>,
    _descriptor_set: Arc<DescriptorSet>,

    _buffer: Arc<Buffer<f32>>,
}

struct CommandBufferState {
    command_buffer: Arc<CommandBuffer>,
    valid: Cell<bool>,
    text_buffers: RefCell<Vec<Arc<Buffer<TexturedVertex>>>>,
}

pub struct GuiHandler {
    device: Arc<Device>,
    queue: Arc<Queue>,

    width: Cell<u32>,
    height: Cell<u32>,

    top_ui: RefCell<Option<Rc<dyn TopGui>>>,

    command_buffers: TargetMode<Vec<CommandBufferState>>,

    text_objects: GuiSeparator,
    rectangle_objects: GuiSeparator,

    _bitmap_font: Arc<Image>,
    _bitmap_desc_pool: Arc<DescriptorPool>,
    bitmap_desc_set: Arc<DescriptorSet>,

    text_color_layout: Arc<DescriptorSetLayout>,

    internal_textures: RefCell<HashMap<String, DisplayableTexture>>,
    internal_colors: RefCell<HashMap<TextColor, TextableColor>>,

    ortho: Cell<cgmath::Matrix4<f32>>,

    icon_descriptor_layout: Arc<DescriptorSetLayout>,

    needs_update: Cell<bool>,
    text_change_queue: RefCell<Vec<Box<dyn Fn() -> VerboseResult<()>>>>,

    menu_button: String,
    menu_button_selected: String,

    // ----- gui handling -----
    frameables: RefCell<Vec<Arc<Frameable>>>,
    hoverables: RefCell<Vec<Arc<Hoverable>>>,
    clickables: RefCell<Vec<Arc<Clickable>>>,
    displayables: RefCell<Vec<Arc<Displayable>>>,
    selectables: RefCell<Vec<Arc<Selectable>>>,
    textables: RefCell<Vec<Arc<Textable>>>,
    writeables: RefCell<Vec<Arc<Writeable>>>,
    iconizables: RefCell<Vec<Arc<Iconizable>>>,

    mouse_x: Cell<u32>,
    mouse_y: Cell<u32>,

    current_writeable: RefCell<Option<Arc<Writeable>>>,
    current_hoverable: RefCell<Option<Arc<Hoverable>>>,
    current_clickable: RefCell<Option<Arc<Clickable>>>,
    current_selectable: RefCell<Option<Arc<Selectable>>>,
}

impl GuiHandler {
    pub(crate) fn new(
        gui_handler_create_info: GuiHandlerCreateInfo,
        target_mode: TargetMode<()>,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        render_core: &Box<dyn RenderCore>,
    ) -> VerboseResult<GuiHandler> {
        let command_buffers = match target_mode {
            TargetMode::Single(_) => {
                let command_buffers = Self::create_command_buffers(render_core)?;

                TargetMode::Single(command_buffers)
            }
            TargetMode::Stereo(_, _) => {
                let left_command_buffers = Self::create_command_buffers(render_core)?;
                let right_command_buffers = Self::create_command_buffers(render_core)?;

                TargetMode::Stereo(left_command_buffers, right_command_buffers)
            }
        };

        let (text_objs, color_layout) =
            GuiHandler::init_text_objects(device, render_core.gui_render_pass())?;
        let rect_objs = GuiHandler::init_rectangle_objects(device, render_core.gui_render_pass())?;

        let (bitmap_texture, bitmap_desc_pool, bitmap_desc_set) = GuiHandler::init_bitmap_font(
            device,
            queue,
            text_objs._descriptor_layout.clone(),
            &gui_handler_create_info.font_path,
        )?;

        let icon_descriptor_layout = DescriptorSetLayout::new()
            .add_layout_binding(
                0,
                VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                VK_SHADER_STAGE_FRAGMENT_BIT,
                0,
            )
            .build(device.clone())?;

        Ok(GuiHandler {
            device: device.clone(),
            queue: queue.clone(),

            width: Cell::new(render_core.width()),
            height: Cell::new(render_core.height()),

            top_ui: RefCell::new(None),

            command_buffers,

            text_objects: text_objs,
            rectangle_objects: rect_objs,

            _bitmap_font: bitmap_texture,
            _bitmap_desc_pool: bitmap_desc_pool,
            bitmap_desc_set,

            text_color_layout: color_layout,

            internal_textures: RefCell::new(HashMap::new()),
            internal_colors: RefCell::new(HashMap::new()),

            icon_descriptor_layout,

            needs_update: Cell::new(true),
            text_change_queue: RefCell::new(Vec::new()),

            menu_button: gui_handler_create_info.menu_button,
            menu_button_selected: gui_handler_create_info.menu_button_selected,

            frameables: RefCell::new(Vec::new()),
            hoverables: RefCell::new(Vec::new()),
            clickables: RefCell::new(Vec::new()),
            displayables: RefCell::new(Vec::new()),
            selectables: RefCell::new(Vec::new()),
            textables: RefCell::new(Vec::new()),
            writeables: RefCell::new(Vec::new()),
            iconizables: RefCell::new(Vec::new()),

            ortho: Cell::new(ortho(
                0.0,
                render_core.width() as f32,
                0.0,
                render_core.height() as f32,
                -1.0,
                1.0,
            )),

            mouse_x: Cell::new(0),
            mouse_y: Cell::new(0),

            current_clickable: RefCell::new(None),
            current_hoverable: RefCell::new(None),
            current_selectable: RefCell::new(None),
            current_writeable: RefCell::new(None),
        })
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub fn width(&self) -> u32 {
        self.width.get()
    }

    pub fn height(&self) -> u32 {
        self.height.get()
    }

    pub(crate) fn icon_descriptor_layout(&self) -> &Arc<DescriptorSetLayout> {
        &self.icon_descriptor_layout
    }

    pub(crate) fn image_descriptor(&self, path: &str) -> VerboseResult<Arc<DescriptorSet>> {
        if self.internal_textures.try_borrow()?.contains_key(path) {
            Ok(self.internal_textures.try_borrow()?[path]
                ._descriptor_set
                .clone())
        } else {
            let texture = Image::file_source(path)
                .format(VK_FORMAT_R8G8B8A8_UNORM)
                .nearest_sampler()
                .build(&self.device, &self.queue)?;

            let desc_pool = DescriptorPool::new()
                .set_layout(self.text_objects._descriptor_layout.clone())
                .build(self.device.clone())?;

            let descriptor_set = DescriptorPool::prepare_set(&desc_pool).allocate()?;

            descriptor_set.update(&[DescriptorWrite::combined_samplers(0, &[&texture])]);

            let displayable_texture = DisplayableTexture {
                _descriptor_pool: desc_pool,
                _descriptor_set: descriptor_set,

                _texture: texture,
            };

            self.internal_textures
                .borrow_mut()
                .insert(path.to_string(), displayable_texture);

            Ok(self.internal_textures.try_borrow()?[path]
                ._descriptor_set
                .clone())
        }
    }

    pub(crate) fn color_descriptor(&self, color: TextColor) -> VerboseResult<Arc<DescriptorSet>> {
        if self.internal_colors.try_borrow()?.contains_key(&color) {
            Ok(self.internal_colors.try_borrow()?[&color]
                ._descriptor_set
                .clone())
        } else {
            let desc_pool = DescriptorPool::new()
                .set_layout(self.text_color_layout.clone())
                .build(self.device.clone())?;

            let desc_set = DescriptorPool::prepare_set(&desc_pool).allocate()?;

            let vec_color = color.as_vec3();
            let color_slice = [vec_color.x, vec_color.y, vec_color.z];

            let buffer = Buffer::new()
                .set_usage(VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT)
                .set_memory_properties(
                    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
                )
                .set_data(&color_slice)
                .build(self.device.clone())?;

            desc_set.update(&[DescriptorWrite::uniform_buffers(0, &[&buffer])]);

            let textable_color = TextableColor {
                _descriptor_pool: desc_pool,
                _descriptor_set: desc_set,

                _buffer: buffer,
            };

            self.internal_colors
                .borrow_mut()
                .insert(color.clone(), textable_color);

            Ok(self.internal_colors.try_borrow()?[&color]
                ._descriptor_set
                .clone())
        }
    }

    pub(crate) fn ortho(&self) -> cgmath::Matrix4<f32> {
        self.ortho.get()
    }

    pub fn menu_button(&self) -> &String {
        &self.menu_button
    }

    pub fn menu_button_selected(&self) -> &String {
        &self.menu_button_selected
    }

    // ---------------------------------------------------------------------
    // -------------------------  event handling  --------------------------
    // ---------------------------------------------------------------------

    pub(crate) fn set_mouse_pos(&self, x: u32, y: u32) -> VerboseResult<()> {
        self.mouse_x.set(x);
        self.mouse_y.set(y);

        if self.current_hoverable.try_borrow()?.is_some() {
            let mut hoverable = self.current_hoverable.try_borrow_mut()?;

            if hoverable.is_some() {
                // unwrap is safe, just tested for `is_some`
                if !hoverable
                    .as_ref()
                    .unwrap()
                    .is_hovered(self.mouse_x.get(), self.mouse_y.get())
                {
                    hoverable.as_ref().unwrap().set_hovered(false)?;
                    *hoverable = None;
                }
            }
        }

        for hoverable in self.hoverables.try_borrow()?.iter() {
            if hoverable.is_hovered(self.mouse_x.get(), self.mouse_y.get()) {
                hoverable.set_hovered(true)?;
                *self.current_hoverable.try_borrow_mut()? = Some(hoverable.clone());
                break;
            }
        }

        Ok(())
    }

    fn find_clickable(&self) -> VerboseResult<Option<Arc<Clickable>>> {
        for clickable in self.clickables.try_borrow()?.iter() {
            if clickable.is_pressed(self.mouse_x.get(), self.mouse_y.get()) {
                *self.current_clickable.try_borrow_mut()? = Some(clickable.clone());
                return Ok(Some(clickable.clone()));
            }
        }

        Ok(None)
    }

    pub(crate) fn mouse_down(&self, mouse_button: MouseButton) -> VerboseResult<bool> {
        if mouse_button == MouseButton::Left {
            if let Some(tmp_clickable) = self.find_clickable()? {
                tmp_clickable.set_clicked(true)?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub(crate) fn mouse_up(&self, mouse_button: MouseButton) -> VerboseResult<bool> {
        if mouse_button == MouseButton::Left {
            let mut clickable = self.current_clickable.try_borrow_mut()?;
            if clickable.is_some() {
                clickable.as_ref().unwrap().set_clicked(false)?;

                if clickable
                    .as_ref()
                    .unwrap()
                    .is_pressed(self.mouse_x.get(), self.mouse_y.get())
                {
                    if let Some(ref hoverable) = self.current_hoverable.try_borrow()?.as_ref() {
                        hoverable.set_hovered(true)?;
                    }
                }

                *clickable = None;

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn current_selectable(&self) -> VerboseResult<Option<Arc<Selectable>>> {
        match self.current_selectable.try_borrow()?.as_ref() {
            Some(selectable) => Ok(Some(selectable.clone())),
            None => Ok(None),
        }
    }

    pub(crate) fn accept_selection(&self) -> VerboseResult<bool> {
        if let Some(current_selectable) = self.current_selectable()? {
            current_selectable.click_event()?;
            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn decline_topgui(&self) -> VerboseResult<bool> {
        // workaround for unwanted borrowing behaviour inside decline function
        let opt_topgui = {
            match self.top_ui.try_borrow()?.as_ref() {
                Some(top_gui) => Some(top_gui.clone()),
                None => None,
            }
        };

        if let Some(topgui) = opt_topgui {
            topgui.decline()?;
            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn next_tab_topgui(&self) -> VerboseResult<bool> {
        // workaround for unwanted borrowing behaviour inside decline function
        let opt_topgui = {
            match self.top_ui.try_borrow()?.as_ref() {
                Some(top_gui) => Some(top_gui.clone()),
                None => None,
            }
        };

        if let Some(topgui) = opt_topgui {
            topgui.next_tab()?;
            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn previous_tab_topgui(&self) -> VerboseResult<bool> {
        // workaround for unwanted borrowing behaviour inside decline function
        let opt_topgui = {
            match self.top_ui.try_borrow()?.as_ref() {
                Some(top_gui) => Some(top_gui.clone()),
                None => None,
            }
        };

        if let Some(topgui) = opt_topgui {
            topgui.previous_tab()?;
            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn remove_char(&self) -> VerboseResult<bool> {
        match self.current_writeable.try_borrow()?.as_ref() {
            Some(ref current_writable) => {
                current_writable.remove_last()?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub(crate) fn check_navigatable(&self) -> VerboseResult<bool> {
        Ok(self.current_selectable.try_borrow()?.is_some())
    }

    pub(crate) fn update_selection(&self, direction: GuiDirection) -> VerboseResult<bool> {
        match self.current_selectable.try_borrow_mut()?.as_mut() {
            Some(current_selectable) => match direction {
                GuiDirection::Left => {
                    if let Some(neighbour) = current_selectable.west_neighbour()? {
                        current_selectable.set_selected(false)?;
                        *current_selectable = neighbour;
                        current_selectable.set_selected(true)?;
                    };

                    Ok(true)
                }
                GuiDirection::Right => {
                    if let Some(neighbour) = current_selectable.east_neighbour()? {
                        current_selectable.set_selected(false)?;
                        *current_selectable = neighbour;
                        current_selectable.set_selected(true)?;
                    };

                    Ok(true)
                }
                GuiDirection::Up => {
                    if let Some(neighbour) = current_selectable.north_neighbour()? {
                        current_selectable.set_selected(false)?;
                        *current_selectable = neighbour;
                        current_selectable.set_selected(true)?;
                    };

                    Ok(true)
                }
                GuiDirection::Down => {
                    if let Some(neighbour) = current_selectable.south_neighbour()? {
                        current_selectable.set_selected(false)?;
                        *current_selectable = neighbour;
                        current_selectable.set_selected(true)?;
                    };

                    Ok(true)
                }
                GuiDirection::None => Ok(false),
            },
            None => Ok(false),
        }
    }

    pub(crate) fn enqueue_text_update(
        &self,
        function: Box<dyn Fn() -> VerboseResult<()>>,
    ) -> VerboseResult<()> {
        self.text_change_queue.try_borrow_mut()?.push(function);
        self.needs_update.set(true);

        Ok(())
    }

    pub(crate) fn enqueue_resize(&self, width: u32, height: u32) -> VerboseResult<()> {
        self.needs_update.set(true);
        self.ortho
            .set(ortho(0.0, width as f32, 0.0, height as f32, -1.0, 1.0));

        self.width.set(width);
        self.height.set(height);

        for frameable in self.frameables.try_borrow()?.iter() {
            frameable.resize()?;
        }

        Ok(())
    }

    // ---------------------------------------------------------------------
    // ----------------------------  rendering  ----------------------------
    // ---------------------------------------------------------------------
    pub fn set_top_gui(&self, top_gui: Option<Rc<dyn TopGui>>) -> VerboseResult<()> {
        *self.top_ui.try_borrow_mut()? = top_gui;

        Ok(())
    }

    pub(crate) fn render(
        &self,
        eye: Option<Eye>,
        index: usize,
        framebuffer: &Arc<Framebuffer>,
        render_pass: &Arc<RenderPass>,
    ) -> VerboseResult<Arc<CommandBuffer>> {
        if self.needs_update.get() {
            match &self.command_buffers {
                TargetMode::Single(command_buffers) => {
                    for state in command_buffers {
                        state.valid.set(false);
                    }
                }
                TargetMode::Stereo(left_cbs, right_cbs) => {
                    for state in left_cbs {
                        state.valid.set(false);
                    }

                    for state in right_cbs {
                        state.valid.set(false);
                    }
                }
            }

            let mut text_changes = self.text_change_queue.try_borrow_mut()?;

            if !text_changes.is_empty() {
                for text_change in text_changes.iter() {
                    (text_change)()?;
                }

                text_changes.clear();
            }

            self.needs_update.set(false);
        }

        let command_buffer_state = match &self.command_buffers {
            TargetMode::Single(command_buffers) => &command_buffers[index],
            TargetMode::Stereo(left_cbs, right_cbs) => match eye {
                Some(eye) => match eye {
                    Eye::Left => &left_cbs[index],
                    Eye::Right => &right_cbs[index],
                },
                None => create_error!("eye parameter needed for stereo mode"),
            },
        };

        if !command_buffer_state.valid.get() {
            let gui_command_buffer = &command_buffer_state.command_buffer;

            let inheritance_info = CommandBuffer::inheritance_info(
                Some(render_pass),
                Some(0),
                Some(framebuffer),
                None,
            );

            let mut command_buffer_begin_info = VkCommandBufferBeginInfo::new(
                VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT
                    | VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
            );
            command_buffer_begin_info.set_inheritance_info(&inheritance_info);
            gui_command_buffer.begin(command_buffer_begin_info)?;

            let viewport = [VkViewport {
                x: 0.0,
                y: 0.0,
                width: self.width.get() as f32,
                height: self.height.get() as f32,
                minDepth: 0.0,
                maxDepth: 1.0,
            }];

            let scissor = [VkRect2D {
                offset: VkOffset2D { x: 0, y: 0 },
                extent: VkExtent2D {
                    width: self.width.get(),
                    height: self.height.get(),
                },
            }];

            gui_command_buffer.bind_pipeline(&self.rectangle_objects._pipeline)?;

            gui_command_buffer.set_scissor(&scissor);
            gui_command_buffer.set_viewport(&viewport);

            // ---------- render displayables ----------
            for displayable in self.displayables.try_borrow()?.iter() {
                gui_command_buffer.bind_vertex_buffer(displayable.buffer());

                gui_command_buffer
                    .bind_descriptor_sets_minimal(&[&displayable.descriptor_set()?])?;

                gui_command_buffer.draw_complete_single_instance(6);
            }

            // ---------- render iconizables ----------
            for iconizable in self.iconizables.try_borrow()?.iter() {
                gui_command_buffer.bind_vertex_buffer(iconizable.buffer());

                gui_command_buffer.bind_descriptor_sets_minimal(&[&iconizable.descriptor_set()])?;

                gui_command_buffer.draw_complete_single_instance(6);
            }

            gui_command_buffer.bind_pipeline(&self.text_objects._pipeline)?;

            gui_command_buffer.set_scissor(&scissor);
            gui_command_buffer.set_viewport(&viewport);

            let mut text_buffers = command_buffer_state.text_buffers.try_borrow_mut()?;
            text_buffers.clear();

            // ---------- render textables ----------
            for textable in self.textables.try_borrow()?.iter() {
                if let Some(text_buffer) = textable.buffer()? {
                    gui_command_buffer.bind_vertex_buffer(&text_buffer);

                    text_buffers.push(text_buffer);

                    gui_command_buffer.bind_descriptor_sets_minimal(&[
                        &self.bitmap_desc_set,
                        &textable.descriptor_set()?,
                    ])?;

                    gui_command_buffer.draw_complete_single_instance(textable.vertex_count());
                }
            }

            gui_command_buffer.end()?;

            command_buffer_state.valid.set(true);
        }

        Ok(command_buffer_state.command_buffer.clone())
    }
}

// object handling
impl GuiHandler {
    // frameable
    pub(crate) fn add_frameable(&self, frameable: &Arc<Frameable>) -> VerboseResult<()> {
        self.frameables.try_borrow_mut()?.push(frameable.clone());
        Ok(())
    }

    pub(crate) fn delete_frameable(&self, frameable: &Arc<Frameable>) -> VerboseResult<()> {
        let mut frameables = self.frameables.try_borrow_mut()?;
        erase_arc(&mut frameables, frameable);
        Ok(())
    }

    // hoverable
    pub(crate) fn add_hoverable(&self, hoverable: &Arc<Hoverable>) -> VerboseResult<()> {
        self.hoverables.try_borrow_mut()?.push(hoverable.clone());
        Ok(())
    }

    pub(crate) fn delete_hoverable(&self, hoverable: &Arc<Hoverable>) -> VerboseResult<()> {
        let mut current_hoverable = self.current_hoverable.try_borrow_mut()?;
        if current_hoverable.is_some() {
            // unwrap is safe, just tested for `is_some`
            if Arc::ptr_eq(&hoverable, &current_hoverable.as_ref().unwrap()) {
                *current_hoverable = None;
            }
        }

        let mut hoverables = self.hoverables.try_borrow_mut()?;
        erase_arc(&mut hoverables, hoverable);
        Ok(())
    }

    // selectable
    pub(crate) fn add_selectable(&self, selectable: &Arc<Selectable>) -> VerboseResult<()> {
        self.selectables.try_borrow_mut()?.push(selectable.clone());
        Ok(())
    }

    pub(crate) fn delete_selectable(&self, selectable: &Arc<Selectable>) -> VerboseResult<()> {
        let mut current_selectable = self.current_selectable.try_borrow_mut()?;
        if current_selectable.is_some() {
            // unwrap is safe, just tested for `is_some`
            if Arc::ptr_eq(&selectable, &current_selectable.as_ref().unwrap()) {
                *current_selectable = None;
            }
        }

        let mut selectables = self.selectables.try_borrow_mut()?;
        erase_arc(&mut selectables, selectable);
        Ok(())
    }

    // displayable
    pub(crate) fn add_displayable(&self, displayable: &Arc<Displayable>) -> VerboseResult<()> {
        self.displayables
            .try_borrow_mut()?
            .push(displayable.clone());
        self.needs_update.set(true);
        Ok(())
    }

    pub(crate) fn delete_displayable(&self, displayable: &Arc<Displayable>) -> VerboseResult<()> {
        let mut displayables = self.displayables.try_borrow_mut()?;
        erase_arc(&mut displayables, displayable);
        self.needs_update.set(true);
        Ok(())
    }

    // clickable
    pub(crate) fn add_clickable(&self, clickable: &Arc<Clickable>) -> VerboseResult<()> {
        self.clickables.try_borrow_mut()?.push(clickable.clone());
        Ok(())
    }

    pub(crate) fn delete_clickable(&self, clickable: &Arc<Clickable>) -> VerboseResult<()> {
        let mut current_clickable = self.current_clickable.try_borrow_mut()?;
        if current_clickable.is_some() {
            // unwrap is safe, just tested for `is_some`
            if Arc::ptr_eq(&clickable, &current_clickable.as_ref().unwrap()) {
                *current_clickable = None;
            }
        }

        let mut clickables = self.clickables.try_borrow_mut()?;
        erase_arc(&mut clickables, clickable);
        Ok(())
    }

    // textable
    pub(crate) fn add_textable(&self, textable: &Arc<Textable>) -> VerboseResult<()> {
        self.textables.try_borrow_mut()?.push(textable.clone());
        self.needs_update.set(true);
        Ok(())
    }

    pub(crate) fn delete_textable(&self, textable: &Arc<Textable>) -> VerboseResult<()> {
        let mut textables = self.textables.try_borrow_mut()?;
        erase_arc(&mut textables, textable);
        self.needs_update.set(true);
        Ok(())
    }

    // writable
    pub(crate) fn add_writeable(&self, writeable: &Arc<Writeable>) -> VerboseResult<()> {
        self.writeables.try_borrow_mut()?.push(writeable.clone());
        Ok(())
    }

    pub(crate) fn delete_writeable(&self, writeable: &Arc<Writeable>) -> VerboseResult<()> {
        if let Some(write) = self.current_writeable.try_borrow()?.as_ref() {
            if Arc::ptr_eq(&writeable, &write) {
                *self.current_writeable.try_borrow_mut()? = None;
            }
        }

        let mut writeables = self.writeables.try_borrow_mut()?;
        erase_arc(&mut writeables, writeable);

        Ok(())
    }

    // iconizable
    pub(crate) fn add_iconizable(&self, iconizable: &Arc<Iconizable>) -> VerboseResult<()> {
        self.iconizables.try_borrow_mut()?.push(iconizable.clone());
        self.needs_update.set(true);
        Ok(())
    }

    pub(crate) fn delete_iconizable(&self, iconizable: &Arc<Iconizable>) -> VerboseResult<()> {
        let mut iconizables = self.iconizables.try_borrow_mut()?;
        erase_arc(&mut iconizables, iconizable);
        self.needs_update.set(true);
        Ok(())
    }

    pub(crate) fn set_selectable(&self, selectable: Option<Arc<Selectable>>) -> VerboseResult<()> {
        let mut current_selectable = self.current_selectable.try_borrow_mut()?;

        if let Some(selectable) = current_selectable.as_ref() {
            selectable.set_selected(false)?;
        }

        if let Some(selectable) = selectable.as_ref() {
            selectable.set_selected(true)?;
        }

        *current_selectable = selectable;

        Ok(())
    }
}

// private
impl GuiHandler {
    fn create_command_buffers(
        render_core: &Box<dyn RenderCore>,
    ) -> VerboseResult<Vec<CommandBufferState>> {
        let mut command_buffers = Vec::with_capacity(render_core.image_count());

        for _ in 0..render_core.image_count() {
            command_buffers.push(CommandBufferState {
                command_buffer: render_core.allocate_secondary_buffer()?,
                valid: Cell::new(false),
                text_buffers: RefCell::new(Vec::new()),
            });
        }

        Ok(command_buffers)
    }

    fn init_bitmap_font(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        descriptor_layout: Arc<DescriptorSetLayout>,
        path: &str,
    ) -> VerboseResult<(Arc<Image>, Arc<DescriptorPool>, Arc<DescriptorSet>)> {
        let texture = Image::file_source(path)
            .format(VK_FORMAT_R8G8B8A8_UNORM)
            .nearest_sampler()
            .build(device, queue)?;

        let descriptor_pool = DescriptorPool::new()
            .set_layout(descriptor_layout)
            .build(device.clone())?;

        let descriptor_set = DescriptorPool::prepare_set(&descriptor_pool).allocate()?;

        descriptor_set.update(&[DescriptorWrite::combined_samplers(0, &[&texture])]);

        Ok((texture, descriptor_pool, descriptor_set))
    }

    fn init_text_objects(
        device: &Arc<Device>,
        render_pass: &Arc<RenderPass>,
    ) -> VerboseResult<(GuiSeparator, Arc<DescriptorSetLayout>)> {
        // --- layout creation ---
        let descriptor_layout = DescriptorSetLayout::new()
            .add_layout_binding(
                0,
                VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                VK_SHADER_STAGE_FRAGMENT_BIT,
                0,
            )
            .build(device.clone())?;

        let color_layout = DescriptorSetLayout::new()
            .add_layout_binding(
                0,
                VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                VK_SHADER_STAGE_FRAGMENT_BIT,
                0,
            )
            .build(device.clone())?;

        let pipeline_layout = PipelineLayout::new(
            device.clone(),
            &[descriptor_layout.as_ref(), color_layout.as_ref()],
            &[],
        )?;

        // --- pipeline creation ---
        let vertex_shader_text = include_bytes!("guishader/text.vert.spv");
        let fragment_shader_text = include_bytes!("guishader/text.frag.spv");

        let shader_modules = vec![
            ShaderModule::from_slice(device.clone(), vertex_shader_text, ShaderType::Vertex)?,
            ShaderModule::from_slice(device.clone(), fragment_shader_text, ShaderType::Fragment)?,
        ];

        let pipeline = GuiHandler::init_gui_pipeline(
            device,
            render_pass,
            pipeline_layout.clone(),
            shader_modules,
        )?;

        Ok((
            GuiSeparator {
                _descriptor_layout: descriptor_layout,
                _pipeline_layout: pipeline_layout,

                _pipeline: pipeline,
            },
            color_layout,
        ))
    }

    fn init_rectangle_objects(
        device: &Arc<Device>,
        render_pass: &Arc<RenderPass>,
    ) -> VerboseResult<GuiSeparator> {
        let descriptor_layout = DescriptorSetLayout::new()
            .add_layout_binding(
                0,
                VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                VK_SHADER_STAGE_FRAGMENT_BIT,
                0,
            )
            .build(device.clone())?;

        let tmp: &DescriptorSetLayout = &descriptor_layout;

        let pipeline_layout = PipelineLayout::new(device.clone(), &[tmp], &[])?;

        // pipeline creation
        let vertex_shader_text = include_bytes!("guishader/rect.vert.spv");
        let fragment_shader_text = include_bytes!("guishader/rect.frag.spv");

        let shader_modules = vec![
            ShaderModule::from_slice(device.clone(), vertex_shader_text, ShaderType::Vertex)?,
            ShaderModule::from_slice(device.clone(), fragment_shader_text, ShaderType::Fragment)?,
        ];

        let pipeline = GuiHandler::init_gui_pipeline(
            device,
            render_pass,
            pipeline_layout.clone(),
            shader_modules,
        )?;

        Ok(GuiSeparator {
            _descriptor_layout: descriptor_layout,
            _pipeline_layout: pipeline_layout,

            _pipeline: pipeline,
        })
    }

    fn init_gui_pipeline(
        device: &Arc<Device>,
        render_pass: &Arc<RenderPass>,
        pipeline_layout: Arc<PipelineLayout>,
        shaders: Vec<Arc<ShaderModule>>,
    ) -> VerboseResult<Arc<Pipeline>> {
        let stages: Vec<VkPipelineShaderStageCreateInfo> =
            shaders.iter().map(|s| s.pipeline_stage_info()).collect();

        let (input_state, _input_bindings, _input_attributes) =
            TexturedVertex::vertex_input_state();

        let assembly_state = VkPipelineInputAssemblyStateCreateInfo::new(
            VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_NULL_BIT,
            VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            false,
        );

        let dummy_viewport = VkViewport::default();
        let dummy_scissor = VkRect2D::default();

        let viewport_state = VkPipelineViewportStateCreateInfo::new(
            VK_PIPELINE_VIEWPORT_STATE_CREATE_NULL_BIT,
            &[dummy_viewport],
            &[dummy_scissor],
        );

        // init rasterization state
        let rasterization_state = VkPipelineRasterizationStateCreateInfo::new(
            VK_PIPELINE_RASTERIZATION_STATE_CREATE_NULL_BIT,
            false,
            false,
            VK_POLYGON_MODE_FILL,
            VK_CULL_MODE_NONE,
            VK_FRONT_FACE_CLOCKWISE,
            false,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        // init multisample state
        let multisample_state = VkPipelineMultisampleStateCreateInfo::new(
            VK_PIPELINE_MULTISAMPLE_STATE_CREATE_NULL_BIT,
            VK_SAMPLE_COUNT_1_BIT,
            false,
            0.0,
            &[],
            false,
            false,
        );

        // init depth/stencil state
        let stencil_op_state = VkStencilOpState {
            failOp: VK_STENCIL_OP_KEEP,
            passOp: VK_STENCIL_OP_KEEP,
            depthFailOp: VK_STENCIL_OP_KEEP,
            compareOp: VK_COMPARE_OP_ALWAYS,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        };

        let depth_stencil_state = VkPipelineDepthStencilStateCreateInfo::new(
            VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_NULL_BIT,
            false,
            false,
            VK_COMPARE_OP_LESS,
            false,
            false,
            stencil_op_state.clone(),
            stencil_op_state,
            0.0,
            0.0,
        );

        // init color blend state
        let color_blend_attachments = [VkPipelineColorBlendAttachmentState {
            blendEnable: VK_TRUE,
            srcColorBlendFactor: VK_BLEND_FACTOR_SRC_ALPHA,
            dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
            colorBlendOp: VK_BLEND_OP_ADD,
            srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
            dstAlphaBlendFactor: VK_BLEND_FACTOR_ZERO,
            alphaBlendOp: VK_BLEND_OP_ADD,
            colorWriteMask: VK_COLOR_COMPONENT_R_BIT
                | VK_COLOR_COMPONENT_G_BIT
                | VK_COLOR_COMPONENT_B_BIT
                | VK_COLOR_COMPONENT_A_BIT,
        }];
        let color_blend_state = VkPipelineColorBlendStateCreateInfo::new(
            VK_PIPELINE_COLOR_BLEND_STATE_CREATE_NULL_BIT,
            false,
            VK_LOGIC_OP_NO_OP,
            &color_blend_attachments,
            [1.0, 1.0, 1.0, 1.0],
        );

        let dynamic_states = [VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR];
        let dynamic_state = VkPipelineDynamicStateCreateInfo::new(
            VK_PIPELINE_DYNAMIC_STATE_CREATE_NULL_BIT,
            &dynamic_states,
        );

        Pipeline::new_graphics(
            device.clone(),
            None,
            0,
            stages.as_slice(),
            Some(input_state),
            Some(assembly_state),
            None,
            Some(viewport_state),
            rasterization_state,
            Some(multisample_state),
            Some(depth_stencil_state),
            Some(color_blend_state),
            Some(dynamic_state),
            &pipeline_layout,
            render_pass,
            0,
            GraphicsPipelineExtensions::default(),
        )
    }
}
