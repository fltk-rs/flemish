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

impl<Message> PartialEq for MenuItem<Message> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.shortcut == other.shortcut && self.flags == other.flags
    }
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
            tprops: TextProps,
            items: Vec<MenuItem<Message>>,
        }

        impl<Message: Clone> $name<Message> {
            pub fn new(items: &[MenuItem<Message>]) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    tprops: TextProps::default(),
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
                    set_tprops!(b, self.tprops);
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
            fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    update_tprops!(b, self.tprops, old.tprops);
                    // Diff menu items to avoid full rebuild when possible
                    if self.items != old.items {
                        // Compute common prefix length
                        let mut prefix = 0usize;
                        let min_len = self.items.len().min(old.items.len());
                        while prefix < min_len && self.items[prefix] == old.items[prefix] {
                            prefix += 1;
                        }

                        // Compute common suffix length, not crossing the prefix
                        let mut suffix = 0usize;
                        let old_len = old.items.len();
                        let new_len = self.items.len();
                        while suffix < (old_len - prefix).min(new_len - prefix)
                            && self.items[new_len - 1 - suffix] == old.items[old_len - 1 - suffix]
                        {
                            suffix += 1;
                        }

                        if prefix == old.items.len() && new_len > old_len {
                            // Pure append
                            for item in &self.items[prefix..] {
                                let sender: app::Sender<Message> = app::Sender::get();
                                b.add_emit(
                                    &item.label,
                                    item.shortcut,
                                    item.flags,
                                    sender,
                                    item.callback.clone(),
                                );
                            }
                        } else if prefix == new_len && old_len > new_len {
                            // Pure truncate
                            for idx in (new_len as i32..old_len as i32).rev() {
                                b.remove(idx);
                            }
                        } else {
                            // Middle-diff: reuse prefix and suffix; rebuild only the middle
                            let old_mid_end = old_len - suffix;
                            let new_mid_end = new_len - suffix;
                            // Remove the old middle segment from end to start so indices stay valid
                            for idx in ((prefix as i32)..(old_mid_end as i32)).rev() {
                                b.remove(idx);
                            }
                            // Append the new middle segment
                            for item in &self.items[prefix..new_mid_end] {
                                let sender: app::Sender<Message> = app::Sender::get();
                                b.add_emit(
                                    &item.label,
                                    item.shortcut,
                                    item.flags,
                                    sender,
                                    item.callback.clone(),
                                );
                            }
                        }
                    }
                });
            }
        }
    };
}

define_menu!(MenuBar);
define_menu!(SysMenuBar);
define_menu!(MenuButton);

#[derive(Clone)]
pub struct Choice<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    tprops: TextProps,
    items: Vec<String>,
    selected_index: i32,
    #[allow(clippy::type_complexity)]
    on_change: Option<std::rc::Rc<Box<dyn Fn(i32) -> Message>>>,
}

impl<Message: Clone> Choice<Message> {
    pub fn new(items: &[&str], selected_index: i32) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Choice,
            wprops: WidgetProps::default(),
            tprops: TextProps::default(),
            items: items.iter().map(|s| s.to_string()).collect(),
            selected_index,
            on_change: None,
        }
    }
    pub fn with_items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }
    pub fn selected_index(mut self, idx: i32) -> Self {
        self.selected_index = idx;
        self
    }
    pub fn on_change<F: 'static + Fn(i32) -> Message>(mut self, f: F) -> Self {
        self.on_change = Some(std::rc::Rc::new(Box::new(f)));
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Choice<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = menu::Choice::default();
        default_mount!(b, self, dom, Choice, {
            set_tprops!(b, self.tprops);
            for it in &self.items {
                b.add_choice(it);
            }
            if self.selected_index >= 0 {
                b.set_value(self.selected_index);
            }
            let on_change = self.on_change.clone();
            b.set_callback(move |b| {
                let idx = b.value();
                if let Some(on_change) = &on_change {
                    app::Sender::<Message>::get().send(on_change(idx));
                }
            });
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Choice, {
            let old: &Choice<Message> = old.as_any().downcast_ref().unwrap();
            update_tprops!(b, self.tprops, old.tprops);
            if self.items != old.items {
                b.clear();
                for it in &self.items {
                    b.add_choice(it);
                }
            }
            if self.selected_index != old.selected_index {
                b.set_value(self.selected_index);
            }
            let on_change = self.on_change.clone();
            b.set_callback(move |b| {
                let idx = b.value();
                if let Some(on_change) = &on_change {
                    app::Sender::<Message>::get().send(on_change(idx));
                }
            });
        });
    }
}
