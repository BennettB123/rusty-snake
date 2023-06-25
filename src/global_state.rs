pub struct GlobalState<'a> {
    pub game_name: &'a str,
    pub screen_width: i32,
    pub screen_height: i32,
    pub num_columns: i32,
    pub num_rows: i32,
    pub draw_grid_padding: f32,
}
