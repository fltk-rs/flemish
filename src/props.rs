use crate::vnode::View;
use fltk::{prelude::*, *};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct WidgetProps {
    pub label: Option<String>,
    pub fixed: Option<i32>,
    pub boxtype: Option<enums::FrameType>,
    pub color: Option<enums::Color>,
    pub selection_color: Option<enums::Color>,
    pub label_color: Option<enums::Color>,
    pub label_font: Option<enums::Font>,
    pub label_size: Option<i32>,
    pub tooltip: Option<String>,
    pub align: Option<enums::Align>,
    pub when: Option<enums::CallbackTrigger>,
    pub visible: Option<bool>,
    pub deactivate: Option<bool>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub w: Option<i32>,
    pub h: Option<i32>,
}

#[derive(Clone)]
pub struct GroupProps<Message> {
    pub children: Vec<View<Message>>,
}

impl<Message> Default for GroupProps<Message> {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

pub fn set_wprops<W>(w: &mut W, wprops: &WidgetProps)
where
    W: WidgetExt + 'static,
{
    if wprops.x.is_some() || wprops.y.is_some() || wprops.w.is_some() || wprops.h.is_some() {
        w.resize(
            wprops.x.unwrap_or(0),
            wprops.y.unwrap_or(0),
            wprops.w.unwrap_or(0),
            wprops.h.unwrap_or(0),
        );
    }

    if let Some(lbl) = &wprops.label {
        w.set_label(lbl);
    }

    if let Some(sz) = &wprops.fixed {
        if let Some(p) = w.parent() {
            if let Some(mut p) = group::Flex::from_dyn_widget(&p) {
                p.fixed(w, *sz);
            }
        }
    }

    if let Some(c) = &wprops.color {
        w.set_color(*c);
    }

    if let Some(c) = &wprops.boxtype {
        w.set_frame(*c);
    }

    if let Some(c) = &wprops.selection_color {
        w.set_selection_color(*c);
    }

    if let Some(c) = &wprops.label_color {
        w.set_label_color(*c);
    }

    if let Some(f) = &wprops.label_font {
        w.set_label_font(*f);
    }

    if let Some(s) = &wprops.label_size {
        w.set_label_size(*s);
    }

    if let Some(t) = &wprops.tooltip {
        w.set_tooltip(t);
    }

    if let Some(a) = &wprops.align {
        w.set_align(*a);
    }

    if let Some(when) = &wprops.when {
        w.set_trigger(*when);
    }

    if let Some(v) = &wprops.visible {
        if *v {
            w.show();
        } else {
            w.hide();
        }
    }

    if let Some(d) = &wprops.deactivate {
        if *d {
            w.deactivate();
        } else {
            w.activate();
        }
    }
}

pub fn update_wprops<W>(w: &mut W, old_wprops: &WidgetProps, new_wprops: &WidgetProps)
where
    W: WidgetExt + 'static,
{
    #[allow(clippy::collapsible_if)]
    if old_wprops.x != new_wprops.x
        || old_wprops.y != new_wprops.y
        || old_wprops.w != new_wprops.w
        || old_wprops.h != new_wprops.h
    {
        if new_wprops.x.is_some()
            || new_wprops.y.is_some()
            || new_wprops.w.is_some()
            || new_wprops.h.is_some()
        {
            w.resize(
                new_wprops.x.unwrap_or(0),
                new_wprops.y.unwrap_or(0),
                new_wprops.w.unwrap_or(0),
                new_wprops.h.unwrap_or(0),
            );
        }
    }

    if old_wprops.label != new_wprops.label {
        if let Some(lbl) = &new_wprops.label {
            w.set_label(lbl);
        } else {
            w.set_label("");
        }
    }

    if old_wprops.fixed != new_wprops.fixed {
        if let Some(sz) = &new_wprops.fixed {
            if let Some(p) = w.parent() {
                if let Some(mut p) = group::Flex::from_dyn_widget(&p) {
                    p.fixed(w, *sz);
                }
            }
        }
    }

    if old_wprops.color != new_wprops.color {
        if let Some(c) = &new_wprops.color {
            w.set_color(*c);
        }
    }

    if old_wprops.boxtype != new_wprops.boxtype {
        if let Some(c) = &new_wprops.boxtype {
            w.set_frame(*c);
        }
    }

    if old_wprops.selection_color != new_wprops.selection_color {
        if let Some(c) = &new_wprops.selection_color {
            w.set_selection_color(*c);
        }
    }

    if old_wprops.label_color != new_wprops.label_color {
        if let Some(c) = &new_wprops.label_color {
            w.set_label_color(*c);
        }
    }

    if old_wprops.label_font != new_wprops.label_font {
        if let Some(f) = &new_wprops.label_font {
            w.set_label_font(*f);
        }
    }

    if old_wprops.label_size != new_wprops.label_size {
        if let Some(s) = &new_wprops.label_size {
            w.set_label_size(*s);
        }
    }

    if old_wprops.tooltip != new_wprops.tooltip {
        if let Some(t) = &new_wprops.tooltip {
            w.set_tooltip(t);
        }
    }

    if old_wprops.align != new_wprops.align {
        if let Some(a) = &new_wprops.align {
            w.set_align(*a);
        }
    }

    if old_wprops.when != new_wprops.when {
        if let Some(when) = &new_wprops.when {
            w.set_trigger(*when);
        }
    }

    if old_wprops.visible != new_wprops.visible {
        if let Some(v) = &new_wprops.visible {
            if *v {
                w.show();
            } else {
                w.hide();
            }
        }
    }

    if old_wprops.deactivate != new_wprops.deactivate {
        if let Some(d) = &new_wprops.deactivate {
            if *d {
                w.deactivate();
            } else {
                w.activate();
            }
        }
    }
}
