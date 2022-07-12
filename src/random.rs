use crate::progress::Bar;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use term_size;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{DeviceEvent, Event},
    event_loop::{ControlFlow, EventLoop},
};

const MOUSE_TICK_COUNT: usize = 1000;
const MOUSE_TICK_DELTA: usize = 100;

pub fn randomize(x: u32, y: u32, seed: u64) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(seed);
    { 0..x }.map(|_| rng.gen_range(1..=y)).collect()
}

// pub async fn mouse_capture() {}

pub fn get_seed() -> u64 {
    let mut event_loop = EventLoop::new();
    let mut seed: u64 = 0;
    let mut tick_counter: usize = 0;
    let mut delta_accum: (f64, f64) = (0., 0.);

    let (terminal_width, _) = term_size::dimensions().unwrap();

    let mut bar = Bar::new(MOUSE_TICK_COUNT, terminal_width);

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                delta_accum = add(delta_accum, delta);
                tick_counter += 1;
                if tick_counter % MOUSE_TICK_DELTA == 0 {
                    seed ^= (delta_accum.0 / delta_accum.1).to_bits();
                    delta_accum = (0., 0.);
                }
                bar.inc();

                if tick_counter == MOUSE_TICK_COUNT {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
    seed
}

fn add(delta1: (f64, f64), delta2: (f64, f64)) -> (f64, f64) {
    (delta1.0 + delta2.0, delta1.1 + delta2.1)
}
