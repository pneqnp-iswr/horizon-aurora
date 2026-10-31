fn aurora_settings(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != crate::game::aurora::MOD_ID {
        return;
    }
    buttons.extend(crate::game::aurora::settings_buttons());
}
