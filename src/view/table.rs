use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::{IsWidget, WidgetUnion};
use fltk::table;
use fltk::{prelude::*, *};
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
struct SmartTable {
    st: fltk_table::SmartTable,
}

impl IsWidget for SmartTable {
    fn as_widget(&self) -> fltk::widget::Widget {
        unsafe { self.st.into_widget() }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct Table<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    headers: Vec<String>,
    cells: Vec<Vec<String>>,
    phantom: PhantomData<Message>,
}

impl<Message: Clone> Table<Message> {
    pub fn new(headers: &[&str], cells: &[&[&str]]) -> Self {
        let headers = headers.iter().map(|s| s.to_string()).collect();
        let cells = cells
            .iter()
            .map(|a| a.iter().map(|s| s.to_string()).collect())
            .collect();
        Self {
            node_id: 0,
            typ: VNodeType::Other(std::any::TypeId::of::<SmartTable>()),
            wprops: WidgetProps::default(),
            headers,
            cells,
            phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for Table<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = fltk_table::SmartTable::default();
        b.set_opts(fltk_table::TableOpts {
            rows: self.cells.len() as i32,
            cols: self.cells[0].len() as i32,
            ..Default::default()
        });
        set_wprops(&mut *b, &self.wprops);
        for i in 0..self.headers.len() {
            b.set_col_header_value(i as i32, &self.headers[i]);
        }
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                b.set_cell_value(i as i32, j as i32, &self.cells[i][j]);
            }
        }
        dom.widget_map.borrow_mut().insert(
            self.node_id,
            WidgetUnion::Other(Rc::new(SmartTable { st: b })),
        );
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        if self.typ != *old.typ() {
            crate::utils::subtree::replace_subtree(old, self, dom);
            return;
        }
        self.set_node_id(old.node_id());
        {
            let mut map = dom.widget_map.borrow_mut();
            if let Some(WidgetUnion::Other(ref mut f)) = map.get_mut(&old.node_id()) {
                update_wprops(&mut f.as_widget(), old.wprops(), &self.wprops);
                let old: &Table<Message> = old.as_any().downcast_ref().unwrap();
                if self.cells != old.cells || self.headers != old.headers {
                    let mut f = f.as_any().downcast_ref::<SmartTable>().unwrap().clone();
                    f.st.clear();
                    f.st.set_opts(fltk_table::TableOpts {
                        rows: self.cells.len() as i32,
                        cols: self.cells[0].len() as i32,
                        ..Default::default()
                    });
                    for i in 0..self.cells.len() {
                        for j in 0..self.cells[i].len() {
                            f.st.set_cell_value(i as i32, j as i32, &self.cells[i][j]);
                        }
                    }
                    for i in 0..self.headers.len() {
                        f.st.set_col_header_value(i as i32, &self.headers[i]);
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TableRow<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    rows: i32,
    #[allow(clippy::type_complexity)]
    on_select: Option<Rc<Box<dyn Fn(i32) -> Message>>>,
}

impl<Message> TableRow<Message> {
    pub fn new(rows: i32) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::TableRow,
            wprops: WidgetProps::default(),
            rows,
            on_select: None,
        }
    }
    pub fn on_select<F: 'static + Fn(i32) -> Message>(mut self, f: F) -> Self {
        self.on_select = Some(Rc::new(Box::new(f)));
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for TableRow<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut t = table::TableRow::default();
        default_mount!(t, self, dom, TableRow, {
            t.set_rows(self.rows);
            if let Some(cb) = self.on_select.clone() {
                t.set_callback(move |tbl| {
                    let row = tbl.callback_row();
                    app::Sender::<Message>::get().send(cb(row));
                });
            }
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, TableRow, {
            let old: &TableRow<Message> = old.as_any().downcast_ref().unwrap();
            if self.rows != old.rows {
                b.set_rows(self.rows);
            }
            if self.on_select.is_some() != old.on_select.is_some() {
                let cb = self.on_select.clone();
                b.set_callback(move |tbl| {
                    if let Some(cb) = &cb {
                        let row = tbl.callback_row();
                        app::Sender::<Message>::get().send(cb(row));
                    }
                });
            }
        });
    }
}
