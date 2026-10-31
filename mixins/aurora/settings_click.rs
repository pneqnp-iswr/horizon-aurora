fn aurora_settings_click(mod_id: &str, button: usize) {
    if mod_id != crate::game::aurora::MOD_ID {
        return;
    }
    crate::game::aurora::handle_settings_click(button);
}
