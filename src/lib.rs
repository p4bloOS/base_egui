// Todo lo que escribamos aquí puede ser invocado en app.rs

#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

struct Util;

impl Util {

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
        fonts.font_data.insert("fuente_1".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/Cantarell-VF.otf")),);
        fonts.font_data.insert("fuente_2".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/DroidSansMono-enMp.ttf")),);
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

        // Obtención de un tamaño de letra que tiene en cuenta la resolución del monitor
        let pixeles_fuente;
        #[cfg(target_arch = "wasm32")]
        {
        pixeles_fuente = 14.0; // Tamaño de letra en versión web (provisional)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
        let resolucion = cc.integration_info.window_info.monitor_size.unwrap();
        println!("Resolución de monitor: {}x{}", resolucion[0], resolucion[1]);
        let pixeles_fuente_con_monitor_1080 = 12.0;
        let pixeles_monitor = resolucion[0]*resolucion[1];
        pixeles_fuente = (
            pixeles_fuente_con_monitor_1080/(1920.0*1080.0))*pixeles_monitor;
        }
    
        let mut style = (*cc.egui_ctx.style()).clone();
        let fuente = egui::FontFamily::Proportional;
        style.text_styles = [
            (egui::style::TextStyle::Heading, egui::FontId::new(pixeles_fuente, fuente.clone())),
            (egui::style::TextStyle::Body, egui::FontId::new(pixeles_fuente, fuente.clone())),
            (egui::style::TextStyle::Monospace, egui::FontId::new(pixeles_fuente, fuente.clone())),
            (egui::style::TextStyle::Button, egui::FontId::new(pixeles_fuente, fuente.clone())),
            (egui::style::TextStyle::Small, egui::FontId::new(pixeles_fuente, fuente)),
        ].into();
        cc.egui_ctx.set_style(style);
    }

}