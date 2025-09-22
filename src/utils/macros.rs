#[doc(hidden)]
#[macro_export]
macro_rules! default_impl {
    () => {
        fn node_id(&self) -> usize {
            self.node_id
        }
        fn set_node_id(&mut self, id: usize) {
            self.node_id = id;
        }
        fn typ(&self) -> &VNodeType {
            &self.typ
        }
        fn wprops(&mut self) -> &mut WidgetProps {
            &mut self.wprops
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    };
}

pub use default_impl;

#[doc(hidden)]
#[macro_export]
macro_rules! default_mount {
    ($b: ident, $self: expr, $dom: ident, $var: ident) => {{
        set_wprops(&mut $b, &$self.wprops);
        $dom.widget_map
            .borrow_mut()
            .insert($self.node_id, WidgetUnion::$var($b));
    }};
    ($b: ident, $self: expr, $dom: ident, $var: ident, $block1: block) => {{
        set_wprops(&mut $b, &$self.wprops);
        $block1
        $dom.widget_map
            .borrow_mut()
            .insert($self.node_id, WidgetUnion::$var($b));
    }};
    ($b: ident, $self: expr, $dom: ident, $var: ident, $block1: block, $block2: block) => {{
        set_wprops(&mut $b, &$self.wprops);
        $block1
        $dom.widget_map
            .borrow_mut()
            .insert($self.node_id, WidgetUnion::$var($b));
        $block2
    }};
}

pub use default_mount;

#[doc(hidden)]
#[macro_export]
macro_rules! default_patch {
    ($b: ident, $self: expr, $old: expr, $dom: ident, $var: ident) => {{
        if $self.typ != *$old.typ() {
            $crate::utils::subtree::replace_subtree($old, $self, $dom);
            return;
        }
        // keep VDOM identity stable
        $self.set_node_id($old.node_id());
        // borrow the actual widget in-place without cloning; keep the borrow guard alive
        {
            let mut map = $dom.widget_map.borrow_mut();
            if let Some(WidgetUnion::$var(ref mut f)) = map.get_mut(&$old.node_id()) {
                $b = f;
                update_wprops($b, $old.wprops(), &$self.wprops);
            }
        }
    }};
    ($b: ident, $self: expr, $old: expr, $dom: ident, $var: ident, $block1: block) => {{
        if $self.typ != *$old.typ() {
            $crate::utils::subtree::replace_subtree($old, $self, $dom);
            return;
        }
        // keep VDOM identity stable
        $self.set_node_id($old.node_id());
        // borrow the actual widget in-place without cloning; keep the borrow guard alive
        {
            let mut map = $dom.widget_map.borrow_mut();
            if let Some(WidgetUnion::$var(ref mut f)) = map.get_mut(&$old.node_id()) {
                $b = f;
                update_wprops($b, $old.wprops(), &$self.wprops);
                $block1
            }
        }
    }};
}

pub use default_patch;
