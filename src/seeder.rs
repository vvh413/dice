use rand::seq::SliceRandom;
use crate::mat::Mat;
use crate::progress::Bar;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{DeviceEvent, Event},
    event_loop::{ControlFlow, EventLoop},
};
use rand::{rngs::StdRng, SeedableRng};

static mut DELTA_BUFFER: Vec<(f64, f64)> = Vec::new();
static mut SEED_BUFFER: Vec<[u8; 32]> = Vec::new();
pub static mut BAR: Bar = Bar::default();
pub static mut SEED: [u8; 32] = [0; 32];

pub async fn get_seed() {
    tokio::spawn(multiply_delta());
    tokio::spawn(compute_seed());
    mouse_capture().await;
}

async fn mouse_capture() {
    let mut event_loop = EventLoop::new();
    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => unsafe {
                DELTA_BUFFER.push(delta);
                BAR.inc();
                if BAR.complete() {
                    *control_flow = ControlFlow::Exit;
                }
            },
            _ => (),
        }
    });
}

async fn multiply_delta() {
    let mut mat = Mat::new();
    let mut delta_accum: (f64, f64) = (0., 0.);

    unsafe {
        while !BAR.complete() {
            if DELTA_BUFFER.len() == 0 {
                continue;
            }
            let delta = DELTA_BUFFER.pop().unwrap();
            delta_accum = add(delta_accum, delta);
                    
            if BAR.step_complite() {
                if mat.set(delta_accum) {
                    let mult = mat.mult();
                    let vec = transform(mult);
                    SEED_BUFFER.push(vec);
                    mat.reset();
                }
                delta_accum = (0., 0.);
            }
        }
    }
}

async fn compute_seed() {
    let mut rng = StdRng::from_entropy();
    unsafe {
        while !BAR.complete() {
            if SEED_BUFFER.len() == 0 {
                continue;
            }
            let mut seed_value = SEED_BUFFER.pop().unwrap();
            seed_value.shuffle(&mut rng);
            SEED = vec_xor(SEED, seed_value);
        }
    }
}

fn vec_xor(mut vec1: [u8; 32], vec2: [u8; 32]) -> [u8; 32] {
    for i in 0..32 {
        vec1[i] ^= vec2[i];
    }
    vec1
}

fn transform(vec: [f64; 4]) -> [u8; 32] {
    vec.iter()
        .map(|x| x.to_ne_bytes())
        .flatten()
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

fn add(delta1: (f64, f64), delta2: (f64, f64)) -> (f64, f64) {
    (delta1.0 + delta2.0, delta1.1 + delta2.1)
}
