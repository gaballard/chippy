use crate::{chip8::ScrollDir, chippy::Chippy, constants};

///
/// Program Actions
///
#[derive(PartialEq)]
pub enum Actions {
    Quit,
    Reset,
    Running,
    ToggleDebugMode,
    ToggleDebugPanel,
    CycleColorPalette,
    Step,
    ToggleCrtFade,
    SetCrtFadeSpeed(u32),
    IncZoomLevel,
    DecZoomLevel,
    ScrollScreen(ScrollDir, usize),
}

impl Actions {
    pub fn handle_reset_action(chippy: &mut Chippy) {
        chippy.devices.cpu.reset();
        chippy.devices.cpu.load_program(&chippy.devices.tape.data);
        chippy.state.debug_mode = false;
    }

    pub fn handle_toggle_debug_panel_action(chippy: &mut Chippy) {
        chippy.state.show_debug_panel = !chippy.state.show_debug_panel;
        chippy
            .devices
            .display
            .debug_panel
            .set_visibility(chippy.state.show_debug_panel);
    }

    pub fn handle_cycle_color_palette_action(chippy: &mut Chippy) {
        chippy.settings.color_palette = chippy.settings.color_palette.wrapping_add(1);
        if chippy.settings.color_palette as usize >= constants::PALETTE_COLORS.len() {
            chippy.settings.color_palette = 0;
        }
        chippy
            .devices
            .display
            .set_color_palette(chippy.settings.color_palette);
        chippy
            .devices
            .display
            .debug_panel
            .set_color_palette(chippy.settings.color_palette);
    }

    pub fn handle_set_crt_fade_rate_action(chippy: &mut Chippy, fade_rate: u32) {
        println!("Setting CRT fade rate to: {fade_rate}");
        chippy.state.crt_fade_rate = fade_rate;
    }

    pub fn handle_toggle_debug_mode_action(chippy: &mut Chippy) {
        chippy.state.debug_mode = !chippy.state.debug_mode
    }

    pub fn handle_toggle_crt_fade_action(chippy: &mut Chippy) {
        chippy.state.crt_fade = !chippy.state.crt_fade
    }

    pub fn handle_inc_zoom_level(chippy: &mut Chippy) {
        let new_zoom = chippy.devices.display.zoom_level.saturating_add(1);
        chippy.devices.display.zoom_level = if new_zoom <= constants::MAX_ZOOM_LEVEL {
            new_zoom
        } else {
            constants::MAX_ZOOM_LEVEL
        };
    }

    pub fn handle_dec_zoom_level(chippy: &mut Chippy) {
        let new_zoom = chippy.devices.display.zoom_level.saturating_sub(1);
        chippy.devices.display.zoom_level = if new_zoom >= constants::MIN_ZOOM_LEVEL {
            new_zoom
        } else {
            constants::MIN_ZOOM_LEVEL
        };
    }

    pub fn handle_scroll(chippy: &mut Chippy, dir: ScrollDir, amount: usize) {
        match dir {
            ScrollDir::Down => chippy.devices.cpu.scd(amount as u8),
            ScrollDir::Up => chippy.devices.cpu.scu(amount as u8),
            ScrollDir::Right => chippy.devices.cpu.scr(),
            ScrollDir::Left => chippy.devices.cpu.scl(),
        }
    }
}
