

use std::collections::VecDeque;

use ndarray::Array2;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum MosaicError {
    TooLargeFloat,
    SlipStitchWrongColour,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Stitch {
    ColorA,
    ColorB,
}
#[derive(Clone, Debug, Default)]
pub struct ChartSize {
    pub columns: usize,
    pub rows: usize,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StichCoordinate {
    pub column: usize,
    pub row: usize,
}

impl StichCoordinate {
    pub fn one_below(self) -> Option<StichCoordinate> {
        if (self.row > 0) {
            return Some(StichCoordinate {
                row: self.row - 1,
                column: self.column,
            });
        } else {
            return None;
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct KnittingChart {
    rows: usize,
    columns: usize,
    contents: Array2<Stitch>,
    acceptable_float: usize,
    previous_states: VecDeque<Array2<Stitch>>,
    max_undo: usize
}

impl KnittingChart {

    fn save_to_previous_states(&mut self){
        self.previous_states.insert(0, self.contents.clone());
        if self.previous_states.len() > self.max_undo {
            self.previous_states.truncate(self.max_undo);
        }
    }

    pub fn undo(&mut self){
        let _curr_st = self.previous_states.pop_front();
        let prev_st = self.previous_states.front();
        match (prev_st){
            Some(val) => {
                self.contents = val.clone();
                self.rows = self.contents.dim().0;
                self.columns = self.contents.dim().1;
            },
            None => {},
        }
    }


    pub fn get_one(&self, coordinate: &StichCoordinate) -> Option<Stitch> {
        if (coordinate.row > 0
            && coordinate.row < self.get_size().rows + 1
            && coordinate.column > 0
            && coordinate.column < self.get_size().columns + 1)
        {
            return match self
                .contents
                .get((coordinate.row - 1, coordinate.column - 1))
            {
                Some(val) => Some(val.clone()),
                None => None,
            };
        } else {
            return None;
        }
    }

    pub fn add_column_right(&self) -> (){}
    pub fn delete_column(&self) -> (){}
    pub fn add_column_left(&self) -> (){}
    pub fn add_row_above(&self) -> (){}
    pub fn add_row_below(&self) -> (){}
    pub fn delete_row(&self) -> (){}

    pub fn get_row_leading_colour(&self, row_id: usize) -> Stitch {
        if row_id % 2 == 0 {
            return Stitch::ColorA;
        } else {
            return Stitch::ColorB;
        }
    }

    fn get_one_mut(&mut self, coordinate: StichCoordinate) -> Option<&mut Stitch> {
        match self
            .contents
            .get_mut((coordinate.row - 1, coordinate.column - 1))
        {
            Some(mut_val) => {
                return Some(mut_val);
            }
            None => None,
        }
    }

    pub fn get_all(&self) -> &Array2<Stitch> {
        &self.contents
    }

    pub fn reset_chart(&mut self) -> () {
        self.contents =
            Array2::from_shape_fn((self.rows, self.columns), |(r, _c)| 
            {
                let col = self.get_row_leading_colour(r+1);
                col
            }
        );
        self.save_to_previous_states();
    }

    pub fn get_size(&self) -> ChartSize {
        return ChartSize {
            columns: self.columns,
            rows: self.rows,
        };
    }

    pub fn interaction(&mut self, coordinate: StichCoordinate) {
        match self.get_one_mut(coordinate) {
            Some(existing_stitch) => {
            match existing_stitch {
                Stitch::ColorA => *existing_stitch = Stitch::ColorB,
                Stitch::ColorB => *existing_stitch = Stitch::ColorA,
            }
            self.save_to_previous_states();
        },
            None => {}
        }
    }

    fn validate_colour(
        &self,
        leading: &Stitch,
        current_color: Stitch,
        one_below_colour: Stitch,
    ) -> Option<MosaicError> {
        if (*leading != current_color && one_below_colour != current_color) {
            match one_below_colour {
                Stitch::ColorA => Some(MosaicError::SlipStitchWrongColour),
                Stitch::ColorB => Some(MosaicError::SlipStitchWrongColour),
            }
        } else {
            return None;
        }
    }

    pub fn validate_mosaic(&self) -> (bool, Vec<(StichCoordinate, MosaicError)>) {
        let mut errors: Vec<(StichCoordinate, MosaicError)> = vec![];
        for row_id in 1..self.get_size().rows + 1 {
            let leading = self.get_row_leading_colour(row_id);
            for column_id in 1..self.get_size().columns + 1 {
                let current_coordinate = StichCoordinate {
                    row: row_id,
                    column: column_id,
                };
                let current_value = self.get_one(&current_coordinate);
                match current_value {
                    Some(stitch_value) => {
                        if (stitch_value == Stitch::ColorA || stitch_value == Stitch::ColorB) {
                            let one_below_coordinate = current_coordinate.clone().one_below();
                            match one_below_coordinate {
                                Some(existing_one_below) => {
                                    let one_below_val = self.get_one(&existing_one_below);
                                    match one_below_val {
                                        Some(one_below_color) => {
                                            let result: Option<MosaicError> = self.validate_colour(
                                                &leading,
                                                stitch_value.clone(),
                                                one_below_color,
                                            );
                                            match result {
                                                Some(some_error) => {
                                                    errors.push((
                                                        current_coordinate.clone(),
                                                        some_error,
                                                    ));
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {}
                            }
                            if stitch_value != leading {
                                //Check for floats
                                let mut first_group_stitch: Option<StichCoordinate> = None;
                                let mut currently_checking: StichCoordinate =
                                    current_coordinate.clone();
                                while first_group_stitch == None {
                                    let left_neighbour = StichCoordinate {
                                        row: currently_checking.row.clone(),
                                        column: currently_checking.column.clone() - 1,
                                    };
                                    match self.get_one(&left_neighbour) {
                                        Some(existing_neighbour) => {
                                            if (existing_neighbour == stitch_value) {
                                                currently_checking = left_neighbour.clone();
                                            } else {
                                                first_group_stitch =
                                                    Some(currently_checking.clone());
                                            }
                                        }
                                        None => {
                                            first_group_stitch = Some(currently_checking.clone());
                                        }
                                    }
                                }

                                let mut last_group_stitch: Option<StichCoordinate> = None;
                                currently_checking = current_coordinate.clone();
                                while last_group_stitch == None {
                                    let right_neighbour = StichCoordinate {
                                        row: currently_checking.row.clone(),
                                        column: currently_checking.column.clone() + 1,
                                    };
                                    match self.get_one(&right_neighbour) {
                                        Some(existing_neighbour) => {
                                            if (existing_neighbour == stitch_value) {
                                                currently_checking = right_neighbour.clone();
                                            } else {
                                                last_group_stitch =
                                                    Some(currently_checking.clone());
                                            }
                                        }
                                        None => {
                                            last_group_stitch = Some(currently_checking.clone());
                                        }
                                    }
                                }

                                let group_len = last_group_stitch.unwrap().column
                                    - first_group_stitch.unwrap().column
                                    + 1;
                                if group_len > self.acceptable_float {
                                    errors.push((
                                        current_coordinate.clone(),
                                        MosaicError::TooLargeFloat,
                                    ));
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        return (false, errors);
    }
}

impl Default for KnittingChart {
    fn default() -> Self {
        let mut obj = Self {
            rows: 12,
            columns: 12,
            contents: Array2::from_shape_fn((12, 12), |(r, _c)| if (r+1) % 2 == 0 {Stitch::ColorA} else {Stitch::ColorB}),
            acceptable_float: 2,
            previous_states: vec![].into(),
            max_undo: 25
        };
        obj.previous_states.insert(0,obj.contents.clone());
        obj
    }
}
