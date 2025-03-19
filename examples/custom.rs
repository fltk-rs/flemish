mod custom {
    use flemish::props::*;
    use flemish::utils;
    use flemish::vdom::VirtualDom;
    use flemish::vnode::{VNode, VNodeType, View};
    use flemish::widgets::{IsWidget, WidgetUnion};
    use fltk::{prelude::*, *};
    use std::any::TypeId;
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::rc::Rc;

    #[derive(Clone)]
    struct MyFrameImpl {
        f: frame::Frame,
        // we use Rc RefCell since we pass these properties in the draw callback
        label: Rc<RefCell<String>>,
        angle: Rc<RefCell<f64>>,
    }

    impl MyFrameImpl {
        pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
            let mut f = frame::Frame::new(x, y, w, h, None);
            f.set_frame(enums::FrameType::FlatBox);
            let label = Rc::new(RefCell::new(String::new()));
            let label_rc = label.clone();
            let angle = Rc::new(RefCell::new(0.0));
            let angle_rc = angle.clone();
            f.draw(move |f| {
                draw::set_draw_color(enums::Color::Black);
                draw::set_font(enums::Font::Courier, 18);
                draw::draw_text_angled(
                    (*angle_rc.borrow() * 360.0) as i32,
                    &label_rc.borrow(),
                    (f.x() + f.w()) / 2,
                    (f.y() + f.h()) / 2,
                );
            });
            Self { f, label, angle }
        }
        pub fn set_label(&mut self, l: &str) {
            *self.label.borrow_mut() = l.to_string();
        }
        pub fn set_angle(&mut self, l: f64) {
            *self.angle.borrow_mut() = l;
        }
    }

    // We implement IsWidget to be able to add our custom widget to our WidgetUnion
    impl IsWidget for MyFrameImpl {
        fn as_widget(&self) -> fltk::widget::Widget {
            unsafe { self.f.into_widget() }
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    // This will become our VNode implementer
    #[derive(Clone)]
    pub struct MyFrame<Message> {
        node_id: usize,
        typ: VNodeType,
        wprops: WidgetProps,
        label: String,
        angle: f64,
        phantom: PhantomData<Message>,
    }

    impl<Message: 'static> MyFrame<Message> {
        pub fn new(label: &str, angle: f64) -> Self {
            Self {
                node_id: 0,
                // Here we use VNodeType::Other.
                // Similarly we use WidgetUnion::Other in our mount and patch functions
                typ: VNodeType::Other(TypeId::of::<Self>()),
                wprops: WidgetProps::default(),
                label: label.to_string(),
                angle,
                phantom: PhantomData,
            }
        }
    }

    // This is our VNode trait implementation
    impl<Message: Clone + 'static + Send + Sync> VNode<Message> for MyFrame<Message> {
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
        // This is here for optimization purpose so we won't need to dynamically cast
        // our widget to get its children.
        fn gprops(&mut self) -> Option<&mut GroupProps<Message>> {
            None
        }

        // Our mount function which will instantiate our widget
        fn mount(&self, dom: &VirtualDom<Message>) {
            let mut f = MyFrameImpl::new(0, 0, 0, 0);
            set_wprops(&mut f.f, &self.wprops);
            f.set_label(&self.label);
            f.set_angle(self.angle);
            dom.widget_map
                .borrow_mut()
                .insert(self.node_id, WidgetUnion::Other(Rc::new(f)));
        }

        // Our patch function which will update our widget in case of change
        fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>) {
            if self.typ != *old.typ() {
                utils::subtree::replace_subtree(old, self, dom);
                return;
            }
            let mut widget = {
                let mut map = dom.widget_map.borrow_mut();
                map.get_mut(&old.node_id()).cloned()
            };
            if let Some(WidgetUnion::Other(ref mut f)) = widget {
                update_wprops(&mut f.as_widget(), old.wprops(), &self.wprops);
                let old: &MyFrame<Message> = old.as_any().downcast_ref().unwrap();
                let mut f = f.as_any().downcast_ref::<MyFrameImpl>().unwrap().clone();
                if self.label != old.label {
                    f.set_label(&self.label);
                }
                if self.angle != old.angle {
                    f.set_angle(self.angle);
                }
            }
        }
    }
}

use flemish::{view::*, Settings, Subscription};

pub fn main() {
    flemish::application("app", Custom::update, Custom::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .subscription(Custom::subscription)
        .run();
}

#[derive(Default)]
struct Custom {
    angle: f64,
}

impl Custom {
    fn update(&mut self, message: i32) {
        self.angle -= message as f64 / 360.0;
    }

    fn view(&self) -> View<i32> {
        custom::MyFrame::new("Label", self.angle).view()
    }

    fn subscription(&self) -> Subscription<i32> {
        Subscription::every(std::time::Duration::from_millis(30)).map(|_| 10)
    }
}
