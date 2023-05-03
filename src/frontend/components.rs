use gtk::{
    Button,
    Orientation, 
    Label,
    DropDown,
    Separator
};

pub fn build_separator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .height_request(2)
        .margin_start(12)
        .margin_end(12)
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