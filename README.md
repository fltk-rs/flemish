# Flemish

An elmish architecture for fltk-rs, inspired by Iced.

## Usage
Add flemish to your dependencies:
```toml,ignore
[dependencies]
flemish = "0.5"
```

A usage example:
```rust,no_run
use flemish::{
    button::Button, color_themes, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox, Settings,
};

pub fn main() {
    Counter::new().run(Settings {
        size: (300, 100),
        resizable: true,
        color_map: Some(color_themes::BLACK_THEME),
        ..Default::default()
    })
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - fltk-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let col = Flex::default_fill().column();
        Button::default()
            .with_label("Increment")
            .on_event(|_| Message::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        Button::default()
            .with_label("Decrement")
            .on_event(|_| Message::DecrementPressed);
        col.end();
    }
}
```
## Examples

To run the [examples:](/examples)
```bash
cargo run --example counter
cargo run --example temperature
cargo run --example crud
cargo run --example flglyph
cargo run --example flnetport
cargo run --example flpicture
cargo run --example flresters
...
```

### [FlCounter](/examples/counter.rs)

![FlCounter](/assets/counter.png)

### [FlTemperature](/examples/temperature.rs)

![FlTemperature](/assets/temperature.png)

### [FlCRUD](/examples/crud.rs)

![FlCRUD](/assets/crud.png)

### [FlGlyph](/examples/flglyph.rs)

![FlGlyph](/assets/flglyph.png)

### [FlNetPort](/examples/flnetport.rs)

![FlNetPort](/assets/flnetport.png)

### [FlPicture](/examples/flpicture.rs)

![FlPicture](/assets/flpicture.gif)

## Demos

### [FlCairo](/demos/cairo)

![FlCairo](https://github.com/fltk-rs/demos/blob/master/cairo/assets/scrot.png)

### [FlCalculator](/demos/calculator)

![FlCalculator](https://github.com/fltk-rs/demos/tree/master/flcalculator/assets/flcalculator.gif)

### [FlCSV](/demos/csv)

![FlCSV](https://github.com/fltk-rs/demos/blob/master/csv/assets/csv.gif)

### [FlDialect](/demos/dialect)

![FlDialect](https://github.com/fltk-rs/demos/tree/master/fldialect/assets/fldialect.gif)

### [FlResters](/demos/resters)

![FlResters](https://github.com/fltk-rs/demos/tree/master/flresters/assets/flresters.gif)

### [FlTodo](/demos/fltodo)

![FlTodo](/demos/fltodo/assets/fltodo.gif)

### [Flightbooker](/demos/flightbooker)

![Flightbooker](/demos/flightbooker/assets/flightbooker.png)
