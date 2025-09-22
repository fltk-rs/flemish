use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};

#[derive(Clone)]
pub struct Window<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
    on_close: Option<Message>,
}

impl<Message: Clone> Window<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Window,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
            on_close: None,
        }
    }
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Window<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut w = window::Window::default();
        default_mount!(w, self, dom, Window, {
            w.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            w.end();
            if let Some(msg) = self.on_close.clone() {
                w.set_callback(move |_| {
                    app::Sender::<Message>::get().send(msg.clone());
                });
            }
            w.show();
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Window, {
            let old: &Window<Message> = old.as_any().downcast_ref().unwrap();
            if self.on_close.is_some() != old.on_close.is_some() {
                let msg = self.on_close.clone();
                b.set_callback(move |_| {
                    if let Some(msg) = &msg {
                        app::Sender::<Message>::get().send(msg.clone());
                    }
                });
            }
        });
        // manual child diff similar to update_group_children!
        let old_id = old.node_id();
        let oldg = old.gprops().unwrap();
        let newg = &mut self.gprops;
        let min_len = oldg.children.len().min(newg.children.len());
        for i in 0..min_len {
            newg.children[i].patch(&mut oldg.children[i], dom);
        }
        if newg.children.len() > oldg.children.len() {
            // Clone the window handle to avoid holding the map borrow while mounting
            let w_opt = {
                let mut map = dom.widget_map.borrow_mut();
                if let Some(WidgetUnion::Window(ref mut w)) = map.get_mut(&old_id) {
                    Some(w.clone())
                } else {
                    None
                }
            };
            if let Some(w) = w_opt {
                w.begin();
                for i in oldg.children.len()..newg.children.len() {
                    newg.children[i].mount(dom);
                }
                w.end();
            }
        }
        if oldg.children.len() > newg.children.len() {
            for i in newg.children.len()..oldg.children.len() {
                crate::utils::subtree::remove_subtree(&mut oldg.children[i], dom);
            }
        }
    }
}

// Note: SubWindow wrapper omitted due to FLTK crate type availability; Window covers common needs.
