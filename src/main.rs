mod chip8;
mod chippy;
mod constants;
mod fonts;
mod platform;
mod utils;

use env_logger;
use log::debug;

use platform::Platform;
use std::env;

use crate::chippy::*;

///
/// Main
///
fn main() {
    env_logger::try_init().expect("Couldn't load env_logger");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must include path to ROM file!");
    }

    let settings = Settings::from(".").expect("Unable to open settings file");

    // Set up SDL context
    let platform = Platform::new();

    // Load resources
    let font = platform
        .get_ttf_context()
        .load_font(settings.font_path.as_str(), settings.font_size)
        .expect("Font does not exist");

    let mut chippy = Chippy::new(&platform, settings, &font);

    println!("There's my Chippy.");

    chippy.load_program(args[1].to_owned());

    println!(
        "Running program using the {} instruction set.",
        chippy.settings.instruction_set.get_title()
    );

    // Main loop
    'emulate: loop {
        chippy.start_frame();

        // Check inputs
        match chippy.devices.keypad.read_host_keypad(&chippy.settings) {
            Actions::Running => {}
            Actions::Quit => break 'emulate,
            Actions::Reset => Actions::handle_reset_action(&mut chippy),
            Actions::ToggleDebugMode => Actions::handle_toggle_debug_mode_action(&mut chippy),
            Actions::Step => chippy.state.should_execute = true,
            Actions::ToggleDebugPanel => Actions::handle_toggle_debug_panel_action(&mut chippy),
            Actions::CycleColorPalette => Actions::handle_cycle_color_palette_action(&mut chippy),
            Actions::ToggleCrtFade => Actions::handle_toggle_crt_fade_action(&mut chippy),
            Actions::SetCrtFadeSpeed(fade_rate) => {
                Actions::handle_set_crt_fade_rate_action(&mut chippy, fade_rate)
            }
            Actions::IncZoomLevel => Actions::handle_inc_zoom_level(&mut chippy),
            Actions::DecZoomLevel => Actions::handle_dec_zoom_level(&mut chippy),
            Actions::ScrollScreen(dir, amount) => Actions::handle_scroll(&mut chippy, dir, amount),
        }

        chippy.refresh_input_buffer();

        // Handle debug mode
        if chippy.state.debug_mode {
            if !chippy.state.should_execute {
                chippy.debug_step();
                continue;
            }
            debug!("Executing frame {}...", chippy.devices.cpu.cycle);
        }

        let old_hires_mode = chippy.devices.cpu.vram.hires_mode;

        // Execute cycle
        chippy.devices.cpu.tick();

        if chippy.settings.instruction_set.is_schip() && chippy.devices.cpu.quit_flag {
            break 'emulate;
        }

        // SCHIP-1.x defines clock speed changes on lo/hi res modes
        if chippy.settings.instruction_set.is_schip()
            && chippy.devices.cpu.vram.hires_mode != old_hires_mode
        {
            let clock_speed = if chippy.settings.instruction_set.is_schip() {
                if chippy.devices.cpu.vram.hires_mode {
                    chippy.settings.schip_hires_clock_speed_hz
                } else {
                    chippy.settings.schip_lores_clock_speed_hz
                }
            } else {
                chippy.settings.default_clock_speed_hz
            };
            if clock_speed != chippy.state.clock_speed {
                debug!("Setting clock speed to: {} hz", clock_speed);
                chippy.state.set_clock_speed(clock_speed);
            }
        }

        chippy.play_audio();

        let dt = chippy.resources.frame_timer.ticks() - chippy.state.prev_frame_tick_ms;

        chippy.refresh_screen(dt);
        chippy.refresh_counters(dt);
        chippy.end_frame(dt);
    }

    println!("Thanks for playing!");
}
