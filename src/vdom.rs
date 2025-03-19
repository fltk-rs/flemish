use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::vnode::View;
use crate::widgets::WidgetMap;

#[derive(Clone)]
pub struct VirtualDom<Message> {
    pub root: Rc<RefCell<View<Message>>>,
    pub widget_map: Rc<RefCell<WidgetMap>>,
    #[allow(clippy::type_complexity)]
    subscribers: Rc<RefCell<Vec<Rc<dyn Fn(&Message)>>>>,
}

impl<Message> VirtualDom<Message>
where
    Message: Clone + 'static + Send + Sync,
{
    pub(crate) fn new(root: View<Message>) -> Self {
        let root_rc = Rc::new(RefCell::new(root));
        let widget_map = Rc::new(RefCell::new(HashMap::new()));

        let dom = Self {
            root: root_rc.clone(),
            widget_map: widget_map.clone(),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        };

        {
            root_rc.borrow().mount(&dom);
        }

        dom
    }

    pub(crate) fn subscribe<F: 'static + Fn(&Message)>(&self, callback: F) {
        self.subscribers.borrow_mut().push(Rc::new(callback));
    }

    pub(crate) fn dispatch(&self, message: Message) {
        for subscriber in self.subscribers.borrow().iter() {
            subscriber(&message);
        }
    }

    pub(crate) fn patch(&self, new: View<Message>) {
        let mut old = self.root.borrow_mut();
        new.patch(&mut old, self);
        *old = new;
    }
}
