use radiant_domain::{
    Channel, MixerInsert, Note, NoteId, Pattern, PatternLane, PlaylistClip, PlaylistClipContent,
    Rgba, Session, Step, Tick,
};
use radiant_session::ProjectStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/home/kenneth/EDM Test.radiant"));
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    let mut session = Session::new("Radiant EDM Test");
    session.tempo_map[0].bpm = 128.0;
    let master = session.master_insert().unwrap().id;
    let mut pattern = Pattern::new("Drop Groove", Tick::beats(16));
    let voices = [
        ("Kick", &[0, 4, 8, 12][..]),
        ("Clap", &[4, 12][..]),
        ("Closed Hat", &[2, 6, 10, 14][..]),
        ("Open Hat", &[7, 15][..]),
        ("Bass", &[0, 3, 8, 11][..]),
        ("Lead", &[0, 4, 8, 12][..]),
        ("Pluck", &[2, 6, 10, 14][..]),
        ("Pad", &[][..]),
        ("Percussion", &[1, 5, 9, 13][..]),
        ("Riser", &[][..]),
        ("Impact", &[0][..]),
    ];
    for (name, active) in voices {
        let insert = MixerInsert::insert(name, master);
        let id = insert.id;
        let mut channel = Channel::sampler(name, id);
        // Leave headroom for the summed EDM arrangement; individual channel
        // faders are intentionally conservative so the master never hard-clips.
        channel.volume = if name == "Kick" { 0.28 } else { 0.18 };
        let steps = active
            .iter()
            .map(|step| Step {
                at: Tick(i64::from(*step) * 240),
                velocity: 0.9,
                probability: 1.0,
                micro_shift: Tick::ZERO,
                ratchets: 1,
            })
            .collect();
        let notes = match name {
            "Bass" => (0..16)
                .step_by(4)
                .map(|bar| Note {
                    id: NoteId::new(),
                    start: Tick(i64::from(bar) * 240),
                    length: Tick::beats(2),
                    pitch: if bar % 8 == 0 { 36 } else { 43 },
                    velocity: 105,
                    channel: 0,
                })
                .collect(),
            "Lead" => [60, 64, 67, 71]
                .iter()
                .enumerate()
                .map(|(i, pitch)| Note {
                    id: NoteId::new(),
                    start: Tick::beats(i as i64 * 4),
                    length: Tick::beats(3),
                    pitch: *pitch,
                    velocity: 98,
                    channel: 0,
                })
                .collect(),
            "Pluck" => [72, 76, 79, 76]
                .iter()
                .enumerate()
                .map(|(i, pitch)| Note {
                    id: NoteId::new(),
                    start: Tick(i64::from(i as i32) * 960),
                    length: Tick::beats(1),
                    pitch: *pitch,
                    velocity: 92,
                    channel: 0,
                })
                .collect(),
            "Pad" => vec![Note {
                id: NoteId::new(),
                start: Tick::ZERO,
                length: Tick::beats(16),
                pitch: 48,
                velocity: 70,
                channel: 0,
            }],
            "Riser" => vec![Note {
                id: NoteId::new(),
                start: Tick::ZERO,
                length: Tick::beats(16),
                pitch: 60,
                velocity: 55,
                channel: 0,
            }],
            "Impact" => vec![Note {
                id: NoteId::new(),
                start: Tick::ZERO,
                length: Tick::beats(1),
                pitch: 36,
                velocity: 70,
                channel: 0,
            }],
            _ => Vec::new(),
        };
        pattern.lanes.push(PatternLane {
            channel_id: channel.id,
            steps,
            notes,
        });
        session.channels.push(channel);
        session.mixer_inserts.push(insert);
    }
    let drop_id = pattern.id;
    let fresh_pattern = |mut source: Pattern, name: &str| {
        source.id = radiant_domain::PatternId::new();
        source.name = name.into();
        for lane in &mut source.lanes {
            for note in &mut lane.notes {
                note.id = NoteId::new();
            }
        }
        source
    };
    let mut intro = fresh_pattern(pattern.clone(), "Intro - DJ Friendly");
    for (index, lane) in intro.lanes.iter_mut().enumerate() {
        if !matches!(index, 0 | 2 | 7) {
            lane.steps.clear();
            lane.notes.clear();
        }
    }
    let mut build = fresh_pattern(pattern.clone(), "Build - Snare Roll");
    for (index, lane) in build.lanes.iter_mut().enumerate() {
        if index == 1 {
            lane.steps = (0..16)
                .map(|step| Step {
                    at: Tick(i64::from(step) * 240),
                    velocity: 0.45 + step as f32 * 0.03,
                    probability: 1.0,
                    micro_shift: Tick::ZERO,
                    ratchets: 1,
                })
                .collect();
        }
        if matches!(index, 0 | 4 | 5 | 6 | 7 | 9 | 10 | 11) {
            lane.steps.clear();
        }
    }
    let mut breakdown = fresh_pattern(pattern.clone(), "Breakdown - Chords");
    for (index, lane) in breakdown.lanes.iter_mut().enumerate() {
        if !matches!(index, 5 | 6 | 7) {
            lane.steps.clear();
            lane.notes.clear();
        }
    }
    let outro = fresh_pattern(pattern.clone(), "Outro");
    let split_pattern = |source: &Pattern, name: &str, melodic: bool| {
        let mut split = fresh_pattern(source.clone(), name);
        split.lanes = split
            .lanes
            .into_iter()
            .enumerate()
            .filter(|(index, _)| {
                if melodic {
                    matches!(index, 4 | 5 | 6 | 7 | 9)
                } else {
                    matches!(index, 0 | 1 | 2 | 3 | 8 | 10)
                }
            })
            .map(|(_, lane)| lane)
            .collect();
        split
    };
    let intro_m = split_pattern(&intro, "Intro - Melody", true);
    let intro_d = split_pattern(&intro, "Intro - Drums", false);
    let build_m = split_pattern(&build, "Build - Melody", true);
    let build_d = split_pattern(&build, "Build - Drums", false);
    let drop_m = split_pattern(&pattern, "Drop - Melody", true);
    let drop_d = split_pattern(&pattern, "Drop - Drums", false);
    let breakdown_m = split_pattern(&breakdown, "Breakdown - Melody", true);
    let breakdown_d = split_pattern(&breakdown, "Breakdown - Drums", false);
    let outro_m = split_pattern(&outro, "Outro - Melody", true);
    let outro_d = split_pattern(&outro, "Outro - Drums", false);
    let intro_id = intro.id;
    let build_id = build.id;
    let breakdown_id = breakdown.id;
    let outro_id = outro.id;
    let melody_ids = [
        intro_m.id,
        build_m.id,
        drop_m.id,
        breakdown_m.id,
        build_m.id,
        drop_m.id,
        outro_m.id,
    ];
    let drum_ids = [
        intro_d.id,
        build_d.id,
        drop_d.id,
        breakdown_d.id,
        build_d.id,
        drop_d.id,
        outro_d.id,
    ];
    session.patterns.extend([
        pattern,
        intro,
        build,
        breakdown,
        outro,
        intro_m,
        intro_d,
        build_m,
        build_d,
        drop_m,
        drop_d,
        breakdown_m,
        breakdown_d,
        outro_m,
        outro_d,
    ]);
    session.playlist_lanes[0].name = "Melody Patterns".into();
    session
        .playlist_lanes
        .push(radiant_domain::PlaylistLane::new("Drum Patterns"));
    // Compact demonstration arrangement: each section is short enough for
    // Radiant's current synchronous preview renderer while retaining the
    // standard intro/build/drop/breakdown/second-drop/outro energy arc.
    let sections = [
        ("Intro", 0, 1, intro_id),
        ("Build 1", 1, 1, build_id),
        ("Drop 1", 2, 2, drop_id),
        ("Breakdown", 4, 1, breakdown_id),
        ("Build 2", 5, 1, build_id),
        ("Drop 2", 6, 1, drop_id),
        ("Outro", 7, 1, outro_id),
    ];
    for (index, (name, start, length, _)) in sections.iter().enumerate() {
        session.playlist_lanes[0].clips.push(PlaylistClip {
            id: radiant_domain::ClipId::new(),
            name: format!("{name} Melody"),
            start: Tick::beats(*start),
            length: Tick::beats(*length),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: melody_ids[index],
            },
        });
        session.playlist_lanes[1].clips.push(PlaylistClip {
            id: radiant_domain::ClipId::new(),
            name: format!("{name} Drums"),
            start: Tick::beats(*start),
            length: Tick::beats(*length),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: drum_ids[index],
            },
        });
    }
    let _ = ProjectStore::create(&root, &session)?;
    println!("{}", root.display());
    Ok(())
}
