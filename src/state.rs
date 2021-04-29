use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Time, transform::Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiImage, UiText,
        UiTransform,
    },
    window::ScreenDimensions,
};
use rand::Rng;


use amethyst::core::ecs::Entity;
use amethyst::ui::{Interactable, UiButton, UiButtonSystem, UiFinder, UiEvent, UiEventType};
use log::info;

/// A dummy game state that shows 3 sprites.
pub struct PlayState {
   pattern :  Vec<char>,
   entered : Vec<char>
}

impl PlayState { 
    pub fn new(pattern : Vec<char>) -> PlayState{
        PlayState {
            pattern,
            entered : vec![]
        }
    }
}

impl SimpleState for PlayState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);
        // this co-ordinate system is wacky.
        // for y, it seems to first priortize anchor type? and than use the coordinate as an offset?
        let square_size = 400.0;
        let padding = 100.0;
        //set color for button A
        let mut color = [0.2, 0.1, 0.5, 0.2];
        let mut hover_color = [0.7, 0.1, 0.0, 0.5];

        let button_a = make_our_button(
            world,
            (-1.0 * square_size) - padding,
            -100.0,
            square_size,
            square_size,
            "A",
            color,
            hover_color,0
        ); // these don't only make, they also add.
           //set color for button B
        color = [0.0, 0.0, 0.8, 0.5];
        let button_b = make_our_button(
            world,
            0.0,
            0.0,
            square_size,
            square_size,
            "B",
            color,
            hover_color,
            1
        );
        //set color for button c
        color = [0.0, 0.7, 0.2, 0.5];
        let button_c = make_our_button(
            world,
            (1.0 * square_size) + padding,
            100.0,
            square_size,
            square_size,
            "C",
            color,
            hover_color,
            2
        );


    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }
        let world = data.world;

        if let StateEvent::Ui(event) = &event {
            if event.event_type == UiEventType::Click{
                //let mut WriteStorageUITransform = world.write_storage::<UiTransform>();

                let buttons =  world.read_storage::<UiTransform>();
                let button = buttons.get(event.target).unwrap();
                println!("Clicked on button: {:?}",button.id);
                let mut image = world.write_storage::<UiImage>();
                let color = [0.0, 0.0, 0.0, 0.0];
                image.insert(event.target, UiImage::SolidColor(color));
                

                let mut press : Option<char> = None;
                if button.id == "1_btn"{
                    press = Some('A');
                }
                else if button.id == "2_btn"{
                    press = Some('B');
                }
                else if button.id == "3_btn"{
                    press = Some('C');
                }

                match press {
                    Some(c) => self.pattern.push(c),
                    None => {}
                }

                if self.pattern.len() == self.entered.len(){
                    if(self.pattern == self.entered){
                        return Trans::Push(Box::new(ShowState::new(Message::Win)));
                    }
                    else {
                        return Trans::Push(Box::new(ShowState::new(Message::Loss)));
                    }
                }
            }
        }

        // Keep going
        Trans::None
    }
}
fn make_our_button(
    world: &mut World,
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    button_text: &str,
    color: [f32; 4],
    hover_color: [f32; 4],
    id: u32
) -> UiButton {
    // Load our sprites and display them
    //let sprites = load_sprites(world);
    //init_sprites(world, &sprites, &dimensions);
    let (_button_id, button) = UiButtonBuilder::<(), u32>::new(button_text.to_string())
        .with_font_size(100.0)
        .with_position(x, y)
        .with_size(width, height)
        .with_anchor(Anchor::Middle)
        .with_image(UiImage::SolidColor(color))
        .with_id(id)
        .with_hover_image(UiImage::SolidColor(hover_color))
        .build_from_world(&world);
    return button;
}
/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);
    let camera = Camera::standard_2d(dimensions.width(), dimensions.height());
    world.create_entity().with(camera).with(transform).build();
}

/// Loads and splits the `logo.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/logo.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/logo.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..3)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

/// Creates an entity in the `world` for each of the provided `sprites`.
/// They are individually placed around the center of the screen.
fn init_sprites(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    for (i, sprite) in sprites.iter().enumerate() {
        // Center our sprites around the center of the window
        let x = (i as f32 - 1.) * 100. + dimensions.width() * 0.8;
        let y = (i as f32 - 1.) * 100. + dimensions.height() * 0.8;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();
    }
}

pub enum Message {
    Welcome, 
    Win,
    Loss
}

enum Showing {
    Nothing,
    Message,
    Pattern
}


pub struct ShowState{
    timer: Option<f32>,
    message : String,
    showing : Showing,
    pattern : Vec<char>,
    button : Option<UiButton>
}

fn gen_pattern(size : u32) -> Vec<char> {
    let mut pattern = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..size{
        let num = rng.gen_range(1..4);
        match num {
            1 => pattern.push('A'),
            2 => pattern.push('B'),
            _ => pattern.push('C')
        }
    }

    return pattern
}
impl ShowState{
    pub fn new(m : Message) -> ShowState{
        let message = match (m) {
            Message::Welcome => "welcome",
            Message::Win => "you win",
            Message::Loss => "you lose"
        };

        ShowState {
            timer : Some(1.0f32),
            message: String::from(message),
            showing :  Showing::Nothing,
            pattern : gen_pattern(8),
            button : None
        }
    }

    fn replace_text(&mut self, world : &mut World, text : String){
        let square_size = 400.0;
            //set color for button A
        let mut color = [0.2, 0.1, 0.5, 0.2];
        let mut hover_color = [0.7, 0.1, 0.0, 0.5];
        match self.button {
            Some(_) => {
                let button = self.button.take();
                world.delete_entity(button.unwrap().text_entity);
            },
            _ => {}
        }
    
        let label = make_our_button(
                    world,
                    0.0 ,
                    500.0,
                    square_size/2.0,
                    square_size * 6.0,
                    text.as_str(),
                    color,
                    hover_color,4
                );
        self.button = Some(label);
    }

}



impl SimpleState for ShowState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
            let world = data.world;

            // Get the screen dimensions so we can initialize the camera and
            // place our sprites correctly later. We'll clone this since we'll
            // pass the world mutably to the following functions.
            let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

            // Place the camera
            init_camera(world, &dimensions);

            
       
    }

    


    fn update(&mut self, state_data: &mut StateData<'_, GameData>) -> SimpleTrans {

        if let Some(mut timer) = self.timer.take() {
            // If the timer isn't expired yet, substract the time that passed since last update.
            {
                let time = state_data.world.read_resource::<Time>();
                timer -= time.delta_time().as_secs_f32();
            }
            if timer <= 0.0 {
                match self.showing {
                    Showing::Nothing => {
                        self.timer.replace(3.0f32);
                        println!("showing the message");
                        self.replace_text(state_data.world, self.message.clone());
                        self.showing = Showing::Message;
                    },
                    Showing::Message => {
                        self.timer.replace(5.0f32);
                        println!("showing the showing the pattern");
                        self.replace_text(state_data.world, self.pattern.iter().collect());
                        self.showing = Showing::Pattern;


                    },
                    Showing::Pattern => {
                        self.replace_text(state_data.world, String::from(""));
                        return Trans::Push(Box::new(PlayState::new(self.pattern.clone())));
                    }
                }
            } else {
                // If timer is not expired yet, put it back onto the state.
                self.timer.replace(timer);
            }
        }
        Trans::None

        
        }



}

//
// /// Creates a simple UI background and a UI text label
// /// This is the pure code only way to create UI with amethyst.
// pub fn create_ui_example(world: &mut World) {
//     // this creates the simple gray background UI element.
//     let ui_background = world
//         .create_entity()
//         .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
//         .with(UiTransform::new(
//             "".to_string(),
//             Anchor::TopLeft,
//             Anchor::TopLeft,
//             30.0,
//             -30.,
//             0.,
//             250.,
//             50.,
//         ))
//         .build();
//
//     // This simply loads a font from the asset folder and puts it in the world as a resource,
//     // we also get a ref to the font that we then can pass to the text label we crate later.
//     let font: FontHandle = world.read_resource::<Loader>().load(
//         "fonts/COMIC.TTF",
//         TtfFormat,
//         (),
//         &world.read_resource(),
//     );
//
//     // This creates the actual label and places it on the screen.
//     // Take note of the z position given, this ensures the label gets rendered above the background UI element.
//     world
//         .create_entity()
//         .with(UiTransform::new(
//             "".to_string(),
//             Anchor::TopLeft,
//             Anchor::TopLeft,
//             40.0,
//             -40.,
//             1.,
//             200.,
//             50.,
//         ))
//         .with(UiText::new(
//             font,
//             "Rust is hard.".to_string(),
//             [1., 1., 1., 1.],
//             30.,
//             LineMode::Single,
//             Anchor::TopLeft,
//         ))
//         .build();
//}
