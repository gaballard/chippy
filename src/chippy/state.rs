use crate::chippy::Settings;

///
/// Program State
///
pub struct State {
    pub rom_filename: String,
    pub debug_mode: bool,
    pub show_debug_panel: bool,
    pub _hires_mode: bool,
    pub crt_fade: bool,
    pub crt_fade_rate: u32,
    pub clock_speed: u32,
    pub target_timestep: u32,
    pub color_palette: usize,
    pub prev_frame_tick_ms: u32,
    pub cpu_timestep_ms: u32,
    pub draw_timestep_ms: u32,
    pub frame_counter: u32,
    pub draw_counter: u32,
    pub cycles: u32,
    pub fps: u32,
    pub should_execute: bool,
}

impl State {
    pub fn new(settings: &Settings) -> Self {
        let clock_speed = if settings.instruction_set.is_schip() {
            if settings.hires_mode {
                settings.schip_hires_clock_speed_hz
            } else {
                settings.schip_lores_clock_speed_hz
            }
        } else {
            settings.default_clock_speed_hz
        };

        State {
            rom_filename: String::new(),
            debug_mode: false,
            show_debug_panel: false,
            _hires_mode: settings.instruction_set.has_hires_mode(),
            crt_fade: true,
            crt_fade_rate: settings.crt_fade_rate_medium,
            clock_speed,
            target_timestep: 1_000 / clock_speed,
            color_palette: 2,
            prev_frame_tick_ms: 0,
            cpu_timestep_ms: 1_000 / clock_speed,
            draw_timestep_ms: 1_000 / settings.display_refresh_rate_hz,
            frame_counter: 0,
            draw_counter: 0,
            cycles: 0,
            fps: 0,
            should_execute: false,
        }
    }

    pub fn set_clock_speed(&mut self, clock_speed: u32) {
        self.clock_speed = clock_speed;
        self.target_timestep = 1_000 / clock_speed;
    }
}
