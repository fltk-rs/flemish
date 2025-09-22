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
        let mut newg_opt = $new.gprops();
        let newg = newg_opt.as_mut().unwrap();
        let min_len = oldg.children.len().min(newg.children.len());
        for i in 0..min_len {
            newg.children[i].patch(&mut oldg.children[i], $dom);
        }
        if newg.children.len() > oldg.children.len() {
            // Clone group handle to avoid holding a borrow across child.mount calls
            let grp_opt = {
                let mut map = $dom.widget_map.borrow_mut();
                if let Some(WidgetUnion::$typ(ref mut grp)) = map.get_mut(&old_id) {
                    Some(grp.clone())
                } else {
                    None
                }
            };
            if let Some(mut grp) = grp_opt {
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
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
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
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
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
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
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

impl FixLayout for group::Tile {
    fn fix_layout(&mut self) {}
}

#[derive(Clone)]
pub struct Tile<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
}

impl<Message> Tile<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Tile,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Tile<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut g = group::Tile::default();
        default_mount!(g, self, dom, Tile, {
            g.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            g.end();
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Tile);
        update_group_children!(old, self, dom, Tile);
    }
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
            fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name);
                update_group_children!(old, self, dom, $name);
            }
        }
    };
}

define_group!(Group);
define_group!(Scroll);
define_group!(Pack);
define_group!(Wizard);

#[derive(Clone)]
pub struct Tabs<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
    active_label: Option<String>,
    active_index: Option<i32>,
    #[allow(clippy::type_complexity)]
    on_change: Option<std::rc::Rc<Box<dyn Fn(String) -> Message>>>,
}

impl<Message> Tabs<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Tabs,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
            active_label: None,
            active_index: None,
            on_change: None,
        }
    }
    pub fn active_label(mut self, label: &str) -> Self {
        self.active_label = Some(label.to_string());
        self
    }
    pub fn active_index(mut self, idx: i32) -> Self {
        self.active_index = Some(idx);
        self
    }
    pub fn on_change<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
        self.on_change = Some(std::rc::Rc::new(Box::new(f)));
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Tabs<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut g = group::Tabs::default();
        default_mount!(g, self, dom, Tabs, {
            g.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            g.end();
            g.fix_layout();

            if let Some(lbl) = &self.active_label {
                for i in 0..g.children() {
                    if let Some(ch) = g.child(i) {
                        let l2 = ch.label();
                        if &l2 == lbl {
                            if let Some(gr) = group::Group::from_dyn_widget(&ch) {
                                let _ = g.set_value(&gr);
                            }
                            break;
                        }
                    }
                }
            }
            if let Some(idx) = self.active_index {
                if idx >= 0 {
                    if let Some(ch) = g.child(idx) {
                        if let Some(gr) = group::Group::from_dyn_widget(&ch) {
                            let _ = g.set_value(&gr);
                        }
                    }
                }
            }

            if let Some(cb) = self.on_change.clone() {
                g.set_callback(move |t| {
                    if let Some(val) = t.value() {
                        let lbl = val.label();
                        app::Sender::<Message>::get().send(cb(lbl));
                    }
                });
            }
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Tabs, {
            let old: &Tabs<Message> = old.as_any().downcast_ref().unwrap();
            if self.active_label != old.active_label {
                if let Some(lbl) = &self.active_label {
                    for i in 0..b.children() {
                        if let Some(ch) = b.child(i) {
                            let l2 = ch.label();
                            if &l2 == lbl {
                                if let Some(gr) = group::Group::from_dyn_widget(&ch) {
                                    let _ = b.set_value(&gr);
                                }
                                break;
                            }
                        }
                    }
                }
            }
            if self.active_index != old.active_index {
                if let Some(idx) = self.active_index {
                    if idx >= 0 {
                        if let Some(ch) = b.child(idx) {
                            if let Some(gr) = group::Group::from_dyn_widget(&ch) {
                                let _ = b.set_value(&gr);
                            }
                        }
                    }
                }
            }
            if self.on_change.is_some() != old.on_change.is_some() {
                let cb = self.on_change.clone();
                b.set_callback(move |t| {
                    if let Some(cb) = &cb {
                        if let Some(val) = t.value() {
                            let lbl = val.label();
                            app::Sender::<Message>::get().send(cb(lbl));
                        }
                    }
                });
            }
        });
        // Update children
        update_group_children!(old, self, dom, Tabs);
    }
}

#[derive(Clone)]
pub struct Grid<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    gprops: GroupProps<Message>,
    rows: i32,
    cols: i32,
    gap_x: i32,
    gap_y: i32,
}

impl<Message> Grid<Message> {
    pub fn new(children: &[View<Message>]) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::Grid,
            wprops: WidgetProps::default(),
            gprops: GroupProps {
                children: children.to_vec(),
            },
            rows: 0,
            cols: 0,
            gap_x: 0,
            gap_y: 0,
        }
    }
    pub fn layout(mut self, rows: i32, cols: i32) -> Self {
        self.rows = rows;
        self.cols = cols;
        self
    }
    pub fn gap(mut self, x: i32, y: i32) -> Self {
        self.gap_x = x;
        self.gap_y = y;
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Grid<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        Some(&mut self.gprops)
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut g = group::Grid::default();
        default_mount!(g, self, dom, Grid, {
            if self.rows > 0 || self.cols > 0 {
                g.set_layout(self.rows, self.cols);
            }
            g.set_gap(self.gap_x, self.gap_y);
            g.begin();
            for child in &self.gprops.children {
                child.mount(dom);
            }
            g.end();
            g.fix_layout();
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, Grid, {
            let old: &Grid<Message> = old.as_any().downcast_ref().unwrap();
            if (self.rows != old.rows || self.cols != old.cols) && (self.rows > 0 || self.cols > 0)
            {
                b.set_layout(self.rows, self.cols);
            }
            if self.gap_x != old.gap_x || self.gap_y != old.gap_y {
                b.set_gap(self.gap_x, self.gap_y);
            }
        });
        // Update children
        update_group_children!(old, self, dom, Grid);
    }
}
