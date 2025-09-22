use crate::props::*;
use crate::utils::macros::*;
use crate::vdom::VirtualDom;
use crate::vnode::{VNode, VNodeType, View};
use crate::widgets::WidgetUnion;
use fltk::{prelude::*, *};
use std::marker::PhantomData;

#[derive(Default, Clone, Debug, PartialEq)]
struct OutputProps {
    value: String,
}

macro_rules! define_output {
    ($name: ident) => {
        #[derive(Clone)]
        pub struct $name<Message> {
            node_id: usize,
            typ: VNodeType,
            wprops: WidgetProps,
            iprops: OutputProps,
            tprops: TextProps,
            phantom: PhantomData<Message>,
        }

        impl<Message> $name<Message> {
            pub fn new(value: &str) -> Self {
                Self {
                    node_id: 0,
                    typ: VNodeType::$name,
                    wprops: WidgetProps::default(),
                    iprops: OutputProps {
                        value: value.to_string(),
                    },
                    tprops: TextProps::default(),
                    phantom: PhantomData,
                }
            }
        }

        impl<Message: Clone + 'static + Send + Sync> VNode<Message> for $name<Message> {
            default_impl!();
            fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
                None
            }
            fn mount(&self, dom: &VirtualDom<Message>) {
                let mut b = output::$name::default();
                default_mount!(b, self, dom, $name, {
                    set_tprops!(b, self.tprops);
                    b.set_value(&self.iprops.value);
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
                        b.set_value(&newi.value);
                    }
                });
            }
        }
    };
}

define_output!(Output);
define_output!(MultilineOutput);
