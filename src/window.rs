use eframe::{egui, CreationContext};
use petgraph::stable_graph::StableGraph;
use petgraph::Graph;

//create Window
pub fn window_view() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),..Default::default()
    };
    eframe::run_native(
        "Graph",options,Box::new(|_cc|Ok(Box::<GraphView>::default())))?;
    Ok(())
}

struct GraphView {
    nodes: u32,
    edges: u32
}

//petgraph -> StableGraph
impl GraphView {
    fn new(_: &CreationContext<'_>) -> Self {
        let g = generate_graph();
        Self {g: Graph::from(&g)}
    }
}

impl Default for GraphView {
    fn default() -> Self {
        Self {
            nodes: 0,
            edges: 0,
        }
    }
}

//setting up the ui
impl eframe::App for GraphView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            //ui elements
            ui.heading("Graph View");
            ui.add(egui::Slider::new(&mut self.nodes, 0..=120).text("nodes"));
            ui.add(egui::Slider::new(&mut self.edges, 0..=120).text("edges"));
            ui.label(format!("edges {}, nodes {}", self.edges, self.nodes));
        });
    }
}

//buid Test graph
fn generate_graph() -> StableGraph<(), ()> {
    let mut g = StableGraph::new();

       let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());

    g.add_edge(a, b, ());
    g.add_edge(b, c, ());
    g.add_edge(c, a, ());
    //return g
    g
}
