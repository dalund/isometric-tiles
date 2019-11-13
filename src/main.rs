use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{event, graphics, input, Context, GameResult};
use std::path;

const GRID_SIZE: (i32, i32) = (14, 10);
const ORIGIN: (i32, i32) = (5, 1);
// Now we define the pixel size of each tile, which we make 32x32 pixels.
const GRID_CELL_SIZE: (i32, i32) = (40, 20);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

struct World {
    spritebatch: graphics::spritebatch::SpriteBatch,
    tiles: [u8; (GRID_SIZE.0 * GRID_SIZE.1) as usize],
}

impl World {
    fn new(ctx: &mut Context) -> GameResult<World> {
        let image = graphics::Image::new(ctx, "/isometric_demo.png")?;
        let spritebatch = graphics::spritebatch::SpriteBatch::new(image);
        let w = World {
            spritebatch,
            tiles: [0; (GRID_SIZE.0 * GRID_SIZE.1) as usize],
        };
        Ok(w)
    }
}

fn to_screen(x: i32, y: i32) -> Vector2<i32> {
    Vector2::new(
        (ORIGIN.0 * GRID_CELL_SIZE.0) + (x - y) * (GRID_CELL_SIZE.0 / 2),
        (ORIGIN.1 * GRID_CELL_SIZE.1) + (x + y) * (GRID_CELL_SIZE.1 / 2),
    )
}

impl EventHandler for World {
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        let mouse = ggez::input::mouse::position(ctx);
        let cell = Point2::new(
            mouse.x as i32 / GRID_CELL_SIZE.0,
            mouse.y as i32 / GRID_CELL_SIZE.1,
        );
        let offset = Point2::new(
            mouse.x as i32 % GRID_CELL_SIZE.0,
            mouse.y as i32 % GRID_CELL_SIZE.1,
        );

        let img = graphics::Image::new(ctx, "/isometric_demo.png").unwrap();
        let img_to_rgba = img.to_rgba8(ctx).unwrap();

        let offset_y = 4 * offset.y * img.width() as i32;
        let red = img_to_rgba[offset_y as usize + 0 + ((3 * 40 * 4) + (4 * offset.x)) as usize];
        let green = img_to_rgba[offset_y as usize + 1 + ((3 * 40 * 4) + (4 * offset.x)) as usize];
        let blue = img_to_rgba[offset_y as usize + 2 + ((3 * 40 * 4) + (4 * offset.x)) as usize];
        let color = graphics::Color::from_rgb(red, green, blue);

        let RED: Color = Color::from_rgb(255, 0, 0);
        let GREEN: Color = Color::from_rgb(0, 255, 0);
        let BLUE: Color = Color::from_rgb(0, 0, 255);
        let YELLOW: Color = Color::from_rgb(255, 255, 0);

        let mut selected = Vector2::new(
            (cell.y - ORIGIN.1) + (cell.x - ORIGIN.0),
            (cell.y - ORIGIN.1) - (cell.x - ORIGIN.0),
        );

        if color == RED {
            selected += Vector2::new(-1, 0);
        }
        if color == GREEN {
            selected += Vector2::new(0, 1);
        }
        if color == BLUE {
            selected += Vector2::new(0, -1);
        }
        if color == YELLOW {
            selected += Vector2::new(1, 0);
        }

        self.tiles[(selected.y * GRID_SIZE.0 + selected.x) as usize] += 1;
        self.tiles[(selected.y * GRID_SIZE.0 + selected.x) as usize] %= 6;
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let mouse = ggez::input::mouse::position(ctx);
        let cell = Point2::new(
            mouse.x as i32 / GRID_CELL_SIZE.0,
            mouse.y as i32 / GRID_CELL_SIZE.1,
        );
        let offset = Point2::new(
            mouse.x as i32 % GRID_CELL_SIZE.0,
            mouse.y as i32 % GRID_CELL_SIZE.1,
        );

        let img = graphics::Image::new(ctx, "/isometric_demo.png")?;
        let img_to_rgba = img.to_rgba8(ctx)?;

        let offset_y = 4 * offset.y * img.width() as i32;
        let red = img_to_rgba[offset_y as usize + 0 + ((3 * 40 * 4) + (4 * offset.x)) as usize];
        let green = img_to_rgba[offset_y as usize + 1 + ((3 * 40 * 4) + (4 * offset.x)) as usize];
        let blue = img_to_rgba[offset_y as usize + 2 + ((3 * 40 * 4) + (4 * offset.x)) as usize];
        let color = graphics::Color::from_rgb(red, green, blue);

        let RED: Color = Color::from_rgb(255, 0, 0);
        let GREEN: Color = Color::from_rgb(0, 255, 0);
        let BLUE: Color = Color::from_rgb(0, 0, 255);
        let YELLOW: Color = Color::from_rgb(255, 255, 0);

        let mut selected = Vector2::new(
            (cell.y - ORIGIN.1) + (cell.x - ORIGIN.0),
            (cell.y - ORIGIN.1) - (cell.x - ORIGIN.0),
        );

        if color == RED {
            selected += Vector2::new(-1, 0);
        }
        if color == GREEN {
            selected += Vector2::new(0, 1);
        }
        if color == BLUE {
            selected += Vector2::new(0, -1);
        }
        if color == YELLOW {
            selected += Vector2::new(1, 0);
        }

        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                let vector_world = to_screen(x, y);
                match self.tiles[(y * GRID_SIZE.0 + x) as usize] {
                    0 => {
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(0.25, 0.0, 0.25, 0.35))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                    1 => {
                        // visible tile
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(0.5, 0.0, 0.25, 1.0 / 3.0))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                    2 => {
                        // tree
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(0.0, 1.0 / 3.0, 0.25, 2.0 / 3.0))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y - GRID_CELL_SIZE.1) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                    3 => {
                        // spooky tree
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(0.25, 1.0 / 3.0, 0.25, 2.0 / 3.0))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y - GRID_CELL_SIZE.1) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                    4 => {
                        // beach
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(0.5, 2.0 / 3.0, 0.25, 1.0 / 3.0))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                    5 => {
                        // water
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(3.0 / 4.0, 2.0 / 3.0, 0.25, 1.0 / 3.0))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                    _ => {
                        // invisible tile
                        let p = graphics::DrawParam::new()
                            .src(graphics::Rect::new(0.25, 0.0, 0.25, 0.35))
                            .dest(Point2::new(
                                (vector_world.x) as f32,
                                (vector_world.y) as f32,
                            ));
                        // .scale(Vector2::new(2.0, 2.0));

                        self.spritebatch.add(p);
                    }
                }
            }
        }

        let selected_world = to_screen(selected.x, selected.y);
        let p = graphics::DrawParam::new()
            .src(graphics::Rect::new(0.0, 0.0, 0.25, 0.35))
            .dest(Point2::new(
                (selected_world.x) as f32,
                (selected_world.y) as f32,
            ));

        self.spritebatch.add(p);

        graphics::draw(ctx, &self.spritebatch, graphics::DrawParam::new())?;
        self.spritebatch.clear();

        // let rectangle = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::stroke(2.0),
        //     graphics::Rect::new(
        //         (cell.x * GRID_CELL_SIZE.0) as f32,
        //         (cell.y * GRID_CELL_SIZE.1) as f32,
        //         (GRID_CELL_SIZE.0) as f32,
        //         (GRID_CELL_SIZE.1) as f32,
        //     ),
        //     [0.0, 0.3, 0.3, 1.0].into(),
        // )?;
        // graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        let text = graphics::Text::new(
            graphics::TextFragment::new(format!("Mouse: {},{}", mouse.x as i32, mouse.y as i32))
                .color(graphics::BLACK),
        );
        let cell_text = graphics::Text::new(
            graphics::TextFragment::new(format!("Cell: {},{}", cell.x, cell.y))
                .color(graphics::BLACK),
        );
        let selected_text = graphics::Text::new(
            graphics::TextFragment::new(format!("selected: {},{}", selected.x, selected.y))
                .color(graphics::BLACK),
        );

        graphics::draw(ctx, &text, (ggez::mint::Point2 { x: 40.0, y: 20.0 },))?;
        graphics::draw(ctx, &cell_text, (ggez::mint::Point2 { x: 40.0, y: 40.0 },))?;
        graphics::draw(
            ctx,
            &selected_text,
            (ggez::mint::Point2 { x: 40.0, y: 60.0 },),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    println!("screen size : {},{}", SCREEN_SIZE.0, SCREEN_SIZE.1);
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("snake", "David Lundell")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0 * 1.0, SCREEN_SIZE.1 * 2.0),
        )
        // add resource path
        .add_resource_path(resource_dir)
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = &mut World::new(ctx).unwrap();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
