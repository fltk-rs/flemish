use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};

macro_rules! update_group_children {
    ($old: tt, $new: tt, $dom: tt, $typ: ident) => {
        let old_id = $old.node_id();
        let oldg = $old.gprops().unwrap();
        let newg = &$new.gprops;
        let min_len = oldg.children.len().min(newg.children.len());
        for i in 0..min_len {
            newg.children[i].patch(&mut oldg.children[i], $dom);
        }
        if newg.children.len() > oldg.children.len() {
            let mut group_widget = {
                let map = $dom.widget_map.borrow();
                map.get(&old_id).cloned()
            };
            if let Some(WidgetUnion::$typ(ref mut grp)) = group_widget {
                grp.begin();
                for i in oldg.children.len()..newg.children.len() {
                    newg.children[i].mount($dom);
                }
                grp.end();
                grp.fix_layout();
            }
        }
        if oldg.children.len() > newg.children.len() {
            for i in newg.children.len()..oldg.children.len() {
                $crate::utils::subtree::remove_subtree(&mut oldg.children[i], $dom);
            }
        }
    };
}

#[derive(Clone)]
pub struct Column<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
    margins: (i32, i32, i32, i32),
    padding: i32,
}

impl<Message> Column<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Column,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
            margins: (0, 0, 0, 0),
            padding: 0,
        }
    }
    pub fn margins(mut self, l: i32, t: i32, r: i32, b: i32) -> Self {
        self.margins = (l, t, r, b);
        self
    }
    pub fn padding(mut self, p: i32) -> Self {
        self.padding = p;
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Column<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut col = group::Flex::default().column();
        default_mount!(col, self, dom, Column, {
            let (l, t, r, bot) = self.margins;
            col.set_margins(l, t, r, bot);
            col.set_pad(self.padding);
            col.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            col.end();
        });
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Column);
        update_group_children!(old, self, dom, Column);
    }
}

#[derive(Clone)]
pub struct Row<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
    margins: (i32, i32, i32, i32),
    padding: i32,
}

impl<Message> Row<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Row,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
            margins: (0, 0, 0, 0),
            padding: 0,
        }
    }
    pub fn margins(mut self, l: i32, t: i32, r: i32, b: i32) -> Self {
        self.margins = (l, t, r, b);
        self
    }
    pub fn padding(mut self, p: i32) -> Self {
        self.padding = p;
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Row<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut row = group::Flex::default().row();
        default_mount!(row, self, dom, Row, {
            let (l, t, r, bot) = self.margins;
            row.set_margins(l, t, r, bot);
            row.set_pad(self.padding);
            row.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            row.end();
        });
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Row, {
            let old: &Row<Message> = old.as_any().downcast_ref().unwrap();
            if self.margins != old.margins {
                let (l, t, r, bot) = self.margins;
                b.set_margins(l, t, r, bot);
            }
            if self.padding != old.padding {
                b.set_pad(self.padding);
            }
        });
        update_group_children!(old, self, dom, Row);
    }
}

#[derive(Clone)]
pub struct HorPack<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
}

impl<Message> HorPack<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::HorPack,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for HorPack<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut row = group::Pack::default().with_type(group::PackType::Horizontal);
        default_mount!(row, self, dom, HorPack, {
            row.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            row.end();
            row.auto_layout();
        });
    }
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, HorPack);
        update_group_children!(old, self, dom, HorPack);
    }
}

trait FixLayout {
    fn fix_layout(&mut self);
}

impl FixLayout for group::Tabs {
    fn fix_layout(&mut self) {
        self.auto_layout();
    }
}

impl FixLayout for group::Scroll {
    fn fix_layout(&mut self) {
        self.auto_layout();
    }
}

impl FixLayout for group::Pack {
    fn fix_layout(&mut self) {}
}

impl FixLayout for group::Group {
    fn fix_layout(&mut self) {}
}

impl FixLayout for group::Wizard {
    fn fix_layout(&mut self) {}
}

impl FixLayout for group::Grid {
    fn fix_layout(&mut self) {}
}

impl FixLayout for group::Flex {
    fn fix_layout(&mut self) {}
}

macro_rules! define_group {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            gprops: GroupProps<Message>,
        }

        impl<Message> $name<Message> {
            pub fn new(children: &[View<Message>]) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    gprops: GroupProps {
                        children: children.to_vec(),
                    },
                }
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                Some(&mut self.gprops)
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut g = group::$name::default();
                default_mount!(g, self, dom, $name, {
                    g.begin();
                    for child in &self.gprops.children {
                        child.mount(dom);
                    }
                    g.end();
                    g.fix_layout();
                });
            }
            fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name);
                update_group_children!(old, self, dom, $name);
            }
        }
    };
}

define_group!(Group);
define_group!(Scroll);
define_group!(Tabs);
define_group!(Pack);
define_group!(Grid);
define_group!(Wizard);
