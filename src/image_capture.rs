/*
 * File : image capture.rs
 * Use  : captures an image for analysis
 */

 //imports
use std::time::Instant;
use xcap::Window;

/*
 * Function: capture game screenshot
 * Description: cycles through all the windows opens, finds Balatro and takes a picture
 * Parameters: n/a
 * ISSUE: currently takes three seconds, need to speed this up some how.
 */

pub fn capture_game_screen_shot () {
 
    //starts a timer and creates a vector of all the windows
    let start = Instant::now();
    let windows = Window::all().unwrap();

    //loops through all the windows to find balatro
    for window in windows {

        //if not balatro skip.
        if window.title() != "Balatro" {
            continue;
        }

        //outputs stats to terminal
        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );

        //saves image to ./target/balatro-Balatro.png for further editing
        let image = window.capture_image().unwrap();
        image.save(format!(
            "target/balatro-{}.png",
            window.title()
        ))
        .unwrap();

    }

    //prints time to take one screen shot of balatro
    println!("time elapsed: {:?}", start.elapsed());
 }