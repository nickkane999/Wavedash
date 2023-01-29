use druid::{Data, widget::{Button, Flex, Label}, AppLauncher, Env, WindowDesc, Widget, WidgetExt};

#[derive(Clone, Data)]
struct FunnyData {
    num: u32
}

fn ui_builder() -> impl Widget<FunnyData> {
    let label = Label::new(|data: &FunnyData, _: &Env| format!("Counter: {}", data.num));
    let increment = Button::new("+").on_click(|ctx, data: &mut FunnyData, _: &Env| data.num += 1);
    let decrement = Button::new("-").on_click(|ctx, data: &mut FunnyData, _: &Env| data.num -= 1);

    Flex::column().with_child(label).with_child(
        Flex::row()
            .with_child(increment)
            .with_child(decrement)
    )
}

fn main() {
    let main_window = WindowDesc::new(ui_builder()).title("Wavedash - Email Builder");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(FunnyData { num: 0 })
        .expect("launch failed");
}
