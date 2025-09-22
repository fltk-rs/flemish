use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum TextEditorCommand<Message>
where
    Message: Clone + Send + Sync + 'static,
{
    Copy,
    Cut,
    Paste,
    BufLen(fn(usize) -> Message),
}

#[derive(Default, Clone, Debug, PartialEq)]
struct DisplayOpts {
    value: String,
    linenumber_width: i32,
    load_path: Option<String>,
}

macro_rules! define_text {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message: 'static + Send + Sync + Clone> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            iprops: DisplayOpts,
            tprops: TextProps,
            #[allow(clippy::type_complexity)]
            change_cb: Option<Rc<Box<dyn Fn(String) -> Message>>>,
            on_command: Option<Rc<Box<dyn Fn(Message) -> Option<TextEditorCommand<Message>>>>>,
        }

        impl<Message: Clone + Send + Sync + 'static> $name<Message> {
            pub fn new(value: &str) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    iprops: DisplayOpts {
                        value: value.to_string(),
                        ..Default::default()
                    },
                    tprops: TextProps::default(),
                    change_cb: None,
                    on_command: None,
                }
            }
            pub fn load_file(mut self, path: &str) -> Self {
                self.iprops.load_path = Some(path.to_string());
                self
            }
            pub fn on_input<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
                self.wprops.when =
                    Some(enums::CallbackTrigger::Changed | enums::CallbackTrigger::EnterKeyAlways);
                self.change_cb = Some(Rc::new(Box::new(f)));
                self
            }
            pub fn on_command<F: 'static + Fn(Message) -> Option<TextEditorCommand<Message>>>(
                mut self,
                f: F,
            ) -> Self {
                self.on_command = Some(Rc::new(Box::new(f)));
                self
            }
            pub fn linenumber_width(mut self, sz: i32) -> Self {
                self.iprops.linenumber_width = sz;
                self
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = text::$name::default();
                let buf = text::TextBuffer::default();
                b.set_buffer(buf);
                if let Some(p) = &self.iprops.load_path {
                    let _ = b.buffer().unwrap().load_file(p);
                } else {
                    b.buffer().unwrap().set_text(&self.iprops.value);
                }
                let change_cb = self.change_cb.clone();
                b.set_callback(move |b| {
                    let v = b.buffer().unwrap().text();
                    if let Some(change_cb) = &change_cb {
                        app::Sender::<Message>::get().send(change_cb(v.clone()));
                    }
                });
                let ed = b.clone();
                default_mount!(b, self, dom, $name, {
                    set_wprops(&mut b, &self.wprops);
                    set_tprops!(b, self.tprops);
                    b.set_linenumber_width(self.iprops.linenumber_width);
                });
                if let Some(ed) = text::TextEditor::from_dyn_widget(&ed) {
                    if let Some(command_handler) = self.on_command.clone() {
                        dom.subscribe_owned(self.node_id, move |msg| {
                            match command_handler(msg.clone()) {
                                Some(TextEditorCommand::Copy) => {
                                    ed.copy();
                                }
                                Some(TextEditorCommand::Cut) => {
                                    ed.cut();
                                }
                                Some(TextEditorCommand::Paste) => {
                                    ed.paste();
                                }
                                Some(TextEditorCommand::BufLen(s)) => {
                                    app::Sender::<Message>::get().send(s(ed
                                        .buffer()
                                        .unwrap()
                                        .text()
                                        .len()));
                                }
                                _ => (),
                            }
                        });
                    }
                }
            }
            fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    update_tprops!(b, self.tprops, old.tprops);
                    let oldi = &old.iprops;
                    let newi = &self.iprops;
                    if oldi.load_path != newi.load_path {
                        if let Some(p) = &newi.load_path {
                            if let Some(mut buf) = b.buffer() {
                                let _ = buf.load_file(p);
                            }
                        }
                    }
                    if oldi.linenumber_width != newi.linenumber_width {
                        b.set_linenumber_width(self.iprops.linenumber_width);
                    }
                });
            }
        }
    };
}

define_text!(TextEditor);
define_text!(TextDisplay);
