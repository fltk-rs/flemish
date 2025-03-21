use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::rc::Rc;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct InputProps {
    value: String,
}

macro_rules! define_input {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            iprops: InputProps,
            tprops: TextProps,
            #[allow(clippy::type_complexity)]
            change_cb: Option<Rc<Box<dyn Fn(String) -> Message>>>,
            enter_cb: Option<Rc<Box<dyn Fn(String) -> Message>>>,
        }

        impl<Message> $name<Message> {
            pub fn new(value: &str) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    iprops: InputProps {
                        value: value.to_string(),
                    },
                    tprops: TextProps::default(),
                    change_cb: None,
                    enter_cb: None,
                }
            }
            pub fn on_input<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
                self.wprops.when =
                    Some(enums::CallbackTrigger::Changed | enums::CallbackTrigger::EnterKeyAlways);
                self.change_cb = Some(Rc::new(Box::new(f)));
                self
            }
            pub fn on_submit<F: 'static + Fn(String) -> Message>(mut self, f: F) -> Self {
                self.wprops.when =
                    Some(enums::CallbackTrigger::Changed | enums::CallbackTrigger::EnterKeyAlways);
                self.enter_cb = Some(Rc::new(Box::new(f)));
                self
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = input::$name::default();
                default_mount!(b, self, dom, $name, {
                    set_tprops!(b, self.tprops);
                    b.set_value(&self.iprops.value);
                    let change_cb = self.change_cb.clone();
                    let enter_cb = self.enter_cb.clone();
                    b.set_callback(move |b| {
                        let v = b.value();
                        if let Some(change_cb) = &change_cb {
                            app::Sender::<Message>::get().send(change_cb(v.clone()));
                        }
                        if let Some(enter_cb) = &enter_cb {
                            if app::event() == enums::Event::KeyDown
                                && app::event_key() == enums::Key::Enter
                            {
                                app::Sender::<Message>::get().send(enter_cb(v));
                            }
                        }
                    });
                });
            }
            fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
                let b;
                default_patch!(b, self, old, dom, $name, {
                    let old: &$name<Message> = old.as_any().downcast_ref().unwrap();
                    update_tprops!(b, self.tprops, old.tprops);
                    let oldi = &old.iprops;
                    let newi = &self.iprops;
                    if oldi.value != newi.value {
                        b.set_value(&newi.value);
                        let change_cb = self.change_cb.clone();
                        let enter_cb = self.enter_cb.clone();
                        b.set_callback(move |b| {
                            let v = b.value();
                            if let Some(change_cb) = &change_cb {
                                app::Sender::<Message>::get().send(change_cb(v.clone()));
                            }
                            if let Some(enter_cb) = &enter_cb {
                                if app::event() == enums::Event::KeyDown
                                    && app::event_key() == enums::Key::Enter
                                {
                                    app::Sender::<Message>::get().send(enter_cb(v));
                                }
                            }
                        });
                    }
                });
            }
        }
    };
}

define_input!(Input);
define_input!(IntInput);
define_input!(FloatInput);
define_input!(MultilineInput);
define_input!(SecretInput);
define_input!(FileInput);
