use fltk::{
    browser::*, button::*, frame::Frame, group::*, input::*, menu::*, misc::*, output::*,
    prelude::WidgetExt, table::*, text::*, tree::*, valuator::*, widget::Widget,
};
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

pub trait IsWidget: Any {
    fn as_widget(&self) -> Widget;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W: WidgetExt + 'static> IsWidget for W {
    fn as_widget(&self) -> Widget {
        unsafe { self.into_widget() }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
pub enum WidgetUnion {
    Column(Flex),
    Row(Flex),
    Button(Button),
    Frame(Frame),
    Input(Input),
    MenuBar(MenuBar),
    RadioButton(RadioButton),
    ToggleButton(ToggleButton),
    RoundButton(RoundButton),
    CheckButton(CheckButton),
    LightButton(LightButton),
    RepeatButton(RepeatButton),
    RadioLightButton(RadioLightButton),
    RadioRoundButton(RadioRoundButton),
    ReturnButton(ReturnButton),
    ShortcutButton(ShortcutButton),
    Group(Group),
    Pack(Pack),
    HorPack(Pack),
    Tabs(Tabs),
    Scroll(Scroll),
    Tile(Tile),
    Wizard(Wizard),
    ColorChooser(ColorChooser),
    Grid(Grid),
    TextDisplay(TextDisplay),
    TextEditor(TextEditor),
    IntInput(IntInput),
    FloatInput(FloatInput),
    MultilineInput(MultilineInput),
    SecretInput(SecretInput),
    FileInput(FileInput),
    Output(Output),
    MultilineOutput(MultilineOutput),
    Choice(Choice),
    MenuButton(MenuButton),
    SysMenuBar(SysMenuBar),
    Slider(Slider),
    NiceSlider(NiceSlider),
    ValueSlider(ValueSlider),
    Dial(Dial),
    LineDial(LineDial),
    Counter(Counter),
    Scrollbar(Scrollbar),
    HorScrollbar(Scrollbar),
    Roller(Roller),
    Adjuster(Adjuster),
    ValueInput(ValueInput),
    ValueOutput(ValueOutput),
    FillSlider(FillSlider),
    FillDial(FillDial),
    HorSlider(HorSlider),
    HorFillSlider(HorFillSlider),
    HorNiceSlider(HorNiceSlider),
    HorValueSlider(HorValueSlider),
    Browser(Browser),
    SelectBrowser(SelectBrowser),
    HoldBrowser(HoldBrowser),
    MultiBrowser(MultiBrowser),
    FileBrowser(FileBrowser),
    CheckBrowser(CheckBrowser),
    Spinner(Spinner),
    Clock(Clock),
    Chart(Chart),
    Progress(Progress),
    InputChoice(InputChoice),
    HelpView(HelpView),
    Table(Table),
    TableRow(TableRow),
    Tree(Tree),
    Other(Rc<dyn IsWidget>),
}

impl WidgetUnion {
    pub(crate) fn view(&self) -> Widget {
        match self {
            WidgetUnion::Column(w) => w.as_widget(),
            WidgetUnion::Row(w) => w.as_widget(),
            WidgetUnion::Button(w) => w.as_widget(),
            WidgetUnion::Frame(w) => w.as_widget(),
            WidgetUnion::Input(w) => w.as_widget(),
            WidgetUnion::MenuBar(w) => w.as_widget(),
            WidgetUnion::RadioButton(w) => w.as_widget(),
            WidgetUnion::ToggleButton(w) => w.as_widget(),
            WidgetUnion::RoundButton(w) => w.as_widget(),
            WidgetUnion::CheckButton(w) => w.as_widget(),
            WidgetUnion::LightButton(w) => w.as_widget(),
            WidgetUnion::RepeatButton(w) => w.as_widget(),
            WidgetUnion::RadioLightButton(w) => w.as_widget(),
            WidgetUnion::RadioRoundButton(w) => w.as_widget(),
            WidgetUnion::ReturnButton(w) => w.as_widget(),
            WidgetUnion::ShortcutButton(w) => w.as_widget(),
            WidgetUnion::Group(w) => w.as_widget(),
            WidgetUnion::Pack(w) => w.as_widget(),
            WidgetUnion::HorPack(w) => w.as_widget(),
            WidgetUnion::Tabs(w) => w.as_widget(),
            WidgetUnion::Scroll(w) => w.as_widget(),
            WidgetUnion::Tile(w) => w.as_widget(),
            WidgetUnion::Wizard(w) => w.as_widget(),
            WidgetUnion::ColorChooser(w) => w.as_widget(),
            WidgetUnion::Grid(w) => w.as_widget(),
            WidgetUnion::TextDisplay(w) => w.as_widget(),
            WidgetUnion::TextEditor(w) => w.as_widget(),
            WidgetUnion::IntInput(w) => w.as_widget(),
            WidgetUnion::FloatInput(w) => w.as_widget(),
            WidgetUnion::MultilineInput(w) => w.as_widget(),
            WidgetUnion::SecretInput(w) => w.as_widget(),
            WidgetUnion::FileInput(w) => w.as_widget(),
            WidgetUnion::Output(w) => w.as_widget(),
            WidgetUnion::MultilineOutput(w) => w.as_widget(),
            WidgetUnion::Choice(w) => w.as_widget(),
            WidgetUnion::SysMenuBar(w) => w.as_widget(),
            WidgetUnion::MenuButton(w) => w.as_widget(),
            WidgetUnion::Slider(w) => w.as_widget(),
            WidgetUnion::NiceSlider(w) => w.as_widget(),
            WidgetUnion::ValueSlider(w) => w.as_widget(),
            WidgetUnion::Dial(w) => w.as_widget(),
            WidgetUnion::LineDial(w) => w.as_widget(),
            WidgetUnion::Counter(w) => w.as_widget(),
            WidgetUnion::Scrollbar(w) => w.as_widget(),
            WidgetUnion::HorScrollbar(w) => w.as_widget(),
            WidgetUnion::Roller(w) => w.as_widget(),
            WidgetUnion::Adjuster(w) => w.as_widget(),
            WidgetUnion::ValueInput(w) => w.as_widget(),
            WidgetUnion::ValueOutput(w) => w.as_widget(),
            WidgetUnion::FillSlider(w) => w.as_widget(),
            WidgetUnion::FillDial(w) => w.as_widget(),
            WidgetUnion::HorSlider(w) => w.as_widget(),
            WidgetUnion::HorFillSlider(w) => w.as_widget(),
            WidgetUnion::HorNiceSlider(w) => w.as_widget(),
            WidgetUnion::HorValueSlider(w) => w.as_widget(),
            WidgetUnion::Browser(w) => w.as_widget(),
            WidgetUnion::SelectBrowser(w) => w.as_widget(),
            WidgetUnion::HoldBrowser(w) => w.as_widget(),
            WidgetUnion::MultiBrowser(w) => w.as_widget(),
            WidgetUnion::FileBrowser(w) => w.as_widget(),
            WidgetUnion::CheckBrowser(w) => w.as_widget(),
            WidgetUnion::Spinner(w) => w.as_widget(),
            WidgetUnion::Clock(w) => w.as_widget(),
            WidgetUnion::Chart(w) => w.as_widget(),
            WidgetUnion::Progress(w) => w.as_widget(),
            WidgetUnion::InputChoice(w) => w.as_widget(),
            WidgetUnion::HelpView(w) => w.as_widget(),
            WidgetUnion::Table(w) => w.as_widget(),
            WidgetUnion::TableRow(w) => w.as_widget(),
            WidgetUnion::Tree(w) => w.as_widget(),
            WidgetUnion::Other(w) => w.as_widget(),
        }
    }
}

pub type WidgetMap = HashMap<usize, WidgetUnion>;
