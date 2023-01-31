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

    fn tamano_fuente_adecuado(_cc: &eframe::CreationContext<'_>) -> f32 {
        // Obtención de un tamaño de letra que tiene en cuenta la resolución del monitor
        let puntos_fuente;
        
        #[cfg(target_arch = "wasm32")]
        {
        puntos_fuente = 20.0; // Tamaño de letra en versión web (provisional)
                            // Los puntos no parecen igual de grandes que en nativo
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
        let resolucion = _cc.integration_info.window_info.monitor_size.unwrap();
        //let resolucion = [1024.0, 768.0];
        let pixeles_por_punto = _cc.integration_info.native_pixels_per_point.unwrap();
        let pixeles_fuente_con_monitor_1080 = 20.0;
        let diagonal_monitor = ((resolucion[0] as f32).powi(2)  + (resolucion[1] as f32).powi(2)).sqrt();
        let diagonal_monitor_1080 = ((1920 as f32).powi(2)  + (1080 as f32).powi(2)).sqrt();
        let pixeles_fuente = (
            pixeles_fuente_con_monitor_1080 / diagonal_monitor_1080) * diagonal_monitor;
        puntos_fuente = pixeles_fuente / pixeles_por_punto;

        println!("Resolución de monitor: {}x{}", resolucion[0], resolucion[1]);
        println!("Píxeles por punto de forma nativa: {}", pixeles_por_punto);
        println!("Diagonal del monitor en píxeles: {}", diagonal_monitor);
        println!("Tamaño de fuente en píxeles: {}", pixeles_fuente);
        println!("Tamaño de fuente en puntos: {}", puntos_fuente);
        }

        puntos_fuente
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

        let mut style = (*cc.egui_ctx.style()).clone();
        let fuente = egui::FontFamily::Proportional;
        let puntos_fuente = Self::tamano_fuente_adecuado(cc);

        style.text_styles = [
            (egui::style::TextStyle::Heading, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Body, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Monospace, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Button, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Small, egui::FontId::new(puntos_fuente, fuente)),
        ].into();
        cc.egui_ctx.set_style(style);
    }

}