use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Clock<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    phantom: PhantomData<Message>,
}

impl<Message> Clock<Message> {
    pub fn new(label: &str) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Clock,
            wprops: WidgetProps {
                label: Some(label.to_string()),
                ..Default::default()
            },
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Clock<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = misc::Clock::default();
        default_mount!(b, self, dom, Clock);
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Clock);
    }
}

pub use fltk::misc::ChartType;

#[derive(Debug, Clone, PartialEq)]
pub struct ChartItem {
    value: f64,
    label: String,
    col: crate::enums::Color,
}

impl ChartItem {
    pub fn new(value: f64, label: &str, col: crate::enums::Color) -> Self {
        Self {
            value,
            label: label.to_string(),
            col,
        }
    }
}

#[derive(Clone)]
pub struct Chart<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    phantom: PhantomData<Message>,
    chart_type: fltk::misc::ChartType,
    bounds: (f64, f64),
    items: Vec<ChartItem>,
}

impl<Message> Chart<Message> {
    pub fn new(items: &[ChartItem]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Chart,
            wprops: WidgetProps::default(),
            phantom: PhantomData,
            chart_type: fltk::misc::ChartType::Bar,
            bounds: (0.0, 100.0),
            items: items.to_vec(),
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Chart<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = misc::Chart::default();
        default_mount!(b, self, dom, Chart, {
            b.set_type(self.chart_type);
            b.set_bounds(self.bounds.0, self.bounds.1);
            for item in &self.items {
                b.add(item.value, &item.label, item.col);
            }
        });
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Chart, {
            let old: &Chart<Message> = old.as_any().downcast_ref().unwrap();
            if self.chart_type != old.chart_type {
                b.set_type(self.chart_type);
            }
            if self.bounds != old.bounds {
                b.set_bounds(self.bounds.0, self.bounds.1);
            }
            if self.items != old.items {
                for item in &self.items {
                    b.add(item.value, &item.label, item.col);
                }
            }
        });
    }
}

#[derive(Clone)]
pub struct Progress<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    value: f64,
    phantom: PhantomData<Message>,
}

impl<Message> Progress<Message> {
    pub fn new(value: f64) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Progress,
            wprops: WidgetProps::default(),
            value,
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Progress<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = misc::Progress::default();
        default_mount!(b, self, dom, Progress, {
            b.set_value(self.value);
        });
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Progress, {
            let old: &Progress<Message> = old.as_any().downcast_ref().unwrap();
            if self.value != old.value {
                b.set_value(self.value);
            }
        });
    }
}

#[derive(Clone)]
pub struct Spinner<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    phantom: PhantomData<Message>,
}

impl<Message> Spinner<Message> {
    pub fn new(label: &str) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Spinner,
            wprops: WidgetProps {
                label: Some(label.to_string()),
                ..Default::default()
            },
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Spinner<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = misc::Spinner::default();
        default_mount!(b, self, dom, Spinner);
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Spinner);
    }
}

#[derive(Clone)]
pub struct HelpView<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    phantom: PhantomData<Message>,
}

impl<Message> HelpView<Message> {
    pub fn new(label: &str) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::HelpView,
            wprops: WidgetProps {
                label: Some(label.to_string()),
                ..Default::default()
            },
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for HelpView<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = misc::HelpView::default();
        default_mount!(b, self, dom, HelpView);
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, HelpView);
    }
}

#[derive(Clone)]
pub struct InputChoice<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    phantom: PhantomData<Message>,
}

impl<Message> InputChoice<Message> {
    pub fn new(label: &str) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::InputChoice,
            wprops: WidgetProps {
                label: Some(label.to_string()),
                ..Default::default()
            },
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for InputChoice<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = misc::InputChoice::default();
        default_mount!(b, self, dom, InputChoice);
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, InputChoice);
    }
}
