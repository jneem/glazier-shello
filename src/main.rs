mod test_scene;

use glazier::kurbo::Size;
use glazier::{
    Application, Cursor, FileDialogToken, FileInfo, IdleToken, KeyEvent, MouseEvent, Region,
    TimerToken, WinHandler, WindowHandle,
};
use parley::FontContext;
use piet_scene::Scene;
use piet_wgsl::util::{RenderContext, RenderSurface};
use piet_wgsl::Renderer;
use std::any::Any;

const WIDTH: usize = 2048;
const HEIGHT: usize = 1536;

fn main() {
    let app = Application::new().unwrap();
    let mut window_builder = glazier::WindowBuilder::new(app.clone());
    window_builder.resizable(false);
    window_builder.set_size((WIDTH as f64 / 2., HEIGHT as f64 / 2.).into());
    window_builder.set_handler(Box::new(WindowState::new()));
    let window_handle = window_builder.build().unwrap();
    window_handle.show();
    app.run(None);
}

struct WindowState {
    handle: WindowHandle,
    render: RenderContext,
    surface: Option<RenderSurface>,
    scene: Scene,
    size: Size,
    font_context: FontContext,
    counter: u64,
}

impl WindowState {
    pub fn new() -> Self {
        let render = pollster::block_on(RenderContext::new()).unwrap();
        Self {
            handle: Default::default(),
            surface: None,
            render,
            scene: Default::default(),
            font_context: FontContext::new(),
            counter: 0,
            size: Size::new(800.0, 600.0),
        }
    }

    #[cfg(target_os = "macos")]
    fn schedule_render(&self) {
        self.handle
            .get_idle_handle()
            .unwrap()
            .schedule_idle(IdleToken::new(0));
    }

    #[cfg(not(target_os = "macos"))]
    fn schedule_render(&self) {
        self.handle.invalidate();
    }

    fn new_surface(&mut self) {
        self.surface = Some(self.render.create_surface(
            &self.handle,
            self.size.width as u32,
            self.size.height as u32,
        ));
    }

    fn render(&mut self) {
        let width = self.size.width as u32;
        let height = self.size.height as u32;
        if self.surface.is_none() {
            self.new_surface();
        }

        let mut renderer = Renderer::new(&self.render.device).unwrap();
        test_scene::render_anim_frame(&mut self.scene, &mut self.font_context, self.counter);
        self.counter += 1;

        let surface_texture = self
            .surface
            .as_ref()
            .unwrap()
            .surface
            .get_current_texture()
            .unwrap();
        renderer
            .render_to_surface(
                &self.render.device,
                &self.render.queue,
                &self.scene,
                &surface_texture,
                width,
                height,
            )
            .unwrap();
        surface_texture.present();
    }
}

impl WinHandler for WindowState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
        self.schedule_render();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, _: &Region) {
        self.render();
        self.schedule_render();
    }

    fn idle(&mut self, _: IdleToken) {
        self.render();
        self.schedule_render();
    }

    fn command(&mut self, _id: u32) {}

    fn open_file(&mut self, _token: FileDialogToken, file_info: Option<FileInfo>) {
        println!("open file result: {:?}", file_info);
    }

    fn save_as(&mut self, _token: FileDialogToken, file: Option<FileInfo>) {
        println!("save file result: {:?}", file);
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        println!("mouse_wheel {:?}", event);
    }

    fn mouse_move(&mut self, _event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        //println!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        println!("mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        println!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken) {
        println!("timer fired: {:?}", id);
    }

    fn size(&mut self, size: Size) {
        self.size = size;
        self.new_surface();
    }

    fn got_focus(&mut self) {
        println!("Got focus");
    }

    fn lost_focus(&mut self) {
        println!("Lost focus");
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
