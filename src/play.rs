use dioxus::prelude::*;
use std::f32::consts::PI;

#[cfg(target_arch = "wasm32")]
use base64::Engine;

const SAMPLE_RATE: usize = 48_000;
const BLOCK_SIZE: usize = 256;
const STEPS_PER_BAR: usize = 16;
const GATE_RATIO: f32 = 0.72;
const MASTER_GAIN: f32 = 0.24;
const MAX_VOICES: usize = 16;
const AUTOMATION_LEVELS: [Option<f32>; 6] = [
    None,
    Some(0.18),
    Some(0.34),
    Some(0.52),
    Some(0.74),
    Some(0.94),
];
const PITCH_CHOICES: [u8; 8] = [36, 43, 48, 50, 55, 60, 62, 67];
const BPM_CHOICES: [u32; 5] = [88, 104, 112, 124, 136];
const BAR_CHOICES: [usize; 3] = [1, 2, 4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LivePatchId {
    MoltenHorizon,
    CathedralBloom,
    EmberVault,
    RazorThaw,
    GravityWake,
    FurnaceChoir,
    GranitePlain,
    GlassTide,
}

impl LivePatchId {
    pub const ALL: [Self; 8] = [
        Self::MoltenHorizon,
        Self::CathedralBloom,
        Self::EmberVault,
        Self::RazorThaw,
        Self::GravityWake,
        Self::FurnaceChoir,
        Self::GranitePlain,
        Self::GlassTide,
    ];

    fn preset(self) -> &'static DemoPatchPreset {
        match self {
            Self::MoltenHorizon => &MOLTEN_HORIZON,
            Self::CathedralBloom => &CATHEDRAL_BLOOM,
            Self::EmberVault => &EMBER_VAULT,
            Self::RazorThaw => &RAZOR_THAW,
            Self::GravityWake => &GRAVITY_WAKE,
            Self::FurnaceChoir => &FURNACE_CHOIR,
            Self::GranitePlain => &GRANITE_PLAIN,
            Self::GlassTide => &GLASS_TIDE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MacroTarget {
    Gravitacija,
    Bloom,
    Heat,
    Ruin,
    Swarm,
}

impl MacroTarget {
    pub const ALL: [Self; 5] = [
        Self::Gravitacija,
        Self::Bloom,
        Self::Heat,
        Self::Ruin,
        Self::Swarm,
    ];

    fn as_index(self) -> usize {
        match self {
            Self::Gravitacija => 0,
            Self::Bloom => 1,
            Self::Heat => 2,
            Self::Ruin => 3,
            Self::Swarm => 4,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Gravitacija => "Gravitacija",
            Self::Bloom => "Bloom",
            Self::Heat => "Heat",
            Self::Ruin => "Ruin",
            Self::Swarm => "Swarm",
        }
    }

    fn blurb(self) -> &'static str {
        match self {
            Self::Gravitacija => "Pulls the body inward and thickens the center.",
            Self::Bloom => "Opens air, width, and upper lift.",
            Self::Heat => "Pushes drive, pressure, and body weight.",
            Self::Ruin => "Adds rupture, noise, and rough edges.",
            Self::Swarm => "Widens detune motion and stereo drift.",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Step {
    pub enabled: bool,
    pub note: u8,
    pub velocity: f32,
}

impl Step {
    const fn new(enabled: bool, note: u8, velocity: f32) -> Self {
        Self {
            enabled,
            note,
            velocity,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequencerPattern {
    pub bpm: u32,
    pub bars: usize,
    pub patch: LivePatchId,
    pub macro_target: MacroTarget,
    pub steps: Vec<Step>,
    pub automation: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scheduled<T> {
    pub frame_offset: usize,
    pub event: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoteEvent {
    NoteOn { note: u8, velocity: f32 },
    NoteOff { note: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControllerEvent {
    Macro { target: MacroTarget, value: f32 },
}

pub type ScheduledNoteEvent = Scheduled<NoteEvent>;
pub type ScheduledControllerEvent = Scheduled<ControllerEvent>;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderRequest {
    pub pattern: SequencerPattern,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderResult {
    pub sample_rate: usize,
    pub frame_count: usize,
    pub left: Vec<f32>,
    pub right: Vec<f32>,
    pub peak: f32,
    pub clipped: bool,
    pub note_event_count: usize,
    pub controller_event_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct RenderSummary {
    patch_name: &'static str,
    duration_seconds: f32,
    peak: f32,
    clipped: bool,
    note_event_count: usize,
    controller_event_count: usize,
}

#[derive(Clone, Copy)]
struct DemoPatchPreset {
    name: &'static str,
    code: &'static str,
    description: &'static str,
    osc1: WaveMix,
    osc2_mix: f32,
    osc2_interval: i8,
    detune_cents: f32,
    sub_level: f32,
    cutoff_hz: f32,
    resonance: f32,
    drive: f32,
    stereo_width: f32,
    attack_ms: f32,
    decay_ms: f32,
    sustain: f32,
    release_ms: f32,
    filter_env_depth: f32,
    body_mix: f32,
    base_macros: [f32; 5],
}

#[derive(Clone, Copy)]
struct WaveMix {
    saw: f32,
    pulse: f32,
    triangle: f32,
    noise: f32,
}

#[derive(Clone, Copy)]
struct VoiceParams {
    freq_hz: f32,
    detune_ratio: f32,
    osc2_ratio: f32,
    cutoff_hz: f32,
    resonance: f32,
    drive: f32,
    stereo_width: f32,
    pulse_width: f32,
    crossmix: f32,
}

struct ProcessBlock<'a> {
    frame_count: usize,
    note_events: &'a [ScheduledNoteEvent],
    controller_events: &'a [ScheduledControllerEvent],
    left: &'a mut [f32],
    right: &'a mut [f32],
}

struct DemoEngine {
    sample_rate: f32,
    patch: DemoPatchPreset,
    macros: [f32; 5],
    voices: Vec<Voice>,
    noise_state: u32,
    render_clock: usize,
}

struct Voice {
    note: u8,
    velocity: f32,
    phase_a: f32,
    phase_b: f32,
    phase_sub: f32,
    pan_seed: f32,
    filter_a: f32,
    filter_b: f32,
    envelope: Adsr,
}

struct Adsr {
    attack_samples: usize,
    decay_samples: usize,
    sustain_level: f32,
    release_samples: usize,
    stage: EnvelopeStage,
    level: f32,
    release_step: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished,
}

#[component]
pub fn PlayPage() -> Element {
    let mut selected_patch = use_signal(|| LivePatchId::MoltenHorizon);
    let mut bpm = use_signal(|| 112_u32);
    let mut bars = use_signal(|| 2_usize);
    let mut macro_target = use_signal(|| MacroTarget::Gravitacija);
    let mut steps = use_signal(default_steps);
    let mut automation = use_signal(default_automation);
    let mut status = use_signal(|| {
        "Choose a patch, shape the phrase, and hear the current EPM1 sound in the browser."
            .to_string()
    });
    let mut last_render = use_signal(|| Option::<RenderSummary>::None);

    let current_patch = *selected_patch.read();
    let current_bpm = *bpm.read();
    let current_bars = *bars.read();
    let current_macro_target = *macro_target.read();
    let step_values = steps.read().clone();
    let automation_values = automation.read().clone();
    let status_text = status.read().clone();
    let render_summary = last_render.read().clone();
    let active_preset = current_patch.preset();
    let render_meter_width = render_summary
        .as_ref()
        .map(|summary| (summary.peak * 100.0).clamp(6.0, 100.0))
        .unwrap_or(18.0);
    let render_meter_style = format!("width: {render_meter_width:.1}%;");
    let render_meter_class = if render_summary
        .as_ref()
        .is_some_and(|summary| summary.clipped)
    {
        "play-meter-fill is-hot"
    } else {
        "play-meter-fill"
    };

    rsx! {
        section { class: "play-shell",
            section { class: "play-hero",
                div { class: "play-copy",
                    span { class: "section-kicker", "Play / Browser demo" }
                    h1 { "Play the current software instrument." }
                    p {
                        "Choose a live-set patch, shape a 16-step phrase, drive one macro lane, and hear the result in the browser."
                    }
                    p {
                        "The patch names, macro targets, and MIDI-oriented controls follow the desktop runtime."
                    }
                    p { class: "play-hint", "{active_preset.code} / {current_macro_target.label()}: {current_macro_target.blurb()}" }
                    div { class: "hero-actions",
                        button {
                            class: "button button-primary",
                            r#type: "button",
                            onclick: move |_| {
                                let request = RenderRequest {
                                    pattern: SequencerPattern {
                                        bpm: *bpm.read(),
                                        bars: *bars.read(),
                                        patch: *selected_patch.read(),
                                        macro_target: *macro_target.read(),
                                        steps: steps.read().clone(),
                                        automation: automation.read().clone(),
                                    },
                                };
                                match render_pattern(&request) {
                                    Ok(rendered) => {
                                        let summary = RenderSummary::from_render(&request.pattern, &rendered);
                                        let message = match play_render(&rendered) {
                                            Ok(()) => format!(
                                                "Rendered {} at {} BPM and started browser playback.",
                                                request.pattern.patch.preset().name,
                                                request.pattern.bpm
                                            ),
                                            Err(error) => format!(
                                                "Rendered {} but browser playback could not start: {error}",
                                                request.pattern.patch.preset().name
                                            ),
                                        };
                                        last_render.set(Some(summary));
                                        status.set(message);
                                    }
                                    Err(error) => {
                                        status.set(format!("Render failed: {error}"));
                                    }
                                }
                            },
                            "Render + play"
                        }
                        button {
                            class: "button button-secondary",
                            r#type: "button",
                            onclick: move |_| {
                                stop_playback();
                                status.set("Stopped browser playback.".to_string());
                            },
                            "Stop"
                        }
                    }
                }
                div { class: "play-panel play-panel-secondary play-summary-panel",
                    div { class: "card-topline", "Current voice" }
                    h2 { "{active_preset.name}" }
                    p { "{active_preset.description}" }
                    div { class: "play-status-grid",
                        StatCard {
                            label: "Patch code".to_string(),
                            value: active_preset.code.to_string(),
                        }
                        StatCard {
                            label: "Output".to_string(),
                            value: "48 kHz stereo WAV".to_string(),
                        }
                        StatCard {
                            label: "Tempo".to_string(),
                            value: format!("{current_bpm} BPM"),
                        }
                        StatCard {
                            label: "Bars".to_string(),
                            value: format!("{current_bars}"),
                        }
                    }
                }
            }

            section { class: "play-grid",
                div { class: "play-main",
                    div { class: "play-panel play-panel-primary",
                        div { class: "control-stack",
                            div { class: "lane-header",
                                div {
                                    span { class: "section-kicker", "Patch bank" }
                                    h2 { "Eight live-set patches" }
                                }
                                p { "These names mirror the current EPM1 live set. The browser renderer keeps the patch and macro controls close to the instrument." }
                            }
                            div { class: "play-patch-grid",
                                for patch in LivePatchId::ALL {
                                    button {
                                        class: if patch == current_patch {
                                            "patch-card is-active"
                                        } else {
                                            "patch-card"
                                        },
                                        r#type: "button",
                                        onclick: move |_| selected_patch.set(patch),
                                        span { class: "card-topline", "{patch.preset().code}" }
                                        strong { "{patch.preset().name}" }
                                        p { "{patch.preset().description}" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "control-stack",
                        section { class: "play-panel play-panel-primary" ,
                            div { class: "lane-header",
                                div {
                                    span { class: "section-kicker", "Transport" }
                                    h2 { "Render the phrase and listen" }
                                }
                                p { "{status_text}" }
                            }

                            div { class: "play-toolbar",
                                div { class: "play-inline-group",
                                    span { class: "section-kicker", "Tempo" }
                                    div { class: "chip-row",
                                        for bpm_value in BPM_CHOICES {
                                            button {
                                                class: if bpm_value == current_bpm {
                                                    "chip-button is-active"
                                                } else {
                                                    "chip-button"
                                                },
                                                r#type: "button",
                                                onclick: move |_| bpm.set(bpm_value),
                                                "{bpm_value}"
                                            }
                                        }
                                    }
                                }

                                div { class: "play-inline-group",
                                    span { class: "section-kicker", "Bars" }
                                    div { class: "chip-row",
                                        for bar_count in BAR_CHOICES {
                                            button {
                                                class: if bar_count == current_bars {
                                                    "chip-button is-active"
                                                } else {
                                                    "chip-button"
                                                },
                                                r#type: "button",
                                                onclick: move |_| bars.set(bar_count),
                                                "{bar_count}"
                                            }
                                        }
                                    }
                                }

                                div { class: "play-inline-group",
                                    span { class: "section-kicker", "Macro lane" }
                                    div { class: "chip-row",
                                        for target in MacroTarget::ALL {
                                            button {
                                                class: if target == current_macro_target {
                                                    "chip-button is-active"
                                                } else {
                                                    "chip-button"
                                                },
                                                r#type: "button",
                                                onclick: move |_| macro_target.set(target),
                                                "{target.label()}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        section { class: "play-lanes",
                            div { class: "lane-panel",
                                div { class: "lane-header",
                                    div {
                                        span { class: "section-kicker", "Note lane" }
                                        h2 { "Toggle steps and cycle pitch" }
                                    }
                                    p { "Each active step plays a NoteOn and matching NoteOff. Click the pad to arm or mute it, then use the lower switch to move the pitch through the fixed stage set." }
                                }
                                LaneRuler {}
                                div { class: "step-grid",
                                    for (index, step) in step_values.iter().enumerate() {
                                        div { class: "step-stack",
                                            button {
                                                class: "{step_card_class(index, step.enabled)}",
                                                r#type: "button",
                                                onclick: move |_| {
                                                    let mut data = steps.write();
                                                    data[index].enabled = !data[index].enabled;
                                                },
                                                span { class: "step-index", {format!("Step {:02}", index + 1)} }
                                                strong { class: "step-note", {note_label(step.note)} }
                                                div { class: "step-meta",
                                                    span { class: "step-state",
                                                        if step.enabled {
                                                            "Armed"
                                                        } else {
                                                            "Muted"
                                                        }
                                                    }
                                                    span { class: "step-velocity", {velocity_label(step.velocity)} }
                                                }
                                            }
                                            button {
                                                class: "step-note-cycle",
                                                r#type: "button",
                                                onclick: move |_| {
                                                    let mut data = steps.write();
                                                    data[index].note = next_pitch(data[index].note);
                                                },
                                                "Cycle pitch"
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "lane-panel",
                                div { class: "lane-header",
                                    div {
                                        span { class: "section-kicker", "Automation lane" }
                                        h2 { "Drive one macro across the phrase" }
                                    }
                                    p { "The lane targets one macro at a time. Each click moves the step from base value to a higher setting, then wraps back to base." }
                                }
                                LaneRuler {}
                                div { class: "macro-grid",
                                    for (index, level_index) in automation_values.iter().enumerate() {
                                        button {
                                            class: "{automation_card_class(*level_index)}",
                                            r#type: "button",
                                            onclick: move |_| {
                                                let mut data = automation.write();
                                                data[index] = (data[index] + 1) % AUTOMATION_LEVELS.len();
                                            },
                                            span { class: "step-index", {format!("Step {:02}", index + 1)} }
                                            strong { {automation_stage_label(*level_index)} }
                                            div { class: "step-meta",
                                                span { class: "step-state", "{current_macro_target.label()}" }
                                                span { class: "step-velocity", {automation_label(*level_index)} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                aside { class: "play-inspector",
                    section { class: "play-panel play-panel-secondary" ,
                        div { class: "card-topline", "Render status" }
                        h2 { "Browser output" }
                        p { "Render a phrase, listen in the browser, then check the basic output numbers." }
                        div { class: "play-meter" ,
                            div { class: render_meter_class, style: render_meter_style }
                        }
                        div { class: "play-meter-copy",
                            span { class: "section-kicker", "Peak" }
                            strong {
                                if let Some(summary) = render_summary.as_ref() {
                                    {format!("{:.1} dBFS", peak_to_db(summary.peak))}
                                } else {
                                    "Ready"
                                }
                            }
                        }
                        div { class: "play-status-grid",
                            {
                                if let Some(summary) = render_summary.as_ref() {
                                    rsx! {
                                        StatCard {
                                            label: "Last patch".to_string(),
                                            value: summary.patch_name.to_string(),
                                        }
                                        StatCard {
                                            label: "Last render".to_string(),
                                            value: format!("{:.2}s", summary.duration_seconds),
                                        }
                                        StatCard {
                                            label: "Events".to_string(),
                                            value: format!(
                                                "{} note / {} macro",
                                                summary.note_event_count,
                                                summary.controller_event_count
                                            ),
                                        }
                                        StatCard {
                                            label: "Safety".to_string(),
                                            value: if summary.clipped {
                                                "Hot / clipped".to_string()
                                            } else {
                                                "Clean headroom".to_string()
                                            },
                                        }
                                    }
                                } else {
                                    rsx! {
                                        StatCard {
                                            label: "Ready".to_string(),
                                            value: "No render generated yet.".to_string(),
                                        }
                                        StatCard {
                                            label: "Pattern".to_string(),
                                            value: "16 steps / 1 note lane / 1 macro lane".to_string(),
                                        }
                                        StatCard {
                                            label: "Current".to_string(),
                                            value: format!("{} / {} BPM", active_preset.name, current_bpm),
                                        }
                                        StatCard {
                                            label: "Output".to_string(),
                                            value: "Stereo WAV via browser audio".to_string(),
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "play-panel play-panel-secondary" ,
                        div { class: "card-topline", "Runtime shape" }
                        h2 { "Constrained browser renderer" }
                        p { "One phrase, one macro lane, one browser render path. Enough to hear the current EPM1 runtime." }
                        ul { class: "play-detail-list",
                            li { "Offline block renderer with browser playback handoff." }
                            li { {format!("{}-frame processing blocks and {}-voice cap.", BLOCK_SIZE, MAX_VOICES)} }
                            li { "Macro lane sends controller events to the selected target." }
                            li { "Patch bank mirrors the current EPM1 live-set names." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String) -> Element {
    rsx! {
        article { class: "play-stat-card",
            span { class: "stat-label", "{label}" }
            strong { "{value}" }
        }
    }
}

#[component]
fn LaneRuler() -> Element {
    rsx! {
        div { class: "lane-ruler",
            for step in 0..STEPS_PER_BAR {
                span {
                    class: if step % 4 == 0 { "ruler-step is-anchor" } else { "ruler-step" },
                    {format!("{:02}", step + 1)}
                }
            }
        }
    }
}

impl RenderSummary {
    fn from_render(pattern: &SequencerPattern, render: &RenderResult) -> Self {
        Self {
            patch_name: pattern.patch.preset().name,
            duration_seconds: render.frame_count as f32 / render.sample_rate as f32,
            peak: render.peak,
            clipped: render.clipped,
            note_event_count: render.note_event_count,
            controller_event_count: render.controller_event_count,
        }
    }
}

impl DemoEngine {
    fn new(sample_rate: f32, patch: DemoPatchPreset) -> Self {
        Self {
            sample_rate,
            patch,
            macros: patch.base_macros,
            voices: Vec::with_capacity(MAX_VOICES),
            noise_state: 0x1357_2468,
            render_clock: 0,
        }
    }

    fn process_block(&mut self, block: &mut ProcessBlock<'_>) {
        block.left.fill(0.0);
        block.right.fill(0.0);

        let mut note_cursor = 0;
        let mut controller_cursor = 0;

        for frame in 0..block.frame_count {
            while controller_cursor < block.controller_events.len()
                && block.controller_events[controller_cursor].frame_offset == frame
            {
                self.handle_controller_event(block.controller_events[controller_cursor].event);
                controller_cursor += 1;
            }

            while note_cursor < block.note_events.len()
                && block.note_events[note_cursor].frame_offset == frame
            {
                self.handle_note_event(block.note_events[note_cursor].event);
                note_cursor += 1;
            }

            let time_seconds = (self.render_clock + frame) as f32 / self.sample_rate;
            let mut left_sample = 0.0_f32;
            let mut right_sample = 0.0_f32;
            for voice in &mut self.voices {
                let (left, right) = voice.render_sample(
                    &self.patch,
                    &self.macros,
                    self.sample_rate,
                    &mut self.noise_state,
                    time_seconds,
                );
                left_sample += left;
                right_sample += right;
            }

            block.left[frame] = soft_clip(left_sample * MASTER_GAIN);
            block.right[frame] = soft_clip(right_sample * MASTER_GAIN);
        }

        self.voices.retain(|voice| !voice.is_finished());
        self.render_clock += block.frame_count;
    }

    fn handle_note_event(&mut self, event: NoteEvent) {
        match event {
            NoteEvent::NoteOn { note, velocity } => {
                if self.voices.len() >= MAX_VOICES {
                    self.voices.remove(0);
                }
                let seed = ((self.render_clock as f32 * 0.0017) + note as f32 * 0.173).sin();
                self.voices
                    .push(Voice::new(note, velocity, seed, self.patch));
            }
            NoteEvent::NoteOff { note } => {
                if let Some(voice) = self
                    .voices
                    .iter_mut()
                    .rev()
                    .find(|voice| voice.note == note && !voice.envelope.is_releasing())
                {
                    voice.envelope.note_off();
                }
            }
        }
    }

    fn handle_controller_event(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::Macro { target, value } => {
                self.macros[target.as_index()] = value.clamp(0.0, 1.0);
            }
        }
    }
}

impl Voice {
    fn new(note: u8, velocity: f32, pan_seed: f32, patch: DemoPatchPreset) -> Self {
        Self {
            note,
            velocity,
            phase_a: 0.0,
            phase_b: 0.37,
            phase_sub: 0.73,
            pan_seed,
            filter_a: 0.0,
            filter_b: 0.0,
            envelope: Adsr::new(
                patch.attack_ms,
                patch.decay_ms,
                patch.sustain,
                patch.release_ms,
                SAMPLE_RATE as f32,
            ),
        }
    }

    fn render_sample(
        &mut self,
        patch: &DemoPatchPreset,
        macros: &[f32; 5],
        sample_rate: f32,
        noise_state: &mut u32,
        time_seconds: f32,
    ) -> (f32, f32) {
        let amplitude = self.envelope.next();
        if amplitude <= 0.0 {
            return (0.0, 0.0);
        }

        let params = self.voice_params(patch, macros);
        let osc1 = mixed_wave(self.phase_a, patch.osc1, params.pulse_width, noise_state);
        let osc2 = mixed_wave(
            self.phase_b,
            patch.osc1,
            (params.pulse_width - 0.08).clamp(0.2, 0.8),
            noise_state,
        );
        let sub = triangle_wave(self.phase_sub) * patch.sub_level;

        self.phase_a = wrap_phase(self.phase_a + params.freq_hz / sample_rate);
        self.phase_b = wrap_phase(
            self.phase_b + (params.freq_hz * params.osc2_ratio * params.detune_ratio) / sample_rate,
        );
        self.phase_sub = wrap_phase(self.phase_sub + (params.freq_hz * 0.5) / sample_rate);

        let noise = random_bipolar(noise_state) * patch.osc1.noise * (0.5 + macros[3] * 0.8);
        let body = (osc1 * 0.68)
            + (osc2 * patch.osc2_mix * (1.0 + macros[4] * 0.12))
            + sub * (0.7 + macros[2] * 0.4)
            + noise;
        let cross = osc1 * osc2 * params.crossmix;
        let envelope_push = 1.0 + (1.0 - amplitude) * patch.filter_env_depth * 0.7;
        let raw = (body + cross) * amplitude * (0.55 + self.velocity * 0.6) * envelope_push;
        let filtered = self.filtered_sample(raw, params.cutoff_hz, params.resonance);
        let driven = soft_clip(filtered * (1.0 + params.drive * 2.8));
        let sway = ((time_seconds * (0.16 + macros[4] * 0.45)) + self.pan_seed * PI).sin();
        let pan = (self.pan_seed * 0.65 + sway * params.stereo_width).clamp(-1.0, 1.0);
        let left = driven * (1.0 - pan) * 0.5;
        let right = driven * (1.0 + pan) * 0.5;
        (left, right)
    }

    fn filtered_sample(&mut self, input: f32, cutoff_hz: f32, resonance: f32) -> f32 {
        let g = (2.0 * PI * cutoff_hz / SAMPLE_RATE as f32).clamp(0.01, 0.92);
        let driven_input = input - self.filter_b * resonance * 0.32;
        self.filter_a += g * (driven_input - self.filter_a);
        self.filter_b += g * (self.filter_a - self.filter_b);
        self.filter_b
    }

    fn voice_params(&self, patch: &DemoPatchPreset, macros: &[f32; 5]) -> VoiceParams {
        let gravitacija = macros[MacroTarget::Gravitacija.as_index()];
        let bloom = macros[MacroTarget::Bloom.as_index()];
        let heat = macros[MacroTarget::Heat.as_index()];
        let ruin = macros[MacroTarget::Ruin.as_index()];
        let swarm = macros[MacroTarget::Swarm.as_index()];

        let base_freq = midi_note_to_hz(self.note);
        let detune_cents =
            patch.detune_cents + swarm * 18.0 + self.pan_seed * patch.stereo_width * 4.0;
        let detune_ratio = cents_to_ratio(detune_cents);
        let osc2_ratio = semitone_ratio(patch.osc2_interval as f32);
        let cutoff_hz = (patch.cutoff_hz
            * (0.82 + bloom * 0.75 + self.velocity * 0.18)
            * (1.0 - gravitacija * 0.24)
            * (1.0 + ruin * 0.08))
            .clamp(180.0, 10_800.0);
        let resonance = (patch.resonance + ruin * 0.12 + gravitacija * 0.08).clamp(0.08, 0.42);
        let drive = patch.drive + heat * 0.55 + ruin * 0.32 + gravitacija * 0.08;
        let stereo_width = (patch.stereo_width + bloom * 0.18 + swarm * 0.22 - gravitacija * 0.12)
            .clamp(0.05, 1.0);
        let pulse_width = (0.5 + bloom * 0.08 - ruin * 0.11).clamp(0.24, 0.76);
        let crossmix = patch.body_mix * 0.16 + ruin * 0.24 + heat * 0.12;

        VoiceParams {
            freq_hz: base_freq,
            detune_ratio,
            osc2_ratio,
            cutoff_hz,
            resonance,
            drive,
            stereo_width,
            pulse_width,
            crossmix,
        }
    }

    fn is_finished(&self) -> bool {
        self.envelope.is_finished()
    }
}

impl Adsr {
    fn new(
        attack_ms: f32,
        decay_ms: f32,
        sustain_level: f32,
        release_ms: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            attack_samples: ms_to_samples(attack_ms, sample_rate),
            decay_samples: ms_to_samples(decay_ms, sample_rate),
            sustain_level: sustain_level.clamp(0.0, 1.0),
            release_samples: ms_to_samples(release_ms, sample_rate),
            stage: EnvelopeStage::Attack,
            level: 0.0,
            release_step: 0.0,
        }
    }

    fn next(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Attack => {
                let step = 1.0 / self.attack_samples.max(1) as f32;
                self.level += step;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }
            }
            EnvelopeStage::Decay => {
                let step = (1.0 - self.sustain_level) / self.decay_samples.max(1) as f32;
                self.level -= step;
                if self.level <= self.sustain_level {
                    self.level = self.sustain_level;
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => {}
            EnvelopeStage::Release => {
                self.level -= self
                    .release_step
                    .max(1.0 / self.release_samples.max(1) as f32);
                if self.level <= 0.0 {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Finished;
                }
            }
            EnvelopeStage::Finished => {
                self.level = 0.0;
            }
        }
        self.level
    }

    fn note_off(&mut self) {
        if matches!(self.stage, EnvelopeStage::Release | EnvelopeStage::Finished) {
            return;
        }
        self.stage = EnvelopeStage::Release;
        self.release_step = self.level / self.release_samples.max(1) as f32;
    }

    fn is_releasing(&self) -> bool {
        matches!(self.stage, EnvelopeStage::Release | EnvelopeStage::Finished)
    }

    fn is_finished(&self) -> bool {
        self.stage == EnvelopeStage::Finished
    }
}

pub fn render_pattern(request: &RenderRequest) -> Result<RenderResult, String> {
    validate_pattern(&request.pattern)?;

    let (note_events, controller_events, total_frames) =
        pattern_to_events(&request.pattern, SAMPLE_RATE);
    let mut left = vec![0.0_f32; total_frames];
    let mut right = vec![0.0_f32; total_frames];
    let mut engine = DemoEngine::new(SAMPLE_RATE as f32, *request.pattern.patch.preset());
    let mut frame_cursor = 0;
    let mut note_cursor = 0;
    let mut controller_cursor = 0;

    while frame_cursor < total_frames {
        let frame_count = BLOCK_SIZE.min(total_frames - frame_cursor);
        let note_start = note_cursor;
        let controller_start = controller_cursor;

        while note_cursor < note_events.len()
            && note_events[note_cursor].frame_offset < frame_cursor + frame_count
        {
            note_cursor += 1;
        }
        while controller_cursor < controller_events.len()
            && controller_events[controller_cursor].frame_offset < frame_cursor + frame_count
        {
            controller_cursor += 1;
        }

        let block_note_events =
            remap_note_events(&note_events[note_start..note_cursor], frame_cursor);
        let block_controller_events = remap_controller_events(
            &controller_events[controller_start..controller_cursor],
            frame_cursor,
        );

        let mut block = ProcessBlock {
            frame_count,
            note_events: &block_note_events,
            controller_events: &block_controller_events,
            left: &mut left[frame_cursor..frame_cursor + frame_count],
            right: &mut right[frame_cursor..frame_cursor + frame_count],
        };
        engine.process_block(&mut block);
        frame_cursor += frame_count;
    }

    let peak = left
        .iter()
        .chain(right.iter())
        .fold(0.0_f32, |peak, sample| peak.max(sample.abs()));
    Ok(RenderResult {
        sample_rate: SAMPLE_RATE,
        frame_count: total_frames,
        left,
        right,
        peak,
        clipped: peak >= 0.995,
        note_event_count: note_events.len(),
        controller_event_count: controller_events.len(),
    })
}

fn validate_pattern(pattern: &SequencerPattern) -> Result<(), String> {
    if pattern.steps.len() != STEPS_PER_BAR {
        return Err(format!(
            "expected {STEPS_PER_BAR} note steps, got {}",
            pattern.steps.len()
        ));
    }
    if pattern.automation.len() != STEPS_PER_BAR {
        return Err(format!(
            "expected {STEPS_PER_BAR} automation steps, got {}",
            pattern.automation.len()
        ));
    }
    if pattern.bpm == 0 {
        return Err("tempo must be above zero".to_string());
    }
    if pattern.bars == 0 {
        return Err("bar count must be above zero".to_string());
    }
    Ok(())
}

fn pattern_to_events(
    pattern: &SequencerPattern,
    sample_rate: usize,
) -> (
    Vec<ScheduledNoteEvent>,
    Vec<ScheduledControllerEvent>,
    usize,
) {
    let bar_frames = frames_per_bar(pattern.bpm, sample_rate);
    let total_frames = bar_frames * pattern.bars;
    let step_frames = bar_frames as f32 / STEPS_PER_BAR as f32;
    let mut note_events = Vec::new();
    let mut controller_events = Vec::new();
    let base_value = pattern.patch.preset().base_macros[pattern.macro_target.as_index()];
    controller_events.push(Scheduled {
        frame_offset: 0,
        event: ControllerEvent::Macro {
            target: pattern.macro_target,
            value: base_value,
        },
    });

    for bar in 0..pattern.bars {
        let bar_offset = bar * bar_frames;
        for (index, step) in pattern.steps.iter().enumerate() {
            let frame_start = bar_offset + (index as f32 * step_frames).round() as usize;
            if step.enabled {
                let gate_frames = (step_frames * GATE_RATIO).round() as usize;
                let frame_end =
                    (frame_start + gate_frames.max(1)).min(total_frames.saturating_sub(1));
                note_events.push(Scheduled {
                    frame_offset: frame_start,
                    event: NoteEvent::NoteOn {
                        note: step.note,
                        velocity: step.velocity,
                    },
                });
                note_events.push(Scheduled {
                    frame_offset: frame_end,
                    event: NoteEvent::NoteOff { note: step.note },
                });
            }

            if let Some(value) = automation_value(pattern.automation[index]) {
                controller_events.push(Scheduled {
                    frame_offset: frame_start,
                    event: ControllerEvent::Macro {
                        target: pattern.macro_target,
                        value,
                    },
                });
            }
        }
    }

    note_events.sort_by_key(|event| event.frame_offset);
    controller_events.sort_by_key(|event| event.frame_offset);
    (note_events, controller_events, total_frames)
}

fn remap_note_events(events: &[ScheduledNoteEvent], base_frame: usize) -> Vec<ScheduledNoteEvent> {
    events
        .iter()
        .map(|event| Scheduled {
            frame_offset: event.frame_offset - base_frame,
            event: event.event,
        })
        .collect()
}

fn remap_controller_events(
    events: &[ScheduledControllerEvent],
    base_frame: usize,
) -> Vec<ScheduledControllerEvent> {
    events
        .iter()
        .map(|event| Scheduled {
            frame_offset: event.frame_offset - base_frame,
            event: event.event,
        })
        .collect()
}

fn default_steps() -> Vec<Step> {
    vec![
        Step::new(true, 36, 0.92),
        Step::new(false, 43, 0.80),
        Step::new(true, 48, 0.84),
        Step::new(false, 43, 0.78),
        Step::new(true, 55, 0.90),
        Step::new(true, 50, 0.82),
        Step::new(false, 48, 0.76),
        Step::new(true, 60, 0.88),
        Step::new(true, 48, 0.86),
        Step::new(false, 43, 0.76),
        Step::new(true, 55, 0.90),
        Step::new(false, 60, 0.74),
        Step::new(true, 62, 0.84),
        Step::new(false, 55, 0.76),
        Step::new(true, 67, 0.94),
        Step::new(true, 60, 0.88),
    ]
}

fn default_automation() -> Vec<usize> {
    vec![1, 0, 2, 0, 3, 1, 0, 4, 2, 0, 3, 0, 4, 1, 5, 2]
}

fn next_pitch(note: u8) -> u8 {
    let next_index = PITCH_CHOICES
        .iter()
        .position(|candidate| *candidate == note)
        .map(|index| (index + 1) % PITCH_CHOICES.len())
        .unwrap_or(0);
    PITCH_CHOICES[next_index]
}

fn note_label(note: u8) -> String {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let octave = (note / 12) as i32 - 1;
    let name = NAMES[(note % 12) as usize];
    format!("{name}{octave}")
}

fn velocity_label(velocity: f32) -> String {
    format!("Vel {:.0}%", velocity * 100.0)
}

fn automation_value(level_index: usize) -> Option<f32> {
    AUTOMATION_LEVELS.get(level_index).copied().flatten()
}

fn automation_label(level_index: usize) -> String {
    match automation_value(level_index) {
        Some(value) => format!("{:.0}%", value * 100.0),
        None => "Base".to_string(),
    }
}

fn automation_stage_label(level_index: usize) -> &'static str {
    match level_index {
        0 => "Base",
        1 => "Lift",
        2 => "Push",
        3 => "Heat",
        4 => "Drive",
        _ => "Max",
    }
}

fn step_card_class(index: usize, enabled: bool) -> &'static str {
    match (enabled, index % 4 == 0) {
        (true, true) => "seq-step is-active is-anchor",
        (true, false) => "seq-step is-active",
        (false, true) => "seq-step is-anchor",
        (false, false) => "seq-step",
    }
}

fn automation_card_class(level_index: usize) -> &'static str {
    if level_index == 0 {
        "macro-step"
    } else {
        "macro-step is-active"
    }
}

fn frames_per_bar(bpm: u32, sample_rate: usize) -> usize {
    ((sample_rate as f32) * 240.0 / bpm as f32).round() as usize
}

fn ms_to_samples(ms: f32, sample_rate: f32) -> usize {
    ((ms.max(0.0) / 1000.0) * sample_rate).round() as usize
}

fn midi_note_to_hz(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

fn cents_to_ratio(cents: f32) -> f32 {
    2.0_f32.powf(cents / 1200.0)
}

fn semitone_ratio(semitones: f32) -> f32 {
    2.0_f32.powf(semitones / 12.0)
}

fn wrap_phase(phase: f32) -> f32 {
    if phase >= 1.0 { phase - 1.0 } else { phase }
}

fn saw_wave(phase: f32) -> f32 {
    phase * 2.0 - 1.0
}

fn pulse_wave(phase: f32, width: f32) -> f32 {
    if phase < width { 1.0 } else { -1.0 }
}

fn triangle_wave(phase: f32) -> f32 {
    1.0 - 4.0 * (phase - 0.5).abs()
}

fn mixed_wave(phase: f32, mix: WaveMix, pulse_width: f32, noise_state: &mut u32) -> f32 {
    let signal = saw_wave(phase) * mix.saw
        + pulse_wave(phase, pulse_width) * mix.pulse
        + triangle_wave(phase) * mix.triangle
        + random_bipolar(noise_state) * mix.noise;
    let normalization = (mix.saw + mix.pulse + mix.triangle + mix.noise).max(0.8);
    signal / normalization
}

fn random_bipolar(state: &mut u32) -> f32 {
    *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    let value = ((*state >> 8) & 0x00ff_ffff) as f32 / 16_777_215.0;
    value * 2.0 - 1.0
}

fn soft_clip(sample: f32) -> f32 {
    sample.tanh()
}

fn peak_to_db(peak: f32) -> f32 {
    20.0 * peak.max(0.000_1).log10()
}

#[cfg(any(target_arch = "wasm32", test))]
fn wav_bytes(render: &RenderResult) -> Vec<u8> {
    let data_bytes = render.frame_count * 4;
    let riff_size = 36 + data_bytes as u32;
    let byte_rate = (render.sample_rate as u32) * 4;
    let mut bytes = Vec::with_capacity(44 + data_bytes);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&riff_size.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&(render.sample_rate as u32).to_le_bytes());
    bytes.extend_from_slice(&byte_rate.to_le_bytes());
    bytes.extend_from_slice(&4_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(data_bytes as u32).to_le_bytes());

    for frame in 0..render.frame_count {
        let left = (render.left[frame].clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        let right = (render.right[frame].clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        bytes.extend_from_slice(&left.to_le_bytes());
        bytes.extend_from_slice(&right.to_le_bytes());
    }

    bytes
}

#[cfg(target_arch = "wasm32")]
fn wav_data_url(render: &RenderResult) -> String {
    let encoded = base64::engine::general_purpose::STANDARD.encode(wav_bytes(render));
    format!("data:audio/wav;base64,{encoded}")
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ACTIVE_AUDIO: std::cell::RefCell<Option<web_sys::HtmlAudioElement>> =
        std::cell::RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
fn play_render(render: &RenderResult) -> Result<(), String> {
    let data_url = wav_data_url(render);
    ACTIVE_AUDIO.with(|audio_slot| {
        if let Some(existing) = audio_slot.borrow_mut().take() {
            existing.pause().map_err(js_error_to_string)?;
        }
        let audio =
            web_sys::HtmlAudioElement::new_with_src(&data_url).map_err(js_error_to_string)?;
        audio.set_preload("auto");
        let _ = audio.play().map_err(js_error_to_string)?;
        audio_slot.borrow_mut().replace(audio);
        Ok(())
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn play_render(_render: &RenderResult) -> Result<(), String> {
    Err("browser playback is only available in the wasm build".to_string())
}

#[cfg(target_arch = "wasm32")]
fn stop_playback() {
    ACTIVE_AUDIO.with(|audio_slot| {
        if let Some(existing) = audio_slot.borrow_mut().take() {
            let _ = existing.pause();
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn stop_playback() {}

#[cfg(target_arch = "wasm32")]
fn js_error_to_string(error: wasm_bindgen::JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "unknown browser audio error".to_string())
}

const MOLTEN_HORIZON: DemoPatchPreset = DemoPatchPreset {
    name: "Molten Horizon",
    code: "MOLT",
    description: "Open mass with late rupture. Wide, slow, and meant to bloom outward before it bites.",
    osc1: WaveMix {
        saw: 0.70,
        pulse: 0.25,
        triangle: 0.10,
        noise: 0.02,
    },
    osc2_mix: 0.45,
    osc2_interval: 0,
    detune_cents: 5.0,
    sub_level: 0.35,
    cutoff_hz: 4200.0,
    resonance: 0.22,
    drive: 0.18,
    stereo_width: 0.68,
    attack_ms: 25.0,
    decay_ms: 380.0,
    sustain: 0.82,
    release_ms: 760.0,
    filter_env_depth: 0.52,
    body_mix: 0.40,
    base_macros: [0.34, 0.62, 0.38, 0.18, 0.44],
};

const CATHEDRAL_BLOOM: DemoPatchPreset = DemoPatchPreset {
    name: "Cathedral Bloom",
    code: "CATH",
    description: "Stable Horizont pad with wide bloom and slow air. Best when the line needs space and slow movement.",
    osc1: WaveMix {
        saw: 0.56,
        pulse: 0.14,
        triangle: 0.26,
        noise: 0.01,
    },
    osc2_mix: 0.32,
    osc2_interval: 0,
    detune_cents: 4.0,
    sub_level: 0.24,
    cutoff_hz: 5200.0,
    resonance: 0.16,
    drive: 0.12,
    stereo_width: 0.82,
    attack_ms: 80.0,
    decay_ms: 420.0,
    sustain: 0.84,
    release_ms: 1120.0,
    filter_env_depth: 0.38,
    body_mix: 0.30,
    base_macros: [0.22, 0.84, 0.22, 0.08, 0.58],
};

const EMBER_VAULT: DemoPatchPreset = DemoPatchPreset {
    name: "Ember Vault",
    code: "EMBR",
    description: "Playable Pec bass with a dry furnace core. The body and pressure drive the patch.",
    osc1: WaveMix {
        saw: 0.48,
        pulse: 0.36,
        triangle: 0.10,
        noise: 0.00,
    },
    osc2_mix: 0.34,
    osc2_interval: -12,
    detune_cents: -3.0,
    sub_level: 0.62,
    cutoff_hz: 1800.0,
    resonance: 0.20,
    drive: 0.24,
    stereo_width: 0.26,
    attack_ms: 6.0,
    decay_ms: 180.0,
    sustain: 0.76,
    release_ms: 240.0,
    filter_env_depth: 0.44,
    body_mix: 0.74,
    base_macros: [0.42, 0.12, 0.78, 0.12, 0.10],
};

const RAZOR_THAW: DemoPatchPreset = DemoPatchPreset {
    name: "Razor Thaw",
    code: "RAZR",
    description: "Baklja-ready lead with high ruin pressure. This is the sharpest voice in the set.",
    osc1: WaveMix {
        saw: 0.58,
        pulse: 0.34,
        triangle: 0.00,
        noise: 0.03,
    },
    osc2_mix: 0.50,
    osc2_interval: 12,
    detune_cents: 7.0,
    sub_level: 0.22,
    cutoff_hz: 3100.0,
    resonance: 0.38,
    drive: 0.30,
    stereo_width: 0.40,
    attack_ms: 8.0,
    decay_ms: 160.0,
    sustain: 0.72,
    release_ms: 260.0,
    filter_env_depth: 0.64,
    body_mix: 0.34,
    base_macros: [0.68, 0.14, 0.46, 0.78, 0.18],
};

const GRAVITY_WAKE: DemoPatchPreset = DemoPatchPreset {
    name: "Gravity Wake",
    code: "GRAV",
    description: "Performance patch that folds inward as Gravitacija rises. Built for motion more than static polish.",
    osc1: WaveMix {
        saw: 0.64,
        pulse: 0.18,
        triangle: 0.12,
        noise: 0.02,
    },
    osc2_mix: 0.42,
    osc2_interval: 7,
    detune_cents: 6.0,
    sub_level: 0.38,
    cutoff_hz: 3600.0,
    resonance: 0.26,
    drive: 0.22,
    stereo_width: 0.58,
    attack_ms: 18.0,
    decay_ms: 280.0,
    sustain: 0.80,
    release_ms: 640.0,
    filter_env_depth: 0.56,
    body_mix: 0.46,
    base_macros: [0.56, 0.36, 0.44, 0.32, 0.34],
};

const FURNACE_CHOIR: DemoPatchPreset = DemoPatchPreset {
    name: "Furnace Choir",
    code: "FRNC",
    description: "Dense choir body with a hot Pec center. Meant to sound stacked even in this small demo engine.",
    osc1: WaveMix {
        saw: 0.62,
        pulse: 0.18,
        triangle: 0.08,
        noise: 0.01,
    },
    osc2_mix: 0.40,
    osc2_interval: 7,
    detune_cents: 3.0,
    sub_level: 0.48,
    cutoff_hz: 2400.0,
    resonance: 0.18,
    drive: 0.26,
    stereo_width: 0.52,
    attack_ms: 18.0,
    decay_ms: 220.0,
    sustain: 0.88,
    release_ms: 540.0,
    filter_env_depth: 0.46,
    body_mix: 0.62,
    base_macros: [0.48, 0.28, 0.72, 0.14, 0.30],
};

const GRANITE_PLAIN: DemoPatchPreset = DemoPatchPreset {
    name: "Granite Plain",
    code: "GRAN",
    description: "Dry poly anchor with restrained body and clean macro travel. Useful when the gesture should stay deliberate.",
    osc1: WaveMix {
        saw: 0.44,
        pulse: 0.18,
        triangle: 0.32,
        noise: 0.00,
    },
    osc2_mix: 0.24,
    osc2_interval: 0,
    detune_cents: 2.0,
    sub_level: 0.26,
    cutoff_hz: 2800.0,
    resonance: 0.14,
    drive: 0.10,
    stereo_width: 0.34,
    attack_ms: 14.0,
    decay_ms: 220.0,
    sustain: 0.86,
    release_ms: 340.0,
    filter_env_depth: 0.32,
    body_mix: 0.26,
    base_macros: [0.26, 0.28, 0.24, 0.08, 0.18],
};

const GLASS_TIDE: DemoPatchPreset = DemoPatchPreset {
    name: "Glass Tide",
    code: "GLSS",
    description: "Wide animated pad with motion in Bloom and Swarm. This one leans into width and shimmer.",
    osc1: WaveMix {
        saw: 0.50,
        pulse: 0.12,
        triangle: 0.22,
        noise: 0.01,
    },
    osc2_mix: 0.28,
    osc2_interval: 7,
    detune_cents: 5.0,
    sub_level: 0.20,
    cutoff_hz: 4800.0,
    resonance: 0.18,
    drive: 0.12,
    stereo_width: 0.88,
    attack_ms: 42.0,
    decay_ms: 360.0,
    sustain: 0.82,
    release_ms: 860.0,
    filter_env_depth: 0.44,
    body_mix: 0.28,
    base_macros: [0.24, 0.76, 0.26, 0.10, 0.70],
};

#[cfg(test)]
mod tests {
    use super::*;

    fn demo_pattern() -> SequencerPattern {
        SequencerPattern {
            bpm: 120,
            bars: 2,
            patch: LivePatchId::MoltenHorizon,
            macro_target: MacroTarget::Gravitacija,
            steps: default_steps(),
            automation: default_automation(),
        }
    }

    #[test]
    fn pattern_to_events_repeats_across_bars() {
        let pattern = demo_pattern();
        let (note_events, _, total_frames) = pattern_to_events(&pattern, SAMPLE_RATE);
        assert!(note_events.len() >= 16);
        assert!(
            note_events
                .iter()
                .any(|event| event.frame_offset > total_frames / 2)
        );
        assert!(matches!(
            note_events.first().map(|event| event.event),
            Some(NoteEvent::NoteOn { .. })
        ));
    }

    #[test]
    fn macro_lane_emits_controller_events() {
        let pattern = demo_pattern();
        let (_, controller_events, _) = pattern_to_events(&pattern, SAMPLE_RATE);
        assert!(controller_events.len() > 2);
        assert!(matches!(
            controller_events.first().map(|event| event.event),
            Some(ControllerEvent::Macro {
                target: MacroTarget::Gravitacija,
                ..
            })
        ));
    }

    #[test]
    fn render_pattern_returns_non_silent_audio() {
        let render = render_pattern(&RenderRequest {
            pattern: demo_pattern(),
        })
        .expect("demo pattern should render");

        assert_eq!(render.sample_rate, SAMPLE_RATE);
        assert!(render.frame_count > 0);
        assert!(render.peak > 0.001);
        assert!(render.left.iter().any(|sample| sample.abs() > 0.0001));
    }

    #[test]
    fn wav_bytes_start_with_riff() {
        let render = render_pattern(&RenderRequest {
            pattern: demo_pattern(),
        })
        .expect("demo pattern should render");
        let bytes = wav_bytes(&render);
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
    }
}
