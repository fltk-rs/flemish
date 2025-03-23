use crate::id::next_id;
use crate::props::*;
use crate::vdom::VirtualDom;
use fltk::enums;
use std::any::Any;

#[derive(Clone, Debug, PartialEq)]
pub enum VNodeType {
    Column,
    Row,
    Button,
    Frame,
    Input,
    MenuBar,
    RadioButton,
    ToggleButton,
    RoundButton,
    CheckButton,
    LightButton,
    RepeatButton,
    RadioLightButton,
    RadioRoundButton,
    ReturnButton,
    ShortcutButton,
    Group,
    Pack,
    HorPack,
    Tabs,
    Scroll,
    Tile,
    Wizard,
    ColorChooser,
    Grid,
    TextDisplay,
    TextEditor,
    IntInput,
    FloatInput,
    MultilineInput,
    SecretInput,
    FileInput,
    Output,
    MultilineOutput,
    Choice,
    SysMenuBar,
    MenuButton,
    Slider,
    NiceSlider,
    ValueSlider,
    Dial,
    LineDial,
    Counter,
    Scrollbar,
    HorScrollbar,
    Roller,
    Adjuster,
    ValueInput,
    ValueOutput,
    FillSlider,
    FillDial,
    HorSlider,
    HorFillSlider,
    HorNiceSlider,
    HorValueSlider,
    Browser,
    SelectBrowser,
    HoldBrowser,
    MultiBrowser,
    FileBrowser,
    CheckBrowser,
    Spinner,
    Clock,
    Chart,
    Progress,
    InputChoice,
    HelpView,
    Table,
    TableRow,
    SmartTable,
    Tree,
    Other(std::any::TypeId),
}

pub trait HasProps<Message> {
    fn label(self, label: &str) -> Self;
    fn fixed(self, sz: i32) -> Self;
    fn color(self, col: enums::Color) -> Self;
    fn boxtype(self, boxtype: enums::FrameType) -> Self;
    fn selection_color(self, v: enums::Color) -> Self;
    fn label_color(self, v: enums::Color) -> Self;
    fn label_font(self, v: enums::Font) -> Self;
    fn label_size(self, v: i32) -> Self;
    fn tooltip(self, v: &str) -> Self;
    fn align(self, v: enums::Align) -> Self;
    fn when(self, v: enums::CallbackTrigger) -> Self;
    fn visible(self, v: bool) -> Self;
    fn deactivate(self, v: bool) -> Self;
    fn x(self, x: i32) -> Self;
    fn y(self, x: i32) -> Self;
    fn w(self, x: i32) -> Self;
    fn h(self, x: i32) -> Self;
}

impl<Message: 'static, W: VNode<Message>> HasProps<Message> for W {
    fn label(mut self, label: &str) -> Self {
        self.wprops().label = Some(label.to_string());
        self
    }
    fn fixed(mut self, sz: i32) -> Self {
        self.wprops().fixed = Some(sz);
        self
    }
    fn color(mut self, col: enums::Color) -> Self {
        self.wprops().color = Some(col);
        self
    }
    fn boxtype(mut self, boxtype: enums::FrameType) -> Self {
        self.wprops().boxtype = Some(boxtype);
        self
    }
    fn selection_color(mut self, v: enums::Color) -> Self {
        self.wprops().selection_color = Some(v);
        self
    }
    fn label_color(mut self, v: enums::Color) -> Self {
        self.wprops().label_color = Some(v);
        self
    }
    fn label_font(mut self, v: enums::Font) -> Self {
        self.wprops().label_font = Some(v);
        self
    }
    fn label_size(mut self, v: i32) -> Self {
        self.wprops().label_size = Some(v);
        self
    }
    fn tooltip(mut self, v: &str) -> Self {
        self.wprops().tooltip = Some(v.to_string());
        self
    }
    fn align(mut self, v: enums::Align) -> Self {
        self.wprops().align = Some(v);
        self
    }
    fn when(mut self, v: enums::CallbackTrigger) -> Self {
        self.wprops().when = Some(v);
        self
    }
    fn visible(mut self, v: bool) -> Self {
        self.wprops().visible = Some(v);
        self
    }
    fn deactivate(mut self, v: bool) -> Self {
        self.wprops().deactivate = Some(v);
        self
    }
    fn x(mut self, x: i32) -> Self {
        self.wprops().x = Some(x);
        self
    }
    fn y(mut self, x: i32) -> Self {
        self.wprops().y = Some(x);
        self
    }
    fn w(mut self, x: i32) -> Self {
        self.wprops().w = Some(x);
        self
    }
    fn h(mut self, x: i32) -> Self {
        self.wprops().h = Some(x);
        self
    }
}

pub type View<Message> = Box<dyn VNode<Message>>;

pub trait VNode<Message: 'static>: Any + dyn_clone::DynClone {
    fn node_id(&self) -> usize;
    fn set_node_id(&mut self, id: usize);
    fn typ(&self) -> &VNodeType;
    fn wprops(&mut self) -> &mut WidgetProps;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn gprops(&mut self) -> Option<&mut GroupProps<Message>>;
    fn mount(&self, dom: &VirtualDom<Message>);
    fn patch(&self, old: &mut View<Message>, dom: &VirtualDom<Message>);
    fn view(self) -> View<Message>
    where
        Self: Sized,
    {
        Box::new(self)
    }
    fn assign_ids_topdown(&mut self) {
        self.set_node_id(next_id());

        if let Some(gprops) = self.gprops().as_mut() {
            for child in &mut gprops.children {
                child.assign_ids_topdown()
            }
        }
    }
}

impl<Message> std::clone::Clone for View<Message> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}
