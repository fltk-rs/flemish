use flemish::{
    dialog,
    enums::{Align, Shortcut},
    theme::color_themes,
    view::*,
    Settings, Task,
};
use std::path::PathBuf;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    flemish::application("editor", Editor::update, Editor::view)
        .settings(Settings {
            size: (800, 600),
            resizable: true,
            color_map: color_themes::TAN_THEME,
            ignore_esc_close: true,
            on_close: Some(Message::Quit),
            ..Default::default()
        })
        .run_with(move || {
            if let Some(p) = args.get(1) {
                Editor::new(p.into()).unwrap()
            } else {
                Editor::default()
            }
        });
}

struct Editor {
    path: PathBuf,
    content: String,
    saved: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            content: String::new(),
            saved: true,
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Changed(String),
    FileSave,
    Quit,
    TextEditorCommand(TextEditorCommand<Message>),
}

impl Editor {
    fn new(path: PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        let saved = true;
        Ok(Self {
            path,
            content,
            saved,
        })
    }
    fn update(&mut self, message: Message) -> Result<Task<Message>, Box<dyn std::error::Error>> {
        match message {
            Message::Changed(s) => {
                self.saved = false;
                self.content = s;
            }
            Message::FileSave => {
                std::fs::write(&self.path, &self.content)?;
                self.saved = true;
            }
            Message::Quit => {
                if self.saved {
                    return Ok(Task::exit());
                } else if let Some(choice) = dialog::choice_default(
                    "You have unsaved changes, are you sure you want to exit?",
                    "Yes",
                    "No",
                    "",
                ) {
                    if choice == 0 {
                        return Ok(Task::exit());
                    }
                }
            }
            _ => {}
        }
        Ok(Task::none())
    }

    fn view(&self) -> View<Message> {
        Column::new(&[
            MenuBar::new(&[
                MenuItem::new(
                    "&File/&Save\t",
                    Shortcut::Ctrl | 's',
                    MenuFlag::MenuDivider,
                    Message::FileSave,
                ),
                MenuItem::new(
                    "&File/&Quit\t",
                    Shortcut::Ctrl | 'q',
                    MenuFlag::Normal,
                    Message::Quit,
                ),
                MenuItem::new(
                    "&Edit/Cu&t\t",
                    Shortcut::Ctrl | 'x',
                    MenuFlag::Normal,
                    Message::TextEditorCommand(TextEditorCommand::Cut),
                ),
                MenuItem::new(
                    "&Edit/&Copy\t",
                    Shortcut::Ctrl | 'c',
                    MenuFlag::Normal,
                    Message::TextEditorCommand(TextEditorCommand::Copy),
                ),
                MenuItem::new(
                    "&Edit/&Paste\t",
                    Shortcut::Ctrl | 'v',
                    MenuFlag::Normal,
                    Message::TextEditorCommand(TextEditorCommand::Paste),
                ),
            ])
            .fixed(30)
            .view(),
            TextEditor::new(&self.content.to_string())
                .linenumber_width(40)
                .on_input(Message::Changed)
                .on_command(|cmd| match cmd {
                    Message::TextEditorCommand(c) => Some(c),
                    _ => None,
                })
                .view(),
            Frame::new(if self.saved { "" } else { "Not saved" })
                .align(Align::Left | Align::Inside)
                .fixed(20)
                .view(),
        ])
        .view()
    }
}
