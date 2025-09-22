use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::vnode::View;
use crate::widgets::WidgetMap;

#[derive(Clone)]
pub struct VirtualDom<Message> {
    pub root: Rc<RefCell<View<Message>>>,
    pub widget_map: Rc<RefCell<WidgetMap>>,
    #[allow(clippy::type_complexity)]
    subscribers: Rc<RefCell<Vec<(usize, Rc<dyn Fn(&Message)>)>>>,
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

    pub(crate) fn subscribe_owned<F: 'static + Fn(&Message)>(&self, owner: usize, callback: F) {
        self.subscribers
            .borrow_mut()
            .push((owner, Rc::new(callback)));
    }

    pub(crate) fn unsubscribe_owner(&self, owner: usize) {
        self.subscribers.borrow_mut().retain(|(id, _)| *id != owner);
    }

    pub(crate) fn dispatch(&self, message: Message) {
        for (_, subscriber) in self.subscribers.borrow().iter() {
            subscriber(&message);
        }
    }

    pub(crate) fn patch(&self, new: View<Message>) {
        let mut new = new;
        let mut old = self.root.borrow_mut();
        new.patch(&mut old, self);
        *old = new;
    }
}
