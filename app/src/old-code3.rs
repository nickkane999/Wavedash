/*
use druid::{Data, widget::{Button, Flex, Label, Padding, Align, TextBox, LensWrap}, TextAlignment, theme, FontWeight,
 AppLauncher, Env, WindowDesc, Widget, WidgetExt, Lens, FontDescriptor, FontFamily, kurbo::Insets, KeyOrValue};
use std::{fs::File, any::TypeId};
use std::io::prelude::*;
use std::any::Any;

#[derive(Clone, Data, Lens)]
pub struct FunnyData {
    pub num: u32,
    pub email_address: String,
    pub email_subject: String,
    pub email_body: String,
    pub content: FunnyDataOptions,
}

/*
pub struct FunnyDataField<T> {
    content: T,
}

impl <T> FunnyDataField<T> {
    pub fn get_email_address<T> (self, T) -> T {
        return self.content;
    }
}
*/



trait FunnyDataField {}
impl FunnyDataField for funny_data_derived_lenses::email_address {}
impl FunnyDataField for funny_data_derived_lenses::email_subject {}
impl FunnyDataField for funny_data_derived_lenses::email_body {}


pub struct LabelData {
    pub text: String,
    pub font_size: f64,
    pub font_weight: FontWeight,
    pub padding: KeyOrValue<druid::Insets>,
    pub alignment: Option<String>,
}
pub struct InputData {
    pub text: String,
    pub padding: KeyOrValue<druid::Insets>,
    pub alignment: Option<String>,
    pub field: FunnyDataOptions,
}

use funny_data_derived_lenses::email_address as email_address_value;
use funny_data_derived_lenses::email_subject as email_subject_value;
use funny_data_derived_lenses::email_body as email_body_value;

impl FunnyData {
    fn add_content(&self) {
        self.content = email_address_value::new(); 
    }
    fn get_email_address(&self, field: InputData) -> LensWrap<FunnyData, String, email_address_value, TextBox<String>> {
        return TextBox::new().with_placeholder(field.text).lens(email_address_value::new());
    }
    fn get_email_subject(&self, field: InputData) -> LensWrap<FunnyData, String, email_subject_value, TextBox<String>> {
        return TextBox::new().with_placeholder(field.text).lens(email_subject_value::new());
    }
    fn get_email_body(&self, field: InputData) -> LensWrap<FunnyData, String, email_body_value, TextBox<String>> {
        return TextBox::new().with_placeholder(field.text).lens(email_body_value::new());
    }
    fn get_field(&self, field: InputData) {
        match field {
            FunnyDataOptions::email_address => self.get_email_address(field),
            FunnyDataOptions::email_subject => self.email_subject,
            FunnyDataOptions::email_body => self.email_body,
        }
    }
}

enum FunnyDataOptions {
    email_address,
    email_subject,
    email_body,
}
/*
pub struct InputData<T> {
    pub text: String,
    pub padding: KeyOrValue<druid::Insets>,
    pub alignment: Option<String>,
    pub field: T,
}
impl<T> InputData<T> {
    pub fn get_field(&self) -> T {
        return self.field;
    }
}
*/


pub fn new_label(label: LabelData) -> Padding<FunnyData, Align<FunnyData>> {
    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(label.font_weight)
        .with_size(label.font_size);
    let applied_font_label = Label::new(format!("{}", label.text)).with_font(font_label.clone());
    let aligned_label = match label.alignment {
        Some(x) =>  
        if x == "left" { Align::left(applied_font_label) }
        else { Align::centered(applied_font_label) },
        None => Align::centered(applied_font_label),
    };
    let label = Padding::new(label.padding, aligned_label);
    return label;
}

pub fn new_field(field: InputData) -> Padding<FunnyData, Align<FunnyData>> {
    //let text_input: LensWrap<FunnyData, String, FunnyDataOptions, TextBox<String>>  = match field.field {
    let text_input: LensWrap<FunnyData, String, email_address_value, TextBox<String>>  = match field.field {
            FunnyDataOptions::email_address => TextBox::new().with_placeholder(field.text).lens(email_address_value::new()),
        FunnyDataOptions::email_subject => TextBox::new().with_placeholder(field.text).lens(email_subject_value::new()),
        FunnyDataOptions::email_body => TextBox::new().with_placeholder(field.text).lens(FunnyData::email_body),
    };
    /*
    let email_address_key = String::from("email_address");
    let email_subject_key = String::from("email_subject");
    let email_body_key = String::from("email_body");

    let funny_data_value: Box<dyn FunnyDataField> = match field.field {
        email_address_key => Box::new(FunnyData::email_address),
        email_subject_key => Box::new(FunnyData::email_subject),
        email_body_key => Box::new(FunnyData::email_body),
        _ => Box::new(FunnyData::email_body),
    };
    */
    //let text_input = TextBox::new().with_placeholder(field.text).lens(FunnyData::email_address);
    let text_input = TextBox::new().with_placeholder(field.text).lens(field.field);
    let aligned_field = match field.alignment {
        Some(x) =>  
        if x == "left" { Align::left(text_input) }
        else { Align::centered(text_input) },
        None => Align::centered(text_input),
    };
    let field = Padding::new(field.padding, aligned_field);
    return field;
}

// Create label format function with text and padding data type
fn label_format(text: &str, padding: KeyOrValue<druid::Insets>) -> impl Widget<()> {
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    return sample_label;
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

    /*
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    
    */
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    //let sample_label = label_format("Sample Label", (0.0, 0.0, 0.0, 60.0).into());

    // Width padding -> (left, top, right, bottom)
    /* 

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

    
    */
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
            .with_child(sample_label)
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
*/