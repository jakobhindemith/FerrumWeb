use eframe::egui;

pub fn window_view() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),..Default::default()
    };
    eframe::run_native(
        "Graph",options,Box::new(|cc|Ok(Box::<Graph>::default())));
    Ok(())
}

struct Graph {
    nodes: u32,
    eges: u32
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: 10,
            eges: 10,
        }
    }
}

impl eframe::App for Graph {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Graph View");
        });
    }
}