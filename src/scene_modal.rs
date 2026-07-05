use crate::chart::ChartSize;
use crate::chart::{KnittingChart, StichCoordinate};
use egui::Sense;
use egui::{Align2, FontId};
use egui::{Color32, Frame, Pos2, Rect, Scene, Stroke, Vec2};
use math::round;
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChartScene {
    scene_rect: Rect,
    grid_lines: Vec<Vec<Pos2>>,
    grid_stroke_width: f32,
    grid_cell_heigth: f32,
    grid_cell_width: f32,
    lmargin: f32,
    tmargin: f32,
    grid_colour: Color32,
    color_a: Color32,
    color_b: Color32,
    dragging_over: Option<StichCoordinate>,
    error_circle_radius_due_to_slip_st: f32,
    error_circle_radius_due_to_float: f32,
    color_error_due_to_slip_st: Color32,
    color_error_due_to_float: Color32,
}

impl Default for ChartScene {
    fn default() -> Self {
        Self {
            scene_rect: Rect::ZERO,
            grid_lines: Default::default(),
            grid_stroke_width: 0.1,
            grid_cell_heigth: 10.0,
            grid_cell_width: 12.0,
            lmargin: 10.0,
            tmargin: 30.0,
            grid_colour: Color32::from_rgb(255, 233, 100),
            color_a: Color32::from_rgb(0, 233, 0),
            color_b: Color32::from_rgb(0, 0, 233),
            dragging_over: None,
            error_circle_radius_due_to_slip_st: 1.0,
            error_circle_radius_due_to_float: 3.0,
            color_error_due_to_slip_st: Color32::from_rgb(255, 233, 0),
            color_error_due_to_float: Color32::from_rgb(0, 233, 100),
        }
    }
}

impl ChartScene {
    pub fn show_options(&mut self, ui: &mut egui::Ui, chart: &mut KnittingChart, open: &mut bool){
        egui::Window::new("Scene")
            .default_width(300.0)
            .default_height(300.0)
            .scroll(false)
            .open(open)
            .constrain_to(ui.available_rect_before_wrap())
            .show(ui, |ui| self.options_ui(ui, chart));
    }
    pub fn show(&mut self, ui: &mut egui::Ui, chart: &mut KnittingChart, open: &mut bool) {
        self.ui(ui, chart);
    }

    fn construct_grid_lines(chart_size: ChartSize, grid_cell_width: f32, grid_cell_heigth: f32, lmargin: f32, tmargin: f32, ) -> Vec<Vec<Pos2>> {
        let mut  grid_lines: Vec<_> = vec![];        
        let overall_chart_width = chart_size.columns as f32 * grid_cell_width ;
        let overall_chart_heigth = chart_size.rows as f32 * grid_cell_heigth;    


        let first_vertical_line = vec![
            Pos2::new(lmargin, tmargin), 
            Pos2::new(lmargin, tmargin + overall_chart_heigth)];
        grid_lines.push(first_vertical_line);

        for column_id in 0..chart_size.columns {
            
            let column_end_line =  vec![
                Pos2::new(lmargin + (column_id as f32 + 1.0) * grid_cell_width , tmargin), 
                Pos2::new(lmargin + (column_id as f32 + 1.0) * grid_cell_width , tmargin + overall_chart_heigth)];
            grid_lines.push(column_end_line);

    
        }

        let first_horizontal_line = vec![
            Pos2::new(lmargin, tmargin), 
            Pos2::new( lmargin + overall_chart_width, tmargin)];
        grid_lines.push(first_horizontal_line);


        for row_id in 0..chart_size.rows {
            let row_end_line =  vec![
                Pos2::new(lmargin, tmargin + (row_id as f32 +1.0) * grid_cell_heigth), 
                Pos2::new(lmargin + overall_chart_width, tmargin + (row_id as f32 +1.0) * grid_cell_heigth )];
            grid_lines.push(row_end_line);  
        }
        return grid_lines;

    }

    fn get_cell_rect(coordinate: StichCoordinate, tmargin: f32, lmargin: f32, cell_width:f32, cell_heigth:f32, grid_size: ChartSize) -> Option<egui::Rect> {
        if coordinate.row > grid_size.rows || coordinate.column > grid_size.columns {
            return None;
        }
        
        let reverse_x_coordinate = grid_size.columns as f32 - coordinate.column as f32;
        let reverse_y_coordinate = grid_size.rows as f32- coordinate.row as f32;
        if (reverse_x_coordinate < -0.1 || reverse_y_coordinate < -0.1) {
            return None;
        }

        let rect_start_x_coord = lmargin + reverse_x_coordinate * cell_width;
        let rect_end_x_coord = lmargin + reverse_x_coordinate * cell_width + cell_width;
        let rect_start_y_coord = tmargin + reverse_y_coordinate * cell_heigth;
        let rect_end_y_coord = tmargin + reverse_y_coordinate * cell_heigth + cell_heigth;

        Some(egui::Rect::from_min_max(Pos2{x:rect_start_x_coord, y:rect_start_y_coord}, Pos2{x: rect_end_x_coord, y:rect_end_y_coord}))
    }

fn options_ui(&mut self, ui: &mut egui::Ui, chart: &mut KnittingChart){
            ui.label("This is a scene");
        ui.separator();

       
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.grid_stroke_width, 0.0..=10.0)
                        .text("grid stroke width"),
                );
                ui.add(egui::Slider::new(&mut self.lmargin, 0.0..=50.0).text("lmargin"));
                ui.add(egui::Slider::new(&mut self.tmargin, 0.0..=50.0).text("tmargin"));
                ui.add(egui::Slider::new(&mut self.grid_cell_width, 0.0..=50.0).text("cell width"));
                ui.add(
                    egui::Slider::new(&mut self.grid_cell_heigth, 0.0..=50.0).text("cell heigth"),
                );
            });
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.error_circle_radius_due_to_float, 0.0..=10.0)
                        .text("error circle size (Float)"),
                );
                ui.add(
                    egui::Slider::new(&mut self.error_circle_radius_due_to_slip_st, 0.0..=10.0)
                        .text("error circle size (Slip St)"),
                );
                
            });
            ui.vertical(|ui| {
                ui.label("Color A");
                ui.color_edit_button_srgba(&mut self.color_a);
                ui.label("Color B");
                ui.color_edit_button_srgba(&mut self.color_b);

                ui.label("Color Error (Float)");
                ui.color_edit_button_srgba(&mut self.color_error_due_to_float);

                ui.label("Color Error (Slip St)");
                ui.color_edit_button_srgba(&mut self.color_error_due_to_slip_st);

                if ui.button("Reset Chart").clicked() {
                    chart.reset_chart();
                }
                if ui.button("Undo").clicked() {
                    chart.undo();
                }
            });
        });

}

    fn ui(&mut self, ui: &mut egui::Ui, chart: &mut KnittingChart) {

         ui.label(format!("Scene rect: {:#?}", self.scene_rect));
        let reset_view = ui.button("Reset view").clicked();
        egui::Frame::group(ui.style())
            .inner_margin(0.0)
            .show(ui, |ui| {
                let scene = Scene::new()
                    .max_inner_size([1000.0, 1000.0])
                    
                    .zoom_range(0.01..=10.0)
                    ;

                
                let mut inner_rect = Rect::NAN;
            
                let scene_size =egui::Vec2::new(ui.available_width().clone(), ui.available_height().clone());

                let response = scene
                    .show(ui, &mut self.scene_rect, |ui| {
                        
                            //  Frame::canvas(ui.style()).show(ui, |ui| {
                                
                                     let (response, painter) =
                                                 ui.allocate_painter(scene_size, Sense::click_and_drag());


                                                                let overall_chart_width = chart.get_size().columns as f32 * self.grid_cell_width ;
                                                                let overall_chart_heigth = chart.get_size().rows as f32 * self.grid_cell_heigth;

                                                                // Grid
                                                                self.grid_lines = ChartScene::construct_grid_lines(
                                                                    chart.get_size().clone(),
                                                                    self.grid_cell_width.clone(),
                                                                    self.grid_cell_heigth.clone(),
                                                                    self.lmargin.clone(),
                                                                    self.tmargin.clone());

                                                                let shapes = self
                                                                .grid_lines
                                                                .iter()
                                                                .filter(|line| line.len() >= 2)
                                                                .map(|line| {
                                                                    let points: Vec<Pos2> = line.iter().map(|p| *p).collect();
                                                                    egui::Shape::line(points,  Stroke::new(self.grid_stroke_width, self.grid_colour))
                                                                });
                                                                painter.extend(shapes);


                                                                                // Column and row numbers
                                                                                for column_id in 0..chart.get_size().columns {
                                                                                    painter.text(Pos2{
                                                                                        x: self.lmargin + ((column_id as f32 + 1.0) * self.grid_cell_width ) - 0.5*self.grid_cell_width, 
                                                                                        y: overall_chart_heigth + self.tmargin + self.tmargin}, Align2::CENTER_CENTER, format!("{}", chart.get_size().columns - column_id), FontId::monospace(5.0), self.grid_colour);
                                                                                }

                                                                                for row_id in 0..chart.get_size().rows {
                                                                                    let label_color = match chart.get_row_leading_colour(chart.get_size().rows - row_id){
                                                                                        crate::chart::Stitch::ColorA => self.color_a,
                                                                                        crate::chart::Stitch::ColorB => self.color_b,
                                                                                    };
                                                                                    painter.text(Pos2{
                                                                                        x: self.lmargin + overall_chart_width + self.lmargin, 
                                                                                        y: self.tmargin +  ((row_id as f32 + 1.0) * self.grid_cell_heigth) - 0.5*self.grid_cell_heigth}, 
                                                                                        Align2::CENTER_CENTER, format!("{}", chart.get_size().rows - row_id), FontId::monospace(5.0), label_color);
                                                                                }

                                                                                for column_id in 1..(chart.get_size().columns +1 ){
                                                                                    for row_id in 1..(chart.get_size().rows +1){
                                                                                        let stitch_value = chart.get_one(&StichCoordinate { column: column_id, row: row_id });
                                                                                        match stitch_value{
                                                                                            Some(stitch_with_value) => {
                                                                                                let st_rect = ChartScene::get_cell_rect(StichCoordinate { column: column_id, row: row_id },self.tmargin,self.lmargin,self.grid_cell_width, self.grid_cell_heigth, chart.get_size());
                                                                                                match st_rect{
                                                                                                    Some(existing_rect) => {
                                                                                                        let colour_to_paint = match stitch_with_value{
                                                                                                            crate::chart::Stitch::ColorA => self.color_a,
                                                                                                            crate::chart::Stitch::ColorB => self.color_b,
                                                                                                        };
                                                                                                        let slightly_smaller_rect = existing_rect.expand(-2.0);
                                                                                                        painter.rect(slightly_smaller_rect, 2.0, colour_to_paint, Stroke::new(0.1, colour_to_paint), egui::StrokeKind::Inside);
                                                                                                    },
                                                                                                    None => {},
                                                                                                }
                                                                                            },
                                                                                            None => {},
                                                                                        }
                                                                                    }
                                                                                }


                                                                                                if let Some(pointer_pos) = response.interact_pointer_pos() {
                                                                                                    
                                                                                                    if response.drag_stopped(){
                                                                                                        self.dragging_over = None;
                                                                                                    }
                                                                                                    if response.clicked(){
                                                                                                        self.dragging_over = None;
                                                                                                    }

                                                                                                    
                                                                                                painter.circle(pointer_pos, 1.0, Color32::from_rgb(255, 255, 0),  Stroke::new(self.grid_stroke_width, self.grid_colour));
                                                                                                let canvas_pos = pointer_pos;
                                                                                                
                                                                                                let check_interaction = 
                                                                                                |canvas_pos: Pos2, tmargin: f32, lmargin: f32, cell_width:f32, cell_heigth:f32, grid_size: ChartSize| -> Option<StichCoordinate> { 
                                                                                                    if (canvas_pos.x < lmargin || canvas_pos.y < tmargin || 
                                                                                                        canvas_pos.x > lmargin + grid_size.columns as f32 *cell_width ||
                                                                                                        canvas_pos.y > tmargin + grid_size.rows as f32 *cell_heigth){
                                                                                                        return  None;
                                                                                                    }
                                                                                                    let xcoord = grid_size.columns -((canvas_pos.x - lmargin) / cell_width ) as usize;
                                                                                                    let ycoord = grid_size.rows - ((canvas_pos.y - tmargin) / cell_heigth ) as usize;
                                                                                                    Some(StichCoordinate { column: xcoord, row: ycoord })
                                                                                                };
                                                                                                
                                                                                            

                                                                                                match check_interaction(canvas_pos, self.tmargin, self.lmargin, self.grid_cell_width, self.grid_cell_heigth, chart.get_size()){
                                                                                                    Some(cell_id_interacted) => {
                                                                                                        match &mut self.dragging_over {
                                                                                                            Some(dragging_currently) => {
                                                                                                                if *dragging_currently != cell_id_interacted.clone() {
                                                                                                                    chart.interaction(cell_id_interacted.clone());
                                                                                                                    self.dragging_over = Some(cell_id_interacted.clone());
                                                                                                                }
                                                                                                            },
                                                                                                            None => {
                                                                                                                if response.clicked(){
                                                                                                                    chart.interaction(cell_id_interacted.clone());
                                                                                                                }
                                                                                                                
                                                                                                                if (response.drag_started() || response.dragged()){
                                                                                                                    chart.interaction(cell_id_interacted.clone());
                                                                                                                    self.dragging_over = Some(cell_id_interacted.clone());
                                                                                                                }
                                                                                                            },
                                                                                                        }

                                                                                                        
                                                                                                        painter.text(Pos2{x: 50.0, y: overall_chart_heigth + 100.0}, Align2::CENTER_CENTER, format!("{:?}", cell_id_interacted), FontId::monospace(6.0), Color32::from_rgb(255, 255, 0));
                                                                                                        let cell_rect = ChartScene::get_cell_rect(cell_id_interacted, self.tmargin,self.lmargin, self.grid_cell_width, self.grid_cell_heigth, chart.get_size());
                                                                                                        match cell_rect{
                                                                                                            Some(cell_rect) => {
                                                                                                                painter.rect(cell_rect, 1.0, Color32::from_white_alpha(0), Stroke::new(self.grid_stroke_width, self.grid_colour), egui::StrokeKind::Inside);
                                                                                                                
                                                                                                            },
                                                                                                            None => {},
                                                                                                        }
                                                                                                    },
                                                                                                    None => {},
                                                                                                };
                                                                                                
                                                                                            }
                        
                                                                                            let mosaic_errors = chart.validate_mosaic();

                                                                                                for item in mosaic_errors.1{
                                                                                                    let st_rect = ChartScene::get_cell_rect(item.0,self.tmargin,self.lmargin,self.grid_cell_width, self.grid_cell_heigth, chart.get_size());
                                                                                                    match st_rect{
                                                                                                        Some(existing_rect) => {
                                                                                                            let existing_rect_centre = existing_rect.center();
                                                                                                            //let error_rect = existing_rect.expand(-3.0);  

                                                                                                            
                                                                                                            let mut error_color = Color32::from_rgb(255, 0, 0);
                                                                                                            let mut radius = 1.0;
                                                                                                            match item.1{
                                                                                                                crate::chart::MosaicError::TooLargeFloat => {
                                                                                                                        error_color =  self.color_error_due_to_float;
                                                                                                                        radius = self.error_circle_radius_due_to_float;
                                                                                                                },
                                                                                                                crate::chart::MosaicError::SlipStitchWrongColour => {
                                                                                                                    error_color = self.color_error_due_to_slip_st;
                                                                                                                    radius = self.error_circle_radius_due_to_slip_st;
                                                                                                                },
                                                                                                            }
                                                                                                            painter.circle(existing_rect_centre, radius, error_color.clone(), Stroke::new(0.1, error_color.clone()));
                                                                                                        
                                                                                                        },
                                                                                                        None => {},
                                                                                                    }
                                                                                                    
                                                                                                }

                        //     response
                        
                        // });


                        inner_rect = ui.min_rect();
                    })
                    .response;

                if reset_view || response.double_clicked() {
                    self.scene_rect = inner_rect;
                }
            });
    }
}
