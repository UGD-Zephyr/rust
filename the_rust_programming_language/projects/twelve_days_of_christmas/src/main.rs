fn main() {

    let days_of_christmas = ["first", "second", "third", "fourth", "fifth", "sixth", 
                             "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth", ];
    for loop_counter in 0..12 {
        primary_lyric(days_of_christmas[loop_counter].to_string());
        match loop_counter {
            0  => first_day_lyric(),
            1  => {
                second_day_lyric(); 
                first_day_lyric();
            }
            2  => {
                third_day_lyric(); 
                second_day_lyric(); 
                first_day_lyric();
            }
            3  => {
                fourth_day_lyric(); 
                third_day_lyric(); 
                second_day_lyric(); 
                first_day_lyric();
            }
            4  => {
                fifth_day_lyric(); 
                fourth_day_lyric(); 
                third_day_lyric(); 
                second_day_lyric(); 
                first_day_lyric();
            }
            5  => {
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            6  => {
                seventh_day_lyric();
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            7  => {
                eighth_day_lyric();
                seventh_day_lyric();
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            8  => {
                nineth_day_lyric();
                eighth_day_lyric();
                seventh_day_lyric();
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            9  => {
                tenth_day_lyric();
                nineth_day_lyric();
                eighth_day_lyric();
                seventh_day_lyric();
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            10 => {
                eleventh_day_lyric();
                tenth_day_lyric();
                nineth_day_lyric();
                eighth_day_lyric();
                seventh_day_lyric();
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            11 => {
                twelveth_day_lyric();
                eleventh_day_lyric();
                tenth_day_lyric();
                nineth_day_lyric();
                eighth_day_lyric();
                seventh_day_lyric();
                sixth_day_lyric();
                fifth_day_lyric();
                fourth_day_lyric();
                third_day_lyric();
                second_day_lyric();
                first_day_lyric();
            }
            _  => println!("Non valid case."),
        }
    }
    println!("Why am I so jolly?");
    println!("Hey! Who spiked the egg nog?");

    if cfg!(windows){
    std::process::Command::new("cmd").arg("/C").arg("pause").status().unwrap();
    }
}

fn primary_lyric(function_parameter: String) {
    println!("On the {} day of christmas, Blizzard gave to me", function_parameter );
}

fn first_day_lyric(){
    println!("A brand new SCV!\n");
}
fn second_day_lyric(){
    println!("Two Terran Wraiths and");
}
fn third_day_lyric(){
    println!("Three Marines, ");
}
fn fourth_day_lyric(){
    println!("Four Hydralisks, ");
}
fn fifth_day_lyric(){
    println!("Five newborn Queens, ");
}
fn sixth_day_lyric(){
    println!("Six Zealots fighting, ");
}
fn seventh_day_lyric(){
    println!("Seven Zerglings swarming, ");
}
fn eighth_day_lyric(){
    println!("Eight Archons burning, ");
}
fn nineth_day_lyric(){
    println!("Nine Battlecruisers, ");
}
fn tenth_day_lyric(){
    println!("Ten Ultralisks, ");
}
fn eleventh_day_lyric(){
    println!("Eleven Science Vessels, ");
}
fn twelveth_day_lyric(){
    println!("Twelve Arbiters, ");
}
