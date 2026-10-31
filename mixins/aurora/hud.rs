fn aurora_hud(game: &mut crate::game::Game<'_>, batch: &mut HudBatch, _corners: &mut Corners) {
    if !game.native_on(crate::game::aurora::MOD_ID) || !game.native_playing() {
        return;
    }

    let snapshot = crate::game::aurora::hud_tick_snapshot(1.0 / 60.0);
    let line1 = match snapshot.locale {
        crate::game::aurora::Locale::En => snapshot.theme.name_en(),
        crate::game::aurora::Locale::Ru => snapshot.theme.name_ru(),
    };
    let line2 = format!("CPS {}  Combo {}", snapshot.cps, snapshot.combo);

    let flash = (snapshot.flash.clamp(0.0, 1.0) * 80.0) as u32;
    let bg_alpha = (96 + flash).min(220);
    let panel_argb = (bg_alpha << 24) | 0x101318;

    game.native_fill(batch, [6.0, 6.0, 154.0, 34.0], panel_argb);
    game.native_fill(
        batch,
        [6.0, 6.0, 154.0, 1.5],
        0xFF00_0000 | snapshot.theme.accent_rgb(),
    );
    game.native_text(batch, "Horizon Aurora", [11.0, 12.0], 0xF3F8FF, true);
    game.native_text(batch, line1, [11.0, 22.0], snapshot.theme.accent_rgb(), false);
    game.native_text(batch, &line2, [85.0, 22.0], 0xC8D0D8, false);
}
