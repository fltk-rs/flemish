use crate::vdom::VirtualDom;
use crate::vnode::{VNode, View};
use fltk::{prelude::*, widget};

pub fn replace_subtree<Message>(
    old: &mut View<Message>,
    new: &dyn VNode<Message>,
    dom: &VirtualDom<Message>,
) where
    Message: Clone + 'static + Send + Sync,
{
    remove_subtree(old, dom);
    new.mount(dom);
}

pub fn remove_subtree<Message>(old: &mut View<Message>, dom: &VirtualDom<Message>)
where
    Message: Clone + 'static + Send + Sync,
{
    if let Some(gprops) = old.gprops() {
        for ch in &mut gprops.children {
            remove_subtree(ch, dom);
        }
    }
    let old_ptr = old.node_id();
    // Clean up any subscriptions owned by this node
    dom.unsubscribe_owner(old_ptr);
    if let Some(wu) = dom.widget_map.borrow_mut().remove(&old_ptr) {
        if let Some(mut par) = wu.view().parent() {
            par.remove(&wu.view());
            widget::Widget::delete(wu.view());
        }
    }
}
