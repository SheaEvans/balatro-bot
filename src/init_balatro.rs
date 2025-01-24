/*
 * File : init balatro.rs
 * Use  : prints information to command line and opens balatro on steam 
 */

//imports
use std::process::Command;


/*
 * Function: init balatro
 * Description: prints information to command line and opens balatro on steam.
 * Parameters: n/a
 */
pub fn init_balatro(){

    //prints text to the command line
    println!("####################################################################");
    println!("############################# SO EXCITING! #########################");
    println!("####################################################################");

    // starts the program steam with the arg of Balatro game id, and if fails prints message.
    // ISSUE: ONLY WORKS IF STEAM IS INSTALLED IN THE DEFAULT LOCATION!!!! NEED TO ADD A PROMPT TO CHANGE THIS!
    let start_balatro = Command::new("C:\\Program Files (x86)\\Steam\\steam.exe")
                      .arg("steam://rungameid/2379780")
                      .output()
                      .expect("Failed to open Balatro! Please try again, or open Balatro for me!");

    // runs the commands, then the garbage truck cleans up!           
    drop(start_balatro);

}