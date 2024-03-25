extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;

use cairo::{Context, Format, ImageSurface};
use gtk::gdk::flush;
use gtk::prelude::*;
use gtk::{Application, Box, Button, DrawingArea, Window, WindowType};

fn main() {
    let appli = Application::new(Some("com.idfk.lol"), Default::default());
    appli.connect_activate(|app| {
        let window = Window::new(WindowType::Toplevel);
        window.set_default_size(1920, 1080);

        let container = Box::new(gtk::Orientation::Vertical, 5);
        window.add(&container);
        let drawbar = Box::new(gtk::Orientation::Horizontal, 5);
        container.add(&drawbar);
        let brush = Button::with_label("Brush");
        drawbar.add(&brush);
        let eraser = Button::with_label("Eraser");
        drawbar.add(&eraser);
        let drawarea = DrawingArea::new();
        drawarea.add_events(
            gtk::gdk::EventMask::BUTTON_PRESS_MASK
                | gtk::gdk::EventMask::BUTTON_RELEASE_MASK
                | gtk::gdk::EventMask::POINTER_MOTION_MASK,
        );

        let surf = ImageSurface::create(Format::ARgb32, 1850, 950).expect("Failed");
        let context = Context::new(&surf);

        drawarea.connect_draw(move |_, c| {
            c.set_source_rgb(1.0, 1.0, 1.0);
            c.paint().expect("Failed");
            c.fill().expect("Failed");
            false.into()
        });

        drawarea.connect_button_press_event({
            move |w, e| {
                let (x, y) = e.position();
                println!("{},{} ", x, y);
                context.set_source_rgb(0.0, 0.0, 0.0);
                context.rectangle(x, y, 150.0, 150.0);
                context.stroke();
                flush();
                context.fill();
                w.queue_draw();
                false.into()
            }
        });

        drawarea.connect_button_release_event(move |w, _| {
            w.queue_draw();
            false.into()
        });

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            false.into()
        });

        container.add(&drawarea);
        container.set_child_packing(&drawarea, true, true, 0, gtk::PackType::Start);
        window.show_all();
        gtk::main();
    });

    appli.run();
}
