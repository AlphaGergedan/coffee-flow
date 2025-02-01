use std::{thread, time};

mod screen;
mod smoke_grid;

use screen::*;
use smoke_grid::*;

fn wait(sec_to_wait: f64, current_sec: u64) {
    println!("{}...", current_sec);
    thread::sleep(time::Duration::from_secs_f64(sec_to_wait));
}

fn main() {
    // Don't touch width and height, they are configured accordingly to the mug
    let width = 25;
    let height = 26;

    // Decays the fixed boundary condition at grid[0][x] near the mug
    // to simulate dissipative effects.
    let decay_rate = 0.0001;

    // This is the continuous grid where the diffusion is simulated
    let mut smoke = SmokeGrid::new(width, height, decay_rate);

    // Delta time of the finite differences
    let dt = 0.1;

    // Diffusion rate in the heat PDE: u_t = diff_rate * u_xx
    let diff_rate = 0.5;

    // Current steps simulated
    let mut step = 0;

    // Seconds to wait before simulating next time step
    // FIXME: remove this and save the solution instead, then declare something like `print_every`
    let secs_to_wait = 0.01;

    // Print fake smoke first for debugging
    clear_screen();
    print_fake_smoke();
    println!();
    print_mug();
    wait(1.0, step);
    step += 1;

    // Print initial condition
    clear_screen();
    //smoke.display_debug();
    smoke.display();
    print_mug();
    wait(secs_to_wait, step);
    step += 1;

    loop {
        // Simulate diffusion
        clear_screen();
        // Diffuse for 1 second
        for _ in 0..9 {
            smoke.diffuse(dt, diff_rate);
        }
        smoke.display();
        //smoke.display_debug();
        print_mug();

        wait(secs_to_wait, step);
        step += 1;
    }
}
