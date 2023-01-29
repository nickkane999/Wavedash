use druid::{Data, widget::{Button, Flex, Label, Padding, Align, TextBox}, TextAlignment, theme, FontWeight,
 AppLauncher, Env, WindowDesc, Widget, WidgetExt, Lens, FontDescriptor, FontFamily, kurbo::Insets};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Data, Lens)]
struct FunnyData {
    num: u32,
    email_address: String,
    email_subject: String,
    email_body: String,
}

fn ui_builder() -> impl Widget<FunnyData> {
    //const FONT_SIZE : Key<f64> = Key::new("org.linebender.druid.theme.text_size_large");
    //println!("My font size is: {:?}", theme::TEXT_SIZE_LARGE);

    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(24.0);

    let count_label = Label::new(|data: &FunnyData, _: &Env| format!("Counter: {}", data.num));
    let increment = Button::new("+").on_click(|ctx, data: &mut FunnyData, _: &Env| data.num += 1);
    let decrement = Button::new("-").on_click(|ctx, data: &mut FunnyData, _: &Env| data.num -= 1);

    // Width padding -> (left, top, right, bottom)
    let email_address_label = Padding::new((0.0, 20.0, 0.0, 10.0), 
    Align::left(Label::new(|data: &FunnyData, _: &Env| format!("Email Address: "))
            .with_font(font_label.clone())));
    let email_address = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Address").lens(FunnyData::email_address));
    let email_subject_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Subject: "))
            .with_font(font_label.clone()));
    let email_subject = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Subject")
            .lens(FunnyData::email_subject));
    let email_body_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Body: "))
            .with_font(font_label.clone()));
    let email_body = Padding::new((0.0, 0.0, 00.0, 20.0),
        TextBox::multiline()
            .with_line_wrapping(true)
            .with_text_alignment(TextAlignment::Start)
            .with_text_size(theme::TEXT_SIZE_NORMAL)
            .with_placeholder("Email Body")
            .lens(FunnyData::email_body));


    let save = Button::new("Save").on_click(|ctx, data: &mut FunnyData, _: &Env| save_data(&data));

    Flex::column().with_child(
        Flex::column()
            .with_child(email_address_label)
            .with_child(email_address)
    ).with_child(
        Flex::column()
            .with_child(email_subject_label)
            .with_child(email_subject)
    ).with_child(
        Flex::column()
            .with_child(email_body_label)
            .with_child(email_body)
    ).with_child(
        Flex::row()
            .with_child(save)
    )
}

fn save_data(data: &FunnyData) {
    let data_string = format!("{}\n{}\n{}\n", data.email_address, data.email_subject, data.email_body);

    // Write command to command line 
    let mut file = File::create("emails/data.txt").expect("Couldn't create file");
    file.write_all(data_string.as_bytes()).expect("Couldn't write to file");
}

fn main() {
    let main_window = WindowDesc::new(ui_builder()).title("Wavedash - Email Builder");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(FunnyData { num: 0, email_address: "".to_string(), email_subject: "".to_string(), email_body: "".to_string() })
        .expect("launch failed");
}
