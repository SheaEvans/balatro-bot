/*
 * File : main.rs
 * Use  : The program loop of the software
 */

//file imports!
mod init_balatro;
mod image_capture;
// vv Basic Comment design.
/*
 * Function: 
 * Description: 
 * Parameters:
 */

 /*
 * Function: main
 * Description: this is the heart of the operation
 * Parameters: n/a
 */
fn main() {

    //initialized balatro
    init_balatro::init_balatro();

    //captures a screen shot of balatro
    image_capture::capture_game_screen_shot();

    //--println!("Making sure we got here!");



    // basic structure of how I am going to design the data base using comments
    
   
    //drop(start_balatro);
    
    /*
     * initialize_bot(){
     * 
     *      // starts the game for the user
     *      boot_balatro();
     * 
     *      // generate the screens overlay to communicate two ways with the user
     *      generate_overlay();
     *       
     *      // generate information to help make the bot able to act faster (e.g. joker information sheets)  
     *      generate_library_of_information_to_make_gameplay_faster();
     * 
     * }
     *          
     * 
     * run_gameplay_bot(){
     * 
     *      //run the bot till the game ends 
     *      loop {
     * 
     *          //display possible chip count for the best possible hand that could be played
     *          display_best_hand();
     *          
     *          // create a list of actions the user could do to set the course of the run
     *          // e.g. build towards straights, flushes, look for just the best course of action, etc.
     *          get_user_input(){
     *          
     *              //processes the users requests and makes the best decision.
     *              process_user_information();
     *              
     *          }
     *       
     *          // applys the users inputs and plays the best moves, accordingly until the next time that the users inoput is required.
     *          apply_user_input(){
     *              
     *          }
     * 
     *      }
     * 
     * }
     * 
     * //apply the last game information to update strategies, and best calls.
     * analyze_previous_gameplay(){
     * 
     * }
     */



}
