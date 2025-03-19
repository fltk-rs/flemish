use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::rc::Rc;

macro_rules! define_browser {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            items: Vec<String>,
            selection: i32,
            #[allow(clippy::type_complexity)]
            on_change: Option<Rc<Box<dyn Fn(i32) -> Message>>>,
        }

        impl<Message: Clone> $name<Message> {
            pub fn new(items: &[&str], selection: i32) -> Self {
                let items = items.iter().map(|s| s.to_string()).collect();
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    items,
                    selection,
                    on_change: None,
                }
            }
            pub fn on_change<F: 'static + Fn(i32) -> Message>(mut self, f: F) -> Self {
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
                let mut b = browser::$name::default();
                default_mount!(b, self, dom, $name, {
                    let on_change = self.on_change.clone();
                    for item in &self.items {
                        b.add(item);
                    }
                    b.select(self.selection);
                    b.set_callback(move |b| {
                        let v = b.value();
                        if let Some(on_change) = &on_change {
                            app::Sender::<Message>::get().send(on_change(v));
                        }
                    });
                });
            }
            fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    if self.items != old.items {
                        b.clear();
                        for item in &self.items {
                            b.add(item);
                        }
                        b.select(self.selection);
                    }
                    if self.selection != old.selection {
                        b.select(self.selection);
                    }
                    let on_change = self.on_change.clone();
                    b.set_callback(move |b| {
                        let v = b.value();
                        if let Some(on_change) = &on_change {
                            app::Sender::<Message>::get().send(on_change(v));
                        }
                    });
                });
            }
        }
    };
}

define_browser!(Browser);
define_browser!(SelectBrowser);
define_browser!(HoldBrowser);
define_browser!(MultiBrowser);
define_browser!(FileBrowser);
// define_browser!(CheckBrowser);
