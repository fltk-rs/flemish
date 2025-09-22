use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Frame<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    phantom: PhantomData<Message>,
}

impl<Message> Frame<Message> {
    pub fn new(label: &str) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Frame,
            wprops: WidgetProps {
                label: Some(label.to_string()),
                ..Default::default()
            },
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Frame<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = frame::Frame::default();
        default_mount!(b, self, dom, Frame);
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Frame);
    }
}
