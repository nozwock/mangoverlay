//! Contains methods for manghud's config format handling. \
//! https://github.com/flightlessmango/MangoHud#environment-variables-mangohud_config-and-mangohud_configfile

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::Lines,
    time::Duration,
};

use egui::KeyboardShortcut;
use rgb::RGB8;

use crate::color::*;

// Defaults:
// https://github.com/flightlessmango/MangoHud/blob/6306fed7f749837f2a780c883743af3a116a5465/src/overlay_params.cpp#L587

// NOTE: Serialize bool as 0/1.
// NOTE: `gpu_mem_clock` and `gpu_mem_temp` also need `vram` to be enabled.

/// Ordered only if `legacy_layout` is set to false.
pub enum OrderableParam {
    Time,
    Version,
    Fps,
    // TODO think of a better way
}

/// https://github.com/flightlessmango/MangoHud#environment-variables-mangohud_config-and-mangohud_configfile
#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    // Performance
    fps_limit: Vec<u16>,
    fps_limit_method: FpsLimitMethod,
    vsync: Option<VSync>,
    gl_vsync: Option<u16>,
    picmip: Option<i8>, // [-16 to 16] Mip-map LoD bias
    af: Option<u8>,     // [0 to 16] Anisotropic filtering level
    bicubic: bool,      // Force bicubic filtering
    trilinear: bool,    // Force trilinear filtering
    retro: bool,        // Disable linear texture filtering

    // Core Visual
    legacy_layout: bool, // Disables default options
    preset: HudPreset,
    histogram: bool, // Replaces line graph
    custom_text_center: String,
    time: bool,          // Show current system time
    time_format: String, // Time format string
    version: bool,       // Show MangoHud version

    // GPU Info
    gpu_stats: bool,
    gpu_temp: bool,
    gpu_junction_temp: bool,
    gpu_core_clock: bool,
    gpu_mem_temp: bool,
    gpu_mem_clock: bool,
    gpu_power: bool,
    gpu_text: String,
    gpu_load_change: bool,     // Switch between colors depending on load value
    gpu_load_value: [u8; 2],   // Medium and High load values
    gpu_load_color: [RGB8; 3], // Color for before Medium, between Medium and High and after High load

    // CPU Info
    cpu_stats: bool,
    cpu_temp: bool,
    cpu_power: bool,
    cpu_text: String,
    cpu_mhz: bool,
    cpu_load_change: bool,
    cpu_load_value: [u8; 2],
    cpu_load_color: [RGB8; 3],
    core_load: bool,
    core_load_change: bool,

    // App IO
    io_read: bool,
    io_write: bool,

    // Storage Usage
    vram: bool,
    ram: bool,
    swap: bool,

    // Per Proc Memory Usage
    procmem: bool,
    procmem_shared: bool,
    procmem_virt: bool,

    // Battery Info
    battery: bool,
    battery_icon: bool,
    gamepad_battery: bool,
    gamepad_battery_icon: bool,

    // FPS Info
    fps: bool,
    fps_sampling_period: Duration, // Serialize to nanoseconds
    fps_color_change: bool,
    fps_value: [u8; 2],
    fps_color: [RGB8; 3],
    frametime: bool,
    frame_timing: bool,
    frame_count: bool,
    show_fps_limit: bool,

    // Misc Info
    throttling_status: bool,
    engine_version: bool,
    gpu_name: bool,
    vulkan_driver: bool,
    wine: bool,
    exec_name: bool,
    arch: bool,
    gamemode: bool,
    vkbasalt: bool,
    resolution: bool,
    custom_text: String,
    exec: String, // Bash command

    // Media Info
    media_player: bool,
    media_player_name: String, // FIXME use an enum?
    media_player_format: String,

    // Hud Font
    font_size: f32,
    font_scale: f32,
    font_size_text: f32,
    font_scale_media_player: f32,
    no_small_font: bool,
    font_file: PathBuf,
    font_file_text: PathBuf,
    font_glyph_ranges: Vec<String>,
    text_outline: bool,
    text_outline_thickness: f32,

    // Hud Appearance
    position: HudPosition,
    round_corners: f32,
    hud_no_margin: bool,
    hud_compact: bool,
    horizontal: bool,
    horizontal_stretch: bool, // Stretches background to screen width only if `horizontal` is set
    no_display: bool,
    offset_x: f32,
    offset_y: f32,
    width: f32,
    height: f32,
    table_columns: u8,
    cellpadding_y: f32,
    background_alpha: f32,
    alpha: f32,

    // FCAT Overlay
    fcat: bool,
    fcat_overlay_width: u16,
    fcat_screen_edge: FcatOverlayEdge,

    // Color
    text_color: RGB8,
    gpu_color: RGB8,
    cpu_color: RGB8,
    vram_color: RGB8,
    ram_color: RGB8,
    engine_color: RGB8,
    io_color: RGB8,
    frametime_color: RGB8,
    background_color: RGB8,
    media_player_color: RGB8,
    wine_color: RGB8,
    battery_color: RGB8,
    text_outline_color: RGB8,

    pci_dev: String,
    blacklist: Vec<String>, // Name of blacklisted programs
    control: String,        // Socket name

    // OpenGL Workarounds
    // gl_size_query: ?
    gl_bind_framebuffer: Option<u16>,
    // gl_dont_flip: ?

    // Keybinds
    toggle_hud: KeyboardShortcut, // FIXME write your own struct for distiction bw left and right shift
    toggle_hud_position: KeyboardShortcut,
    toggle_fps_limit: KeyboardShortcut,
    toggle_logging: KeyboardShortcut,
    reload_cfg: KeyboardShortcut,
    upload_log: KeyboardShortcut,

    // Logging
    autostart_log: bool,
    log_duration: Duration, // Serialize to seconds
    log_interval: Duration, // Serialize to miliseconds
    output_folder: PathBuf, // FIXME add default
    permit_upload: bool,
    benchmark_percentiles: String,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FpsLimitMethod {
    Early,
    #[default]
    Late,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum VSync {
    Adaptive = 0,
    #[default]
    Off = 1,
    Mailbox = 2,
    On = 3,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum HudPreset {
    #[default]
    Default = -1,
    Off = 0,
    FpsOnly = 1,
    Horizontal = 2,
    Extended = 3,
    Detailed = 4,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum HudPosition {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FcatOverlayEdge {
    #[default]
    Left = 0,
    Bottom = 1,
    Right = 2,
    Top = 3,
}

pub enum ConfigValue<T> {
    True,
    Some(T),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            legacy_layout: true,
            gpu_stats: true,
            cpu_stats: true,
            fps: true,
            frametime: true,
            frame_timing: true,
            horizontal_stretch: true,
            text_outline: true,
            fps_sampling_period: Duration::from_millis(500),
            height: 140.0,
            fps_limit: vec![0],
            background_alpha: 0.5,
            alpha: 1.0,
            fcat_overlay_width: 24,
            time_format: "%T".into(),
            fps_value: [30, 60],
            gpu_load_value: [60, 90],
            cpu_load_value: [60, 90],
            gpu_color: DARK_LIME_GREEN,
            cpu_color: BLUE,
            vram_color: LIGHT_MAGENTA,
            ram_color: LIGHT_PINK,
            engine_color: SOFT_RED,
            io_color: LIGHT_VIOLET,
            frametime_color: LIME_GREEN,
            background_color: ALMOST_BLACK,
            text_color: WHITE,
            media_player_color: WHITE,
            wine_color: SOFT_RED,
            battery_color: LIGHT_RED,
            text_outline_color: BLACK,
            fps_color: [DARK_RED, VIVID_YELLOW, GREEN],
            gpu_load_color: [GREEN, VIVID_YELLOW, DARK_RED],
            cpu_load_color: [GREEN, VIVID_YELLOW, DARK_RED],
            font_size: 24.0,
            font_size_text: 24.0,
            font_scale: 1.0,
            font_scale_media_player: 0.55,
            media_player_format: "{title};{artist};{album}".into(),
            benchmark_percentiles: "97+AVG".into(),
            cellpadding_y: -0.085,
            table_columns: 3,
            text_outline_thickness: 1.5,

            vsync: Default::default(),
            gl_vsync: Default::default(),
            fps_limit_method: Default::default(),
            picmip: Default::default(),
            af: Default::default(),
            bicubic: Default::default(),
            trilinear: Default::default(),
            retro: Default::default(),
            preset: Default::default(),
            histogram: Default::default(),
            custom_text_center: Default::default(),
            time: Default::default(),
            version: Default::default(),
            gpu_temp: Default::default(),
            gpu_junction_temp: Default::default(),
            gpu_core_clock: Default::default(),
            gpu_mem_temp: Default::default(),
            gpu_mem_clock: Default::default(),
            gpu_power: Default::default(),
            gpu_text: Default::default(),
            gpu_load_change: Default::default(),
            cpu_temp: Default::default(),
            cpu_power: Default::default(),
            cpu_text: Default::default(),
            cpu_mhz: Default::default(),
            cpu_load_change: Default::default(),
            core_load: Default::default(),
            core_load_change: Default::default(),
            io_read: Default::default(),
            io_write: Default::default(),
            vram: Default::default(),
            ram: Default::default(),
            swap: Default::default(),
            procmem: Default::default(),
            procmem_shared: Default::default(),
            procmem_virt: Default::default(),
            battery: Default::default(),
            battery_icon: Default::default(),
            gamepad_battery: Default::default(),
            gamepad_battery_icon: Default::default(),
            fps_color_change: Default::default(),
            frame_count: Default::default(),
            show_fps_limit: Default::default(),
            throttling_status: Default::default(),
            engine_version: Default::default(),
            gpu_name: Default::default(),
            vulkan_driver: Default::default(),
            wine: Default::default(),
            exec_name: Default::default(),
            arch: Default::default(),
            gamemode: Default::default(),
            vkbasalt: Default::default(),
            resolution: Default::default(),
            custom_text: Default::default(),
            exec: Default::default(),
            media_player: Default::default(),
            media_player_name: Default::default(),
            no_small_font: Default::default(),
            font_file: Default::default(),
            font_file_text: Default::default(),
            font_glyph_ranges: Default::default(),
            position: Default::default(),
            round_corners: Default::default(),
            hud_no_margin: Default::default(),
            hud_compact: Default::default(),
            horizontal: Default::default(),
            no_display: Default::default(),
            offset_x: Default::default(),
            offset_y: Default::default(),
            width: Default::default(),
            fcat: Default::default(),
            fcat_screen_edge: Default::default(),
            pci_dev: Default::default(),
            blacklist: Default::default(),
            control: Default::default(),
            gl_bind_framebuffer: Default::default(),
            toggle_hud: todo!(),
            toggle_hud_position: todo!(),
            toggle_fps_limit: todo!(),
            toggle_logging: todo!(),
            reload_cfg: todo!(),
            upload_log: todo!(),
            autostart_log: Default::default(),
            log_duration: Default::default(),
            log_interval: Default::default(),
            output_folder: Default::default(),
            permit_upload: Default::default(),
        }
    }
}

fn deserialize_by_line(lines: Lines<'_>) -> Config {
    for line in lines.map(|s| s.trim()).filter(|s| !s.starts_with('#')) {
        match line.split_once('=') {
            Some((key, value)) => todo!(),
            None => todo!(),
        }
    }
    todo!()
}

fn deserialize<P: AsRef<Path>>(file: P) -> Config {
    todo!()
}
