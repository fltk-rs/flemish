use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::rc::Rc;

#[derive(Default, Clone, Debug, PartialEq)]
struct ButtonProps {
    value: bool,
    down_box: Option<enums::FrameType>,
    shortcut: Option<enums::Shortcut>,
}

fn set_bprops<W>(w: &mut W, wprops: &ButtonProps)
where
    W: ButtonExt + 'static,
{
    if let Some(b) = &wprops.down_box {
        w.set_down_frame(*b);
    }
    if let Some(b) = &wprops.shortcut {
        w.set_shortcut(*b);
    }
}

fn update_bprops<W>(w: &mut W, old_wprops: &ButtonProps, new_wprops: &ButtonProps)
where
    W: ButtonExt + 'static,
{
    if old_wprops.down_box != new_wprops.down_box {
        if let Some(c) = &new_wprops.down_box {
            w.set_down_frame(*c);
        }
    }
    if old_wprops.shortcut != new_wprops.shortcut {
        if let Some(c) = &new_wprops.shortcut {
            w.set_shortcut(*c);
        }
    }
}

#[derive(Clone)]
pub struct Button<Message> {
    node_id: usize,
    typ: VNodeType,
    callback: Message,
    wprops: WidgetProps,
    bprops: ButtonProps,
}

impl<Message> Button<Message> {
    pub fn new(label: &str, callback: Message) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Button,
            callback,
            wprops: WidgetProps {
                label: Some(label.to_string()),
                ..Default::default()
            },
            bprops: ButtonProps::default(),
        }
    }
    pub fn shortcut(mut self, s: enums::Shortcut) -> Self {
        self.bprops.shortcut = Some(s);
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Button<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = button::Button::default();
        let sender: app::Sender<Message> = app::Sender::get();
        b.emit(sender, self.callback.clone());
        default_mount!(b, self, dom, Button, {
            set_bprops(&mut b, &self.bprops);
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Button, {
            let old: &Button<Message> = old.as_any().downcast_ref().unwrap();
            let sender: app::Sender<Message> = app::Sender::get();
            b.emit(sender, self.callback.clone());
            update_bprops(b, &old.bprops, &self.bprops);
        });
    }
}

macro_rules! define_button {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            bprops: ButtonProps,
            #[allow(clippy::type_complexity)]
            on_change: Option<Rc<Box<dyn Fn(bool) -> Message>>>,
        }

        impl<Message> $name<Message> {
            pub fn new(label: &str, value: bool) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps {
                        label: Some(label.to_string()),
                        ..Default::default()
                    },
                    bprops: ButtonProps {
                        value,
                        down_box: None,
                        shortcut: None,
                    },
                    on_change: None,
                }
            }
            pub fn shortcut(mut self, s: enums::Shortcut) -> Self {
                self.bprops.shortcut = Some(s);
                self
            }
            pub fn on_change<F: 'static + Fn(bool) -> Message>(mut self, f: F) -> Self {
                self.on_change = Some(Rc::new(Box::new(f)));
                self
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = button::$name::default();
                default_mount!(b, self, dom, $name, {
                    set_bprops(&mut b, &self.bprops);
                    b.set_value(self.bprops.value);
                    let on_change = self.on_change.clone();
                    b.set_callback(move |b| {
                        let v = b.value();
                        if let Some(on_change) = &on_change {
                            app::Sender::<Message>::get().send(on_change(v));
                        }
                    });
                });
            }
            fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    update_bprops(b, &old.bprops, &self.bprops);
                    let oldi = &old.bprops;
                    let newi = &self.bprops;
                    if oldi.value != newi.value {
                        b.set_value(newi.value);
                        let on_change = self.on_change.clone();
                        b.set_callback(move |b| {
                            let v = b.value();
                            if let Some(on_change) = &on_change {
                                app::Sender::<Message>::get().send(on_change(v));
                            }
                        });
                    }
                });
            }
        }
    };
}

define_button!(CheckButton);
define_button!(RadioButton);
define_button!(ToggleButton);
define_button!(RoundButton);
define_button!(LightButton);
define_button!(RepeatButton);
define_button!(RadioLightButton);
define_button!(RadioRoundButton);
define_button!(ReturnButton);
