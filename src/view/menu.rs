use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
pub use fltk::menu::MenuFlag;
use fltk::{prelude::*, *};

#[derive(Debug, Clone)]
pub struct MenuItem<Message> {
    label: String,
    shortcut: enums::Shortcut,
    flags: menu::MenuFlag,
    callback: Message,
}

impl<Message> MenuItem<Message> {
    pub fn new(
        label: &str,
        shortcut: enums::Shortcut,
        flags: menu::MenuFlag,
        callback: Message,
    ) -> Self {
        Self {
            label: label.to_string(),
            shortcut,
            flags,
            callback,
        }
    }
}

macro_rules! define_menu {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            items: Vec<MenuItem<Message>>,
        }

        impl<Message: Clone> $name<Message> {
            pub fn new(items: &[MenuItem<Message>]) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    items: items.to_vec(),
                }
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = menu::$name::default();
                default_mount!(b, self, dom, $name, {
                    for item in &self.items {
                        let sender: app::Sender<Message> = app::Sender::get();
                        b.add_emit(
                            &item.label,
                            item.shortcut,
                            item.flags,
                            sender,
                            item.callback.clone(),
                        );
                    }
                });
            }
            fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    // TODO: check if actually things changed!
                    b.clear();
                    for item in &self.items {
                        let sender: app::Sender<Message> = app::Sender::get();
                        b.add_emit(
                            &item.label,
                            item.shortcut,
                            item.flags,
                            sender,
                            item.callback.clone(),
                        );
                    }
                });
            }
        }
    };
}

define_menu!(MenuBar);
define_menu!(SysMenuBar);
define_menu!(Choice);
