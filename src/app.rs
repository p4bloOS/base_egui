use crate::Util;

/// Derivamos Deserialize/Serialize para poder persistir el estado de la app al cerrarse
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // si agregamos nuevos campos, daremos valores predeterminados al deserializar el estado anterior
pub struct TemplateApp {
    // Atributos de ejemplo:
    texto_introducido: String,

    // Así es como se excluye de la serialización un miembro
    #[serde(skip)]
    replicar: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Atributos de ejemplo:
            texto_introducido: "Hola mundo!".to_owned(),
            replicar: false,
        }
    }
}

impl TemplateApp {
    /// Llamada una vez antes del primer fotograma
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Aquí se puede personalizar la apariencia de la IGU usando
        // `cc.egui_ctx.set_visuals` y `cc.egui_ctx.set_fonts`.

        // Personaliza los colores
        let visuales = Util::visuales();
        cc.egui_ctx.set_visuals(visuales);

        // Personaliza la fuente
        Util::cambiar_estilo_texto(cc);

        // Carga el estado previo de la aplicación (si lo hubiese)
        // Advierte que se debe habilitar la característica `persistence` para que funcione
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

/*Tenemos que implementar la característica App para escribir aplicaciones que puedan
compilarse tanto para web como para nativo usando eframe*/
impl eframe::App for TemplateApp {
    /// Llamada por el framework para guardar el estado después de cerrar.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Llamada cuando la IU necesita redibujarse, lo cual podría ocurrir múltiples veces por segundo.
    /// ctx es el contexto de la interfaz egui; nos permite manejarla.
    /// Pon tus widgets dentro de `SidePanel`, `TopPanel`, `CentralPanel`, `Window` o `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { texto_introducido, replicar } = self;

        // Ejemplos de cómo crear algunos paneles y widgets.
        // Consejo: una buena elección por defecto es simplemente dejar el `CentralPanel`.
        // Para más ejemplos e inspiración ir a https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Esta es una aplicación base muy sencilla usando Rust y la biblioteca egui con el framework eframe");

            ui.text_edit_multiline(texto_introducido);

            let boton = ui.button("Replicar");
            if boton.clicked(){
                *replicar = true;
            }
            if *replicar { ui.label(egui::RichText::new(texto_introducido.clone()).color(egui::Color32::WHITE));
            } else { ui.label(""); }

            // Este elemento no aparecerá en la versión web
            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Salir").clicked() {
                _frame.close();
            }
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
