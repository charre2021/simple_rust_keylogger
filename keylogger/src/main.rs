use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, ElementState, RawKeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents, EventLoop};
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::WindowId;

struct Keylogger;

impl ApplicationHandler for Keylogger {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.listen_device_events(DeviceEvents::Always);
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {
        todo!()
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::Key(RawKeyEvent {
                physical_key,
                state,
            }) if state == ElementState::Pressed => {
                println!("{}", physical_key.to_scancode().unwrap())
            }
            _ => {}
        }
    }
}

fn main() {
    let _ = EventLoop::new().unwrap().run_app(&mut Keylogger);
}
