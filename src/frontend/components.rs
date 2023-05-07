use gtk::{
    Button,
    Orientation, 
    Label,
    DropDown,
    Separator, 
    PolicyType,
    Frame, 
    ScrolledWindow
};

pub fn build_separator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .height_request(2)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_vseparator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Vertical)
        .height_request(2)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_frame () -> Frame {
    Frame::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .hexpand(true)
        .build()
}

pub fn build_scrollwindow() -> ScrolledWindow {
    ScrolledWindow::builder()
        .margin_start(12)
        .margin_end(12)
        .vscrollbar_policy(PolicyType::Automatic)
        .build()
}


pub fn build_label(title: &str) -> Label {
    Label::builder()
        .label(title)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_clean_label(title: &str) -> Label {
    Label::builder()
        .label(title)
        .margin_top(8)
        .margin_bottom(8)
        .build()
}

pub fn build_button(title: &str) -> Button {
    Button::builder()
        .label(title)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_dropdown() -> DropDown {
    DropDown::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

/* 
pub fn build_svg_image(path:: Path) -> Picture{

    let content_bytes = include_bytes!(path);
    let loader = PixbufLoader::new();

    loader.write(content_bytes).unwrap();
    loader.close().unwrap();

    let pixbuf = loader.pixbuf().unwrap();

    let picture = Picture::new();
    picture.set_pixbuf(Some(&pixbuf));
    picture
}
*/