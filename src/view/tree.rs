use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::prelude::*;
use fltk::tree;
use fltk::tree::TreeReason;
use fltk::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeItem {
    pub label: String,
}

impl TreeItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Tree<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    items: Vec<TreeItem>,
    selected: Option<String>,
    #[allow(clippy::type_complexity)]
    on_select: Option<std::rc::Rc<Box<dyn Fn(String) -> Message>>>,
    #[allow(clippy::type_complexity)]
    on_open: Option<std::rc::Rc<Box<dyn Fn(String) -> Message>>>,
    #[allow(clippy::type_complexity)]
    on_close: Option<std::rc::Rc<Box<dyn Fn(String) -> Message>>>,
    opened: Vec<String>,
    phantom: PhantomData<Message>,
}

impl<Message: Clone> Tree<Message> {
    pub fn new(items: &[TreeItem]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Tree,
            wprops: WidgetProps::default(),
            items: items.to_vec(),
            selected: None,
            on_select: None,
            on_open: None,
            on_close: None,
            opened: vec![],
            phantom: PhantomData,
        }
    }
    pub fn on_select<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
        self.on_select = Some(std::rc::Rc::new(Box::new(f)));
        self
    }
    pub fn on_open<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
        self.on_open = Some(std::rc::Rc::new(Box::new(f)));
        self
    }
    pub fn on_close<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
        self.on_close = Some(std::rc::Rc::new(Box::new(f)));
        self
    }
    pub fn selected(mut self, path: &str) -> Self {
        self.selected = Some(path.to_string());
        self
    }
    pub fn open(mut self, path: &str) -> Self {
        self.opened.push(path.to_string());
        self
    }
    pub fn opened(mut self, paths: &[&str]) -> Self {
        self.opened = paths.iter().map(|s| s.to_string()).collect();
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Tree<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = tree::Tree::default();
        default_mount!(b, self, dom, Tree, {
            for item in &self.items {
                b.add(&item.label);
            }
            if let Some(path) = &self.selected {
                let _ = b.select(path, true);
            }
            if !self.opened.is_empty() {
                for p in &self.opened {
                    let _ = b.open(p, true);
                }
            }
            let on_select = self.on_select.clone();
            let on_open = self.on_open.clone();
            let on_close = self.on_close.clone();
            if on_select.is_some() || on_open.is_some() || on_close.is_some() {
                b.set_callback(move |t| match t.callback_reason() {
                    TreeReason::Opened => {
                        if let Some(cb) = &on_open {
                            if let Some(it) = t.callback_item() {
                                if let Some(lbl) = it.label() {
                                    app::Sender::<Message>::get().send(cb(lbl.to_string()));
                                }
                            }
                        }
                    }
                    TreeReason::Closed => {
                        if let Some(cb) = &on_close {
                            if let Some(it) = t.callback_item() {
                                if let Some(lbl) = it.label() {
                                    app::Sender::<Message>::get().send(cb(lbl.to_string()));
                                }
                            }
                        }
                    }
                    TreeReason::Selected | TreeReason::Reselected => {
                        if let Some(cb) = &on_select {
                            if let Some(it) = t.first_selected_item() {
                                if let Some(lbl) = it.label() {
                                    app::Sender::<Message>::get().send(cb(lbl.to_string()));
                                }
                            }
                        }
                    }
                    _ => {}
                });
            }
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Tree, {
            let old: &Tree<Message> = old.as_any().downcast_ref().unwrap();
            let items_changed = self.items != old.items;
            if items_changed {
                b.clear();
                for item in &self.items {
                    b.add(&item.label);
                }
            }
            if self.selected != old.selected {
                if let Some(path) = &self.selected {
                    let _ = b.select(path, true);
                }
            }
            if items_changed || self.opened != old.opened {
                // Close paths no longer opened
                for p in old.opened.iter() {
                    if !self.opened.iter().any(|n| n == p) {
                        let _ = b.open(p, false);
                    }
                }
                // Open any new paths
                for p in self.opened.iter() {
                    if !old.opened.iter().any(|n| n == p) {
                        let _ = b.open(p, true);
                    }
                }
            }
            if self.on_select.is_some() != old.on_select.is_some()
                || self.on_open.is_some() != old.on_open.is_some()
                || self.on_close.is_some() != old.on_close.is_some()
            {
                let on_select = self.on_select.clone();
                let on_open = self.on_open.clone();
                let on_close = self.on_close.clone();
                b.set_callback(move |t| match t.callback_reason() {
                    TreeReason::Opened => {
                        if let Some(cb) = &on_open {
                            if let Some(it) = t.callback_item() {
                                if let Some(lbl) = it.label() {
                                    app::Sender::<Message>::get().send(cb(lbl.to_string()));
                                }
                            }
                        }
                    }
                    TreeReason::Closed => {
                        if let Some(cb) = &on_close {
                            if let Some(it) = t.callback_item() {
                                if let Some(lbl) = it.label() {
                                    app::Sender::<Message>::get().send(cb(lbl.to_string()));
                                }
                            }
                        }
                    }
                    TreeReason::Selected | TreeReason::Reselected => {
                        if let Some(cb) = &on_select {
                            if let Some(it) = t.first_selected_item() {
                                if let Some(lbl) = it.label() {
                                    app::Sender::<Message>::get().send(cb(lbl.to_string()));
                                }
                            }
                        }
                    }
                    _ => {}
                });
            }
        });
    }
}
