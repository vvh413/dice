use crate::constants::MOUSE_TICK_COUNT;
use crate::mat::Mat;
use crate::progress::Bar;
use rand::seq::SliceRandom;
use std::sync::Arc;
use std::sync::Mutex;

use rand::{rngs::StdRng, SeedableRng};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{DeviceEvent, Event},
    event_loop::{ControlFlow, EventLoop},
};

struct Seeder {
    delta_buffer: Vec<(f64, f64)>,
    seed_buffer: Vec<[u8; 32]>,
    bar: Bar,
    seed: [u8; 32],
}

impl Seeder {
    fn new() -> Self {
        Seeder {
            delta_buffer: Vec::new(),
            seed_buffer: Vec::new(),
            bar: Bar::new(MOUSE_TICK_COUNT),
            seed: [0; 32],
        }
    }
}

pub async fn get_seed() -> [u8; 32] {
    let seeder = Arc::new(Mutex::new(Seeder::new()));

    tokio::spawn(multiply_delta(seeder.clone()));
    tokio::spawn(compute_seed(seeder.clone()));
    mouse_capture(seeder.clone()).await;
    let seeder = seeder.lock().unwrap();
    seeder.seed
}

async fn mouse_capture(seeder: Arc<Mutex<Seeder>>) {
    let mut event_loop = EventLoop::new();

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                let mut seeder = seeder.lock().unwrap();

                seeder.delta_buffer.push(delta);
                seeder.bar.inc();
                if seeder.bar.complete() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

async fn multiply_delta(seeder: Arc<Mutex<Seeder>>) {
    let mut mat = Mat::new();
    let mut delta_accum: (f64, f64) = (0., 0.);

    loop {
        let mut seeder = seeder.lock().unwrap();
        if seeder.bar.complete() {
            break;
        }
        if seeder.delta_buffer.len() == 0 {
            continue;
        }
        let delta = seeder.delta_buffer.pop().unwrap();
        delta_accum = add(delta_accum, delta);

        if seeder.bar.step_complete() {
            if mat.set(delta_accum) {
                let mult = mat.mult();
                let vec = transform(mult);
                seeder.seed_buffer.push(vec);
                mat.reset();
            }
            delta_accum = (0., 0.);
        }
    }
}

async fn compute_seed(seeder: Arc<Mutex<Seeder>>) {
    let mut rng = StdRng::from_entropy();
    loop {
        let mut seeder = seeder.lock().unwrap();
        if seeder.bar.complete() {
            break;
        }
        if seeder.seed_buffer.len() == 0 {
            continue;
        }
        let mut seed_value = seeder.seed_buffer.pop().unwrap();
        seed_value.shuffle(&mut rng);
        seeder.seed = vec_xor(seeder.seed, seed_value);
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
