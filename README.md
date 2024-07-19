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
...
```

### [Counter](/examples/counter.rs)

![img](/assets/counter.png)

### [Temperature](/examples/temperature.rs)

![img](/assets/temperature.png)

### [CRUD](/examples/crud.rs)

![img](/assets/crud.png)

### [Glyph](/examples/flglyph.rs)

![img](/assets/flglyph.png)

### [NetPort](/examples/flnetport.rs)

![img](/assets/flnetport.png)

### [Picture](/examples/flpicture.rs)

![img](/assets/flpicture.gif)

## Demos

### [Cairo](/demos/cairo)

![img](../demos/blob/master/cairo/assets/scrot.png)

### [Calculator](/demos/calculator)

![img](../demos/blob/master/flcalculator/assets/flcalculator.gif)

### [CSV](/demos/csv)

![img](../demos/blob/master/csv/assets/csv.gif)

### [Dialect](/demos/dialect)

![img](../demos/blob/master/fldialect/assets/fldialect.gif)

### [Resters](/demos/resters)

![img](../demos/blob/master/flresters/assets/flresters.gif)

### [Todo](/demos/fltodo)

![img](/demos/fltodo/assets/fltodo.gif)

### [Flightbooker](/demos/flightbooker)

![img](/demos/flightbooker/assets/flightbooker.png)
