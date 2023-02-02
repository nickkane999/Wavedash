#![allow(warnings)]
use druid::{Data, widget::{Button, Flex, Label, Padding, Align, TextBox, LensWrap}, TextAlignment, theme, FontWeight,
 AppLauncher, Env, WindowDesc, Widget, WidgetExt, Lens, FontDescriptor, FontFamily, kurbo::Insets, KeyOrValue};
use std::{fs::File, any::TypeId};
use std::io::prelude::*;
use std::any::Any;
use std::marker::PhantomData;

use self::funny_data_derived_lenses::email_address;

#[derive(Clone, Data, Lens)]
pub struct FunnyData {
    pub num: u32,
    pub email_address: String,
    pub email_subject: String,
    pub email_body: String,
}

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
    pub field_type: String
}

impl InputData {
    pub fn new(data: InputData) -> InputData {
        InputData {
            text: data.text,
            padding: data.padding,
            alignment: data.alignment,
            field_type: data.field_type
        }
    }
    pub fn create_label(&self) {
        TextBox::new().with_placeholder(self.text.clone()).lens(FunnyData::email_address);
    }
    pub fn format_label(&self) -> Padding<FunnyData, Align<FunnyData>> {
        let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_weight(FontWeight::NORMAL)
            .with_size(12.0);
        let applied_font_label = Label::new(format!("{}", self.text)).with_font(font_label.clone());
        let aligned_label = match self.alignment.clone() {
            Some(x) =>  
            if x == "left" { Align::left(applied_font_label) }
            else { Align::centered(applied_font_label) },
            None => Align::centered(applied_font_label),
        };
        let label = Padding::new(self.padding.clone(), aligned_label);
        label
    }
    pub fn create_textbox(&self) -> LensWrap<FunnyData, std::string::String, funny_data_derived_lenses::email_address, TextBox<std::string::String>> {
        TextBox::new().with_placeholder(self.text.clone()).lens(FunnyData::email_address)
    }
    pub fn create_textbox2(&self) -> TextBox<std::string::String> {
        TextBox::new().with_placeholder("test")
    }
    pub fn format_textbox(&self) -> Padding<FunnyData, Align<FunnyData>> {
        //let text_input: TextBox<std::string::String> = self.create_textbox2();
        let text_input = self.create_textbox();
        let aligned_field = match self.alignment.clone() {
            Some(x) =>  
            if x == "left" { Align::left(text_input) }
            else { Align::centered(text_input) },
            None => Align::centered(text_input),
        };
        let field = Padding::new(self.padding.clone(), aligned_field);
        return field;
    }
}


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


// Create label format function with text and padding data type
fn label_format(text: &str, padding: KeyOrValue<druid::Insets>) -> impl Widget<()> {
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    return sample_label;
}