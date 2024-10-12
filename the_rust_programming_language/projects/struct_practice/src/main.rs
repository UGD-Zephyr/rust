struct IVLUser {
    active:         bool,
    username:       String,
    email:          String,
    sign_in_count:  u64,
}

fn main() {

    let user1 = IVLUser {
        active:         true,
        username:       String::from("per4345"),
        email:          String::from("per.stoor@ivl.se"),
        sign_in_count:  1,
    };

    if user1.active == true{
        println!("Username:             {}", user1.username);
        println!("email:                {}", user1.email);
        println!("number of sign-ins:   {}", user1.sign_in_count);

    };
}
