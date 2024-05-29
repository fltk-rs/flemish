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
cargo run --example flcalculator
cargo run --example fldialect
cargo run --example flnetport
cargo run --example flpicture
...
```

### [FlCounter](/examples/counter.rs)

![FlCalculator](/assets/counter.png)

### [FlTemperature](/examples/temperature.rs)

![FlTemperature](/assets/temperature.png)

### [FlCalculator](/examples/flcalculator.rs)

![FlCalculator](/assets/flcalculator.gif)

### [FlDialect](/examples/fldialect.rs)

![FlDialect](/assets/fldialect.gif)

### [FlNetPort](/examples/flnetport.rs)

![FlNetPort](/assets/flnetport.png)

### [FlPicture](/examples/flpicture.rs)

![FlPicture](/assets/flpicture.gif)

## Demos

### [FlErrands](/demos/flerrands)

![FlPicture](/demos/flerrands/assets/flerrands.gif)

### [FlCSV](/demos/csv)

![FlCSV](/demos/csv/assets/flcsv.png)

### [FlCairo](/demos/cairo)

![FlCairo](/demos/cairo/assets/flcairo.png)
