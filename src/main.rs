/*
By: <Smith>
Date: 2026-02-19
Program Details: <Simple 6 button gui with 5 different images that show when each different 
button is pressed, an exit button, and a text box describing each image.>
*/

mod modules;

use macroquad::{prelude::*, rand};
use crate::modules::grid::draw_grid;
use crate::modules::still_image;
use crate::modules::text_button::TextButton;
 use crate::modules::label::Label;
 use crate::modules::still_image::StillImage;
   use crate::modules::preload_image::TextureManager;
    use crate::modules::preload_image::LoadingScreenOptions; // If you want to customize the loading screen

use miniquad::date;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "image selector".to_string(),
        window_width: 850,
        window_height: 750,
        fullscreen: false,
        high_dpi: true,
        window_resizable: false,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    rand::srand(date::now() as u64);
    let mut tm = TextureManager::new();
   let loading_options = LoadingScreenOptions {
       title: Some("Seele's Image UI".to_string()),
       background_color: DARKPURPLE,
       bar_fill_color: GOLD,
       // Use default values for other options
       ..Default::default()
   };

   
tm.preload_with_loading_screen(&["assets/watermark.png", "assets/placeholder.png", "assets/stickmanwithhat.png", "assets/catsleeping.png", "assets/lakesunrise.png", "assets/rockymountains.png", "assets/iceoncar.png"], Some(loading_options)).await;
let mut image_box = StillImage::new("", 450.0, 300.0, 350.0, 50.0, true, 1.0).await;    
let mut backgroundimage: still_image::StillImage = still_image::StillImage::new("", 850.0, 750.0, 0.0, 0.0, true, 1.0).await;

                let mut lbl_msg = Label::new("click a button to start", 355.0, 430.0, 30);
                    lbl_msg.with_fixed_size(450.0, 300.0);
                    lbl_msg.with_border(BLACK, 5.0);
            backgroundimage.set_preload(tm.get_preload("assets/watermark.png").unwrap());
            image_box.set_preload(tm.get_preload("assets/placeholder.png").unwrap());
        

    let mut btn_1 = TextButton::new(
        50.0,
        50.0,
        250.0,
        50.0,
        "Picture 1",
        WHITE,
        GRAY,
        30
    );
    btn_1.with_border(GRAY, 12.5);
    btn_1.with_text_color(BLACK);        // Sets the normal text color
    btn_1.with_hover_text_color(WHITE);  // Sets the text color when hovering
    let mut btn_2 = TextButton::new(
        50.0,
        112.5,
        250.0,
        50.0,
        "Picture 2",
        WHITE,
        GRAY,
        30
    );
    btn_2.with_border(GRAY, 12.5);
       btn_2.with_text_color(BLACK);        // Sets the normal text color
    btn_2.with_hover_text_color(WHITE);  // Sets the text color when hovering
    let mut btn_3 = TextButton::new(
        50.0,
        175.0,
        250.0,
        50.0,
        "Picture 3",
        WHITE,
        GRAY,
        30
    );
    btn_3.with_border(GRAY, 12.5);
       btn_3.with_text_color(BLACK);        // Sets the normal text color
    btn_3.with_hover_text_color(WHITE);  // Sets the text color when hovering
    let mut btn_4 = TextButton::new(
        50.0,
        237.5,
        250.0,
        50.0,
        "Picture 4",
        WHITE,
        GRAY,
        30
    );
    btn_4.with_border(GRAY, 12.5);
       btn_4.with_text_color(BLACK);        // Sets the normal text color
    btn_4.with_hover_text_color(WHITE);  // Sets the text color when hovering
    let mut btn_5 = TextButton::new(
        50.0,
        300.0,
        250.0,
        50.0,
        "Picture 5",
        WHITE,
        GRAY,
        30
    );
    btn_5.with_border(GRAY, 12.5);
       btn_5.with_text_color(BLACK);        // Sets the normal text color
    btn_5.with_hover_text_color(WHITE);  // Sets the text color when hovering
      let mut btn_random = TextButton::new(
        50.0,
        400.0,
        250.0,
        50.0,
        "Random Picture",
        WHITE,
        GRAY,
        30
    );
    btn_random.with_border(GRAY, 12.5);
       btn_random.with_text_color(BLACK);        // Sets the normal text color
    btn_random.with_hover_text_color(WHITE);  // Sets the text color when hovering
    let mut btn_exit = TextButton::new(
            50.0, 
            650.0, 
            250.0, 
            52.5, 
            "Exit", 
            WHITE, 
            RED, 
            36
    );
    btn_exit.with_border(RED, 12.5);
       btn_exit.with_text_color(BLACK);        // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);  // Sets the text color when hovering  



    loop {
        clear_background(WHITE);
    
        
        backgroundimage.draw();
       
     (lbl_msg, image_box, tm) = buttons(&btn_1, tm, &btn_2, &btn_3, &btn_4, &btn_5, &btn_random, &btn_exit, lbl_msg, image_box);
        lbl_msg.draw();
        image_box.draw();
        next_frame().await;
    }
}



fn buttons(btn_1: &TextButton, tm: TextureManager, btn_2: &TextButton, btn_3: &TextButton, btn_4: &TextButton, btn_5: &TextButton, btn_random: &TextButton, btn_exit: &TextButton,mut lbl_msg: Label, mut image_box: StillImage) -> (Label, StillImage, TextureManager){ // checks if buttons are clicked and changes image variable accordingly
    if btn_1.click() {
        lbl_msg.set_text(" A cat sleeping on a blanket");
        image_box.set_preload(tm.get_preload("assets/catsleeping.png").unwrap());
    }
     if btn_2.click() {
        lbl_msg.set_text(" Ice on a car door after\n freezing rain");
        image_box.set_preload(tm.get_preload("assets/iceoncar.png").unwrap());
    }
     if btn_3.click() {
        lbl_msg.set_text(" The Canadian rocky mountains");
        image_box.set_preload(tm.get_preload("assets/rockymountains.png").unwrap());
    }
     if btn_4.click() {
        lbl_msg.set_text(" A drawing of a stick figure\n with a hat");
        image_box.set_preload(tm.get_preload("assets/stickmanwithhat.png").unwrap());
    }
     if btn_5.click() {
        lbl_msg.set_text(" A sunrise on the lake");
        image_box.set_preload(tm.get_preload("assets/lakesunrise.png").unwrap());
    }
    if btn_random.click() {
       let random_int = rand::gen_range(1, 6);


        lbl_msg.set_text(match random_int {
            1 => " A cat sleeping on a blanket",
            2 => " Ice on a car door after\n freezing rain",
            3 => " The Canadian rocky mountains",
            4 => " A drawing of a stick figure\n with a hat",
            5 => " A sunrise on the lake",
            _ => "Error: Invalid random number",
            
        });
        image_box.set_preload(match random_int {
            1 => tm.get_preload("assets/catsleeping.png").unwrap(),
            2 => tm.get_preload("assets/iceoncar.png").unwrap(),
            3 => tm.get_preload("assets/rockymountains.png").unwrap(),
            4 => tm.get_preload("assets/stickmanwithhat.png").unwrap(),
            5 => tm.get_preload("assets/lakesunrise.png").unwrap(),
            _ => tm.get_preload("assets/placeholder.png").unwrap(), // Default case, should never happen
        });
    }
     if btn_exit.click() {
        std::process::exit(0);
    }
    (lbl_msg, image_box, tm)
}
