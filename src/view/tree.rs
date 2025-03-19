use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
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
    phantom: PhantomData<Message>,
}

impl<Message: Clone> Tree<Message> {
    pub fn new(items: &[TreeItem]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Tree,
            wprops: WidgetProps::default(),
            items: items.to_vec(),
            phantom: PhantomData,
        }
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
        });
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Tree, {
            b.clear();
            for item in &self.items {
                b.add(&item.label);
            }
        });
    }
}
