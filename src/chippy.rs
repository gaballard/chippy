mod actions;
mod instruction_sets;
mod settings;
mod state;

use std::time::Duration;

use sdl2::{TimerSubsystem, ttf::Font};

pub use actions::*;
pub use instruction_sets::*;
pub use settings::*;

use crate::{
    chip8::Cpu,
    chippy::state::State,
    platform::{Audio, Display, Keypad, Platform, Tape},
};

pub struct Resources {
    pub frame_timer: TimerSubsystem,
}

pub struct HardwareDevices<'a> {
    pub tape: Tape,
    pub display: Display<'a>,
    pub audio: Audio,
    pub keypad: Keypad,
    pub cpu: Cpu,
}

pub struct Chippy<'a> {
    pub settings: Settings,
    pub state: State,
    pub resources: Resources,
    pub devices: HardwareDevices<'a>,
}

///
/// Chippy The Program
///
impl<'a> Chippy<'a> {
    pub fn new(platform: &'a Platform, settings: Settings, font: &'a Font<'a, 'static>) -> Self {
        let state = State::new(&settings);

        // Init resources
        let frame_timer = platform
            .get_sdl_context()
            .timer()
            .expect("FPS delay timer failed to initialize in main");

        let resources = Resources { frame_timer };

        // Init hardware
        let mut devices = HardwareDevices {
            tape: Tape::new(),
            display: Display::new(
                &platform,
                settings.zoom_level,
                settings.instruction_set,
                &font,
                state.color_palette,
            ),
            audio: Audio::new(&platform.get_sdl_context(), settings.beep_freq_hz),
            keypad: Keypad::new(&platform.get_sdl_context()),
            cpu: Cpu::new(settings.instruction_set),
        };

        devices.cpu.vram.hires_mode = settings.hires_mode;

        Chippy {
            settings,
            state,
            resources,
            devices,
        }
    }

    pub fn load_program(&mut self, rom_filename: String) {
        self.state.rom_filename = rom_filename;
        self.devices.tape.read(&self.state.rom_filename);
        self.devices.cpu.load_program(&self.devices.tape.data);
    }

    pub fn play_audio(&self) {
        if self.devices.cpu.sound_timer > 0 {
            self.devices.audio.start_beep();
        } else {
            self.devices.audio.stop_beep()
        }
    }

    pub fn debug_step(&mut self) {
        // Clear frame buffer
        self.devices.display.reset();
        self.devices.display.debug_panel.reset();
        self.devices.cpu.dec_timers();
        self.draw_screen();
    }

    pub fn start_frame(&mut self) {
        self.state.prev_frame_tick_ms = self.resources.frame_timer.ticks();
        self.state.should_execute = false;
        self.state.cycles = self.state.cycles.wrapping_add(1);
    }

    pub fn end_frame(&mut self, dt: u32) {
        if dt < self.state.cpu_timestep_ms {
            std::thread::sleep(Duration::from_millis(
                (self.state.cpu_timestep_ms - dt).into(),
            ));
        }
    }

    ///
    /// Refresh Screen
    ///
    /// Runs @ display refresh hz
    ///
    pub fn refresh_screen(&mut self, dt: u32) {
        self.state.draw_counter =
            self.state
                .draw_counter
                .saturating_add(if dt == 0 { 1 } else { dt });
        if self.state.draw_counter >= self.state.draw_timestep_ms {
            self.devices.cpu.dec_timers();
            self.devices.display.reset();
            self.devices.display.debug_panel.reset();
            self.draw_screen();
            self.state.fps += 1;
            self.state.draw_counter = 0;
        }
    }

    ///
    /// Refresh Debug Counters
    ///
    /// Runs once per second
    ///
    pub fn refresh_counters(&mut self, dt: u32) {
        self.state.frame_counter =
            self.state
                .frame_counter
                .saturating_add(if dt == 0 { 1 } else { dt });
        if self.state.frame_counter >= 1_000 {
            self.devices.display.debug_panel.fps = self.state.fps;
            self.devices.display.debug_panel.cycles_per_frame = self.state.cycles / self.state.fps;
            self.state.cycles = 0;
            self.state.fps = 0;
            self.state.frame_counter = 0;
        }
    }

    pub fn draw_screen(&mut self) {
        self.devices.display.debug_panel.update(&self.devices.cpu);
        self.devices.display.update(
            &mut self.devices.cpu,
            self.state.crt_fade,
            self.state.crt_fade_rate as u8,
        );
    }

    pub fn refresh_input_buffer(&mut self) {
        self.devices
            .keypad
            .read_keypad(self.devices.cpu.keys.get_buffer());
    }
}
