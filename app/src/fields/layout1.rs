mod components2;
use components2::{LabelData, InputData, FunnyData};

use druid::{Data, widget::{Button, Flex, Label, Padding, Align, TextBox, LensWrap}, TextAlignment, theme, FontWeight,
 AppLauncher, Env, WindowDesc, Widget, WidgetExt, Lens, FontDescriptor, FontFamily, kurbo::Insets, KeyOrValue};
use std::fs::File;
use std::io::prelude::*;
use std::any::Any;

pub 