// Todo lo que escribamos aquí puede ser invocado en app.rs

#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

struct Util;

impl Util {

    fn funcion_ejemplo() {
        println!("Esta es la función de ejemplo situada en el fichero lib.rs. Esto es lo único que hace");
    }

    fn visuales() -> egui::Visuals {
        let mut visual = egui::Visuals::dark();
        visual.panel_fill = egui::Color32::from_rgb(32, 33, 36);
        visual.override_text_color = Some(egui::Color32::from_rgb(5,213,255));
        //visual.override_text_color = Some(egui::Color32::WHITE);
        visual
    }
    
    fn cambiar_estilo_texto(cc: &eframe::CreationContext<'_>) {

        let mut fonts = egui::FontDefinitions::default();
        // Instalamos nuestra propia fuente (.ttf and .otf files supported)
        fonts.font_data.insert(
            "fuente_1".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../assets/Cantarell-VF.otf"
        )),);
        fonts.font_data.insert(
            "fuente_2".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../assets/DroidSansMono-enMp.ttf" //
        )),);
        // Damos la máxima prioridad a nuestra fuente_1 para el texto "Proportional":
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "fuente_1".to_owned());
        // Damos la máxima prioridad a nuestra fuente_2 para el texto "Monospace":
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "fuente_2".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        // println!("Píxeles por punto: {}", cc.egui_ctx.pixels_per_point());
        let mut style = (*cc.egui_ctx.style()).clone();
        let fuente = egui::FontFamily::Proportional;
        style.text_styles = [
            (egui::style::TextStyle::Heading, egui::FontId::new(16.0, fuente.clone())),
            (egui::style::TextStyle::Body, egui::FontId::new(16.0, fuente.clone())),
            (egui::style::TextStyle::Monospace, egui::FontId::new(16.0, fuente.clone())),
            (egui::style::TextStyle::Button, egui::FontId::new(16.0, fuente.clone())),
            (egui::style::TextStyle::Small, egui::FontId::new(16.0, fuente)),
        ].into();
        cc.egui_ctx.set_style(style);
    }

}




