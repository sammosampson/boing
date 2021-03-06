use crate::prelude::*;

pub fn create_system_event_loop() -> SystemEventLoop {
    SystemEventLoop {
        inner: EventLoop::new()
    }
}

pub struct SystemEventLoop {
    inner: EventLoop<()>
}

impl SystemEventLoop {
    pub fn get_loop(&self) -> &EventLoop<()> {
        &self.inner
    }

    pub fn run(
        &mut self,
        event_producer: &mut SystemEventProducer,
        event_channel: &mut SystemEventChannel
    ) {
        self.inner.run_return(|event, _, flow| {
            match event {
                Event::WindowEvent { window_id: _, event} => {
                    match event {
                        WindowEvent::CloseRequested => event_producer.push(SystemEvent::CloseRequested),
                        WindowEvent::KeyboardInput { input, device_id: _, is_synthetic: _} => {
                            event_producer.push(SystemEvent::KeyboardAction { state: input.state.into(), button: input.virtual_keycode.into() });
                        },
                        _ => {} 
                    }
                    
                },
                _ => {}
            }
            *flow = glium::glutin::event_loop::ControlFlow::Exit;
        });

        event_producer.drain_to(event_channel);
    }
}
