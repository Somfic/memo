use deck::Deck;
use tauri_specta::collect_commands;

mod card;
mod deck;
mod prelude;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let spec_builder = tauri_specta::Builder::<tauri::Wry>::new().commands(collect_commands![
        deck::read_decks_from_file,
        deck::read_decks_from_anki_file
    ]);

    #[cfg(debug_assertions)]
    spec_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/gen/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(spec_builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            deck::read_decks_from_file,
            deck::read_decks_from_anki_file
        ])
        .setup(move |app| {
            spec_builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
