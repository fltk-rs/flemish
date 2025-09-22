use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
struct ValuatorProps {
    value: f64,
    minimum: f64,
    maximum: f64,
    step: f64,
}

impl Default for ValuatorProps {
    fn default() -> Self {
        Self {
            value: 0.,
            minimum: 0.,
            maximum: 100.,
            step: 1.,
        }
    }
}

macro_rules! define_valuator {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            iprops: ValuatorProps,
            #[allow(clippy::type_complexity)]
            change_cb: Option<Rc<Box<dyn Fn(f64) -> Message>>>,
        }

        impl<Message> $name<Message> {
            pub fn new(value: f64) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    iprops: ValuatorProps {
                        value,
                        ..Default::default()
                    },
                    change_cb: None,
                }
            }
            pub fn minimum(mut self, v: f64) -> Self {
                self.iprops.minimum = v;
                self
            }
            pub fn maximum(mut self, v: f64) -> Self {
                self.iprops.maximum = v;
                self
            }
            pub fn step(mut self, v: f64) -> Self {
                self.iprops.step = v;
                self
            }
            pub fn on_change<F: 'static + Fn(f64) -> Message>(mut self, f: F) -> Self {
                self.change_cb = Some(Rc::new(Box::new(f)));
                self
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = valuator::$name::default();
                default_mount!(b, self, dom, $name, {
                    set_wprops(&mut b, &self.wprops);
                    b.set_value(self.iprops.value);
                    b.set_minimum(self.iprops.minimum);
                    b.set_maximum(self.iprops.maximum);
                    b.set_step(self.iprops.step, 1);
                    let change_cb = self.change_cb.clone();
                    b.set_callback(move |b| {
                        let v = b.value();
                        if let Some(change_cb) = &change_cb {
                            app::Sender::<Message>::get().send(change_cb(v));
                        }
                    });
                });
            }
            fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    let oldi = &old.iprops;
                    let newi = &self.iprops;
                    if oldi.value != newi.value {
                        b.set_value(newi.value);
                        let change_cb = self.change_cb.clone();
                        b.set_callback(move |b| {
                            let v = b.value();
                            if let Some(change_cb) = &change_cb {
                                app::Sender::<Message>::get().send(change_cb(v));
                            }
                        });
                    }
                    if oldi.minimum != newi.minimum {
                        b.set_minimum(newi.minimum);
                    }
                    if oldi.maximum != newi.maximum {
                        b.set_maximum(newi.maximum);
                    }
                    if oldi.step != newi.step {
                        b.set_step(newi.step, 1);
                    }
                });
            }
        }
    };
}

macro_rules! define_valuator_with_tprops {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            iprops: ValuatorProps,
            tprops: TextProps,
            #[allow(clippy::type_complexity)]
            change_cb: Option<Rc<Box<dyn Fn(f64) -> Message>>>,
        }

        impl<Message> $name<Message> {
            pub fn new(value: f64) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    iprops: ValuatorProps {
                        value,
                        ..Default::default()
                    },
                    tprops: TextProps::default(),
                    change_cb: None,
                }
            }
            pub fn minimum(mut self, v: f64) -> Self {
                self.iprops.minimum = v;
                self
            }
            pub fn maximum(mut self, v: f64) -> Self {
                self.iprops.maximum = v;
                self
            }
            pub fn step(mut self, v: f64) -> Self {
                self.iprops.step = v;
                self
            }
            pub fn on_change<F: 'static + Fn(f64) -> Message>(mut self, f: F) -> Self {
                self.change_cb = Some(Rc::new(Box::new(f)));
                self
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = valuator::$name::default();
                set_tprops!(b, self.tprops);
                default_mount!(b, self, dom, $name, {
                    set_wprops(&mut b, &self.wprops);
                    b.set_value(self.iprops.value);
                    b.set_minimum(self.iprops.minimum);
                    b.set_maximum(self.iprops.maximum);
                    b.set_step(self.iprops.step, 1);
                    let change_cb = self.change_cb.clone();
                    b.set_callback(move |b| {
                        let v = b.value();
                        if let Some(change_cb) = &change_cb {
                            app::Sender::<Message>::get().send(change_cb(v));
                        }
                    });
                });
            }
            fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    update_tprops!(b, self.tprops, old.tprops);
                    let oldi = &old.iprops;
                    let newi = &self.iprops;
                    if oldi.value != newi.value {
                        b.set_value(newi.value);
                        let change_cb = self.change_cb.clone();
                        b.set_callback(move |b| {
                            let v = b.value();
                            if let Some(change_cb) = &change_cb {
                                app::Sender::<Message>::get().send(change_cb(v));
                            }
                        });
                    }
                    if oldi.minimum != newi.minimum {
                        b.set_minimum(newi.minimum);
                    }
                    if oldi.maximum != newi.maximum {
                        b.set_maximum(newi.maximum);
                    }
                    if oldi.step != newi.step {
                        b.set_step(newi.step, 1);
                    }
                });
            }
        }
    };
}

#[derive(Clone)]
pub struct HorScrollbar<Message> {
    node_id: usize,
    typ: VNodeType,
    wprops: WidgetProps,
    iprops: ValuatorProps,
    #[allow(clippy::type_complexity)]
    change_cb: Option<Rc<Box<dyn Fn(f64) -> Message>>>,
}

impl<Message> HorScrollbar<Message> {
    pub fn new(value: f64) -> Self {
        Self {
            node_id: 0,
            typ: VNodeType::HorScrollbar,
            wprops: WidgetProps::default(),
            iprops: ValuatorProps {
                value,
                ..Default::default()
            },
            change_cb: None,
        }
    }
    pub fn on_change<F: 'static + Fn(f64) -> Message>(mut self, f: F) -> Self {
        self.change_cb = Some(Rc::new(Box::new(f)));
        self
    }
}

impl<Message: Clone + 'static + Send + Sync> VNode<Message> for HorScrollbar<Message> {
    default_impl!();
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
        None
    }
    fn mount(&self, dom: &VirtualDom<Message>) {
        let mut b = valuator::Scrollbar::default().with_type(valuator::ScrollbarType::Horizontal);
        default_mount!(b, self, dom, HorScrollbar, {
            set_wprops(&mut b, &self.wprops);
            b.set_value(self.iprops.value);
            b.set_minimum(self.iprops.minimum);
            b.set_maximum(self.iprops.maximum);
            b.set_step(self.iprops.step, 1);
            let change_cb = self.change_cb.clone();
            b.set_callback(move |b| {
                let v = b.value();
                if let Some(change_cb) = &change_cb {
                    app::Sender::<Message>::get().send(change_cb(v));
                }
            });
        });
    }
    fn patch(&mut self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
        let b;
        default_patch!(b, self, old, dom, HorScrollbar, {
            let old: &HorScrollbar<Message> = old.as_any().downcast_ref().unwrap();
            let oldi = &old.iprops;
            let newi = &self.iprops;
            if oldi.value != newi.value {
                b.set_value(newi.value);
                let change_cb = self.change_cb.clone();
                b.set_callback(move |b| {
                    let v = b.value();
                    if let Some(change_cb) = &change_cb {
                        app::Sender::<Message>::get().send(change_cb(v));
                    }
                });
            }
            if oldi.minimum != newi.minimum {
                b.set_minimum(newi.minimum);
            }
            if oldi.maximum != newi.maximum {
                b.set_maximum(newi.maximum);
            }
            if oldi.step != newi.step {
                b.set_step(newi.step, 1);
            }
        });
    }
}

define_valuator!(Scrollbar);
define_valuator!(Roller);
define_valuator!(Adjuster);
define_valuator_with_tprops!(ValueInput);
define_valuator_with_tprops!(ValueOutput);
define_valuator!(FillSlider);
define_valuator!(FillDial);
define_valuator!(HorSlider);
define_valuator!(HorFillSlider);
define_valuator!(HorNiceSlider);
define_valuator_with_tprops!(ValueSlider);
define_valuator_with_tprops!(HorValueSlider);
