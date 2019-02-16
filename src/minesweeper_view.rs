//! Minesweeper view.

use graphics::{
        character::CharacterCache,
        {Context, Graphics},
        types::Color,
    };

use crate::minesweeper_controller::MinesweeperController;

/// Stores Minesweeper board view settings.
pub struct MinesweeperViewSettings {
    /// Position from top left corner.
    pub position: [f64; 2],
    /// Size of Minesweeper board along horizontal and vertical edge.
    pub size: f64,

    pub background_color: Color,

    pub cell_open_color: Color,

    pub cell_closed_color: Color,

    pub border_color: Color,

    pub outside_border_color: Color,

    pub hover_color_open: Color,

    pub hover_color_closed: Color,

    pub text_color: Color,

    pub outside_line_radius: f64,

    pub cell_line_radius: f64,
}


// TODO(Wesley): Figure the colors out.
impl MinesweeperViewSettings {
    /// Creates a new MinesweeperViewSettings. 
    pub fn new() -> MinesweeperViewSettings {
        MinesweeperViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [1.0; 4],
            cell_open_color: [0.66, 0.79, 0.64, 1.0],
            cell_closed_color: [0.4, 0.55, 0.55, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
            outside_border_color: [0.0, 0.0, 0.0, 1.0],
            hover_color_open: [0.65, 0.84, 0.52, 1.0],
            hover_color_closed: [0.65, 0.84, 0.52, 1.0],
            text_color: [1.0, 1.0, 1.0, 1.0],
            outside_line_radius: 3.0,
            cell_line_radius: 2.0,
        }
    }
}

/// Stores visual information about the Minesweeper board.
pub struct MinesweeperView {
    pub settings: MinesweeperViewSettings,
}

impl MinesweeperView {

    pub fn new(settings: MinesweeperViewSettings) -> MinesweeperView {
        MinesweeperView {
            settings: settings,
        }            
    }

    pub fn draw<G: Graphics, C>(&self, controller: &MinesweeperController, glyphs: &mut C, c: &Context, g: &mut G) 
        where C: CharacterCache<Texture = G::Texture>
    {
        use graphics::{Image, Line, Rectangle, Transformed};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size
        ];

        // Draw board background.
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // Draw selected cell background.
        if let Some(ind) = controller.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect = [
                settings.position[0] + pos[0], settings.position[1] + pos[1],
                cell_size, cell_size
            ];
            Rectangle::new(settings.hover_color_closed)
                .draw(cell_rect, &c.draw_state, c.transform, g);
        }

        // Draw characters.
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
                if let Some(ch) = controller.board.char([i, j]) {
                    let pos = [
                        settings.position[0] + i as f64 * cell_size + 15.0,
                        settings.position[1] + j as f64 * cell_size + 34.0,
                    ];
                    if let Ok(character) = glyphs.character(34, ch) {
                        let ch_x = pos[0] + character.left();
                        let ch_y = pos[1] - character.top();
                        text_image.draw(character.texture,
                                        &c.draw_state,
                                        c.transform.trans(ch_x, ch_y),
                                        g);
                    }
                }
            }
        }

        // Draw cell borders.
        let cell_edge = Line::new(settings.border_color, settings.cell_line_radius);
        for i in 0..9 {
            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_border(settings.border_color, settings.outside_line_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }

}
