//! Basic signals contains winit events defined here.

use std::any::Any;
use std::path::PathBuf;

use winit::event::{DeviceId, ElementState, InnerSizeWriter, KeyEvent, Modifiers, MouseButton, MouseScrollDelta, TouchPhase, Ime, WindowEvent};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::keyboard::ModifiersKeyState;
use winit::window::Theme;

pub trait AnySignal: 'static + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Signal> AnySignal for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub trait Signal: 'static + Send + Sync {
    fn get_bound(&self) -> Option<u64> {
        None
    }
}

macro_rules! impl_signal {
    ($s: ident) => {
        impl Signal for $s {

        }
    };
}

pub struct Resized {
    pub size: PhysicalSize<u32>,
}
impl Resized {
    fn new(size: PhysicalSize<u32>) -> Self {
        Resized { size }
    }
}
impl_signal!(Resized);

pub struct Moved {
    pub position: PhysicalPosition<i32>,
}
impl Moved {
    fn new(position: PhysicalPosition<i32>) -> Self {
        Moved { position }
    }
}
impl_signal!(Moved);

pub struct CloseRequested {

}
impl CloseRequested {
    fn new() -> Self {
        CloseRequested {  }
    }
}
impl_signal!(CloseRequested);

pub struct Destroyed {

}
impl Destroyed {
    fn new() -> Self {
        Destroyed {  }
    }
}
impl_signal!(Destroyed);

pub struct DroppedFile {
    pub path: PathBuf,
}
impl DroppedFile {
    fn new(path: PathBuf) -> Self {
        DroppedFile { path }
    }
}
impl_signal!(DroppedFile);

pub struct HoveredFile{
    pub path: PathBuf,
}
impl HoveredFile {
    fn new(path: PathBuf) -> Self {
        HoveredFile { path }
    }
}
impl_signal!(HoveredFile);

pub struct HoveredFileCancelled {

}
impl HoveredFileCancelled {
    fn new() -> Self {
        HoveredFileCancelled {  }
    }
}
impl_signal!(HoveredFileCancelled);

pub struct Focused {
    pub focused: bool,
}
impl Focused {
    fn new(focused: bool) -> Self {
        Focused { focused }
    }
}
impl_signal!(Focused);

pub struct KeyboardInput {
    pub device_id: DeviceId,
    pub event: KeyEvent,
    pub is_synthetic: bool,
}
impl KeyboardInput {
    fn new(
        device_id: DeviceId,
        event: KeyEvent,
        is_synthetic: bool,
    ) -> Self {
        KeyboardInput { device_id, event, is_synthetic }
    }
}
impl_signal!(KeyboardInput);

pub struct ModifiersChanged {
    pub lshift: ModifiersKeyState,
    pub rshift: ModifiersKeyState,
    pub lalt: ModifiersKeyState,
    pub ralt: ModifiersKeyState,
    pub lcontrol: ModifiersKeyState,
    pub rcontrol: ModifiersKeyState,
    pub lsuper: ModifiersKeyState,
    pub rsuper: ModifiersKeyState,
}
impl ModifiersChanged {
    fn new(modifiers: Modifiers) -> Self {
        ModifiersChanged {
            lshift: modifiers.lshift_state(),
            rshift: modifiers.rshift_state(),
            lalt: modifiers.lalt_state(),
            ralt: modifiers.ralt_state(),
            lcontrol: modifiers.lcontrol_state(),
            rcontrol: modifiers.rcontrol_state(),
            lsuper: modifiers.lsuper_state(),
            rsuper: modifiers.rsuper_state(),
        }
    }
}
impl_signal!(ModifiersChanged);

pub struct ImeEnabled {

}
impl ImeEnabled {
    fn new() -> Self {
        ImeEnabled {  }
    }
}
impl_signal!(ImeEnabled);

pub struct ImeEdited {
    pub edit: String,
    pub cursor: Option<(usize, usize)>,
}
impl ImeEdited {
    fn new(edit: String, cursor: Option<(usize, usize)>) -> Self {
        ImeEdited { edit, cursor }
    }
}
impl_signal!(ImeEdited);

pub struct ImeCommitted {
    pub text: String,
}
impl ImeCommitted {
    fn new(text: String) -> Self {
        ImeCommitted { text }
    }
}
impl_signal!(ImeCommitted);

pub struct ImeDisabled {

}
impl ImeDisabled {
    fn new() -> Self {
        ImeDisabled {  }
    }
}
impl_signal!(ImeDisabled);

pub struct CursorMoved {
    pub device_id: DeviceId,
    pub position: PhysicalPosition<f64>,
}
impl CursorMoved {
    fn new(device_id: DeviceId, position: PhysicalPosition<f64>) -> Self {
        CursorMoved { device_id, position }
    }
}
impl_signal!(CursorMoved);

pub struct CursorEntered {
    pub device_id: DeviceId,
}
impl CursorEntered {
    fn new(device_id: DeviceId) -> Self {
        CursorEntered { device_id }
    }
}
impl_signal!(CursorEntered);

pub struct CursorLeft {
    pub device_id: DeviceId,
}
impl CursorLeft {
    fn new(device_id: DeviceId) -> Self {
        CursorLeft { device_id }
    }
}
impl_signal!(CursorLeft);

pub struct MouseWheel {
    pub device_id: DeviceId,
    pub delta: MouseScrollDelta,
    pub phase: TouchPhase,
}
impl MouseWheel {
    fn new(
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    ) -> Self {
        MouseWheel { device_id, delta, phase }
    }
}
impl_signal!(MouseWheel);

pub struct MouseInput {
    pub device_id: DeviceId,
    pub state: ElementState,
    pub button: MouseButton,
}
impl MouseInput {
    fn new(
        device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    ) -> Self {
        MouseInput { device_id, state, button }
    }
}
impl_signal!(MouseInput);

pub struct ScaleFactorChanged {
    pub scale_factor: f64,
    pub inner_size_writer: InnerSizeWriter,
}
impl ScaleFactorChanged {
    fn new(scale_factor: f64, inner_size_writer: InnerSizeWriter) -> Self {
        ScaleFactorChanged { scale_factor, inner_size_writer }
    }
}
impl_signal!(ScaleFactorChanged);

pub struct ThemeChanged {
    pub theme: Theme,
}
impl ThemeChanged {
    fn new(theme: Theme) -> Self {
        ThemeChanged { theme }
    }
}
impl_signal!(ThemeChanged);

pub fn window_event_to_signal(event: WindowEvent) -> Option<Box<dyn Signal>> {
    match event {
        WindowEvent::Resized(size) => {
            Some(Box::new(Resized::new(size)))
        }
        WindowEvent::Moved(position) => {
            Some(Box::new(Moved::new(position)))
        }
        WindowEvent::CloseRequested => {
            Some(Box::new(CloseRequested::new()))
        }
        WindowEvent::Destroyed => {
            Some(Box::new(Destroyed::new()))
        }
        WindowEvent::DroppedFile(path) => {
            Some(Box::new(DroppedFile::new(path)))
        }
        WindowEvent::HoveredFile(path) => {
            Some(Box::new(HoveredFile::new(path)))
        }
        WindowEvent::HoveredFileCancelled => {
            Some(Box::new(HoveredFileCancelled::new()))
        }
        WindowEvent::Focused(focused) => {
            Some(Box::new(Focused::new(focused)))
        }
        WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
            Some(Box::new(KeyboardInput::new(device_id, event, is_synthetic)))
        }
        WindowEvent::ModifiersChanged(modifiers) => {
            Some(Box::new(ModifiersChanged::new(modifiers)))
        }
        WindowEvent::Ime(ime) => {
            match ime {
                Ime::Enabled => {
                    Some(Box::new(ImeEnabled::new()))
                }
                Ime::Preedit(edit, cursor) => {
                    Some(Box::new(ImeEdited::new(edit, cursor)))
                }
                Ime::Commit(text) => {
                    Some(Box::new(ImeCommitted::new(text)))
                }
                Ime::Disabled => {
                    Some(Box::new(ImeDisabled::new()))
                }
            }
        }
        WindowEvent::CursorMoved { device_id, position } => {
            Some(Box::new(CursorMoved::new(device_id, position)))
        }
        WindowEvent::CursorEntered { device_id } => {
            Some(Box::new(CursorEntered::new(device_id)))
        }
        WindowEvent::CursorLeft { device_id } => {
            Some(Box::new(CursorLeft::new(device_id)))
        }
        WindowEvent::MouseWheel { device_id, delta, phase } => {
            Some(Box::new(MouseWheel::new(device_id, delta, phase)))
        }
        WindowEvent::MouseInput { device_id, state, button } => {
            Some(Box::new(MouseInput::new(device_id, state, button)))
        }
        WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {
            Some(Box::new(ScaleFactorChanged::new(scale_factor, inner_size_writer)))
        }
        WindowEvent::ThemeChanged(theme) => {
            Some(Box::new(ThemeChanged::new(theme)))
        }
        _ => {
            None
        }
    }
}
