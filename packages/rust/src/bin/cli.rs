use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let selections = &[
        "CRISP: Voting Protocol (ETH)",
        "More Coming Soon!"
    ];

    let selections_2 = &[
        "Initialize new CRISP round.",
        "Continue Existing CRISP round."
    ];

    let selection_1 = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Enclave (EEEE): Please choose the private execution environment you would like to run!")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if(selection_1 == 0){
    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    	println!("Encrypted Protocol Selected {}!", selections[selection_1]);
	    let selection_2 = FuzzySelect::with_theme(&ColorfulTheme::default())
	        .with_prompt("Create a new CRISP round or particpate in an existing round.")
	        .default(0)
	        .items(&selections_2[..])
	        .interact()
	        .unwrap();

	    if(selection_2 == 0){
	    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	    	println!("Starting new CRISP round!");
		    let input_token: String = Input::with_theme(&ColorfulTheme::default())
		        .with_prompt("Enter Proposal Registration Token")
		        .interact_text()
		        .unwrap();
		    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		    println!("Reading proposal details from config and calling contract...");
	        let three_seconds = time::Duration::from_millis(3000);
	        thread::sleep(three_seconds);
	        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Initializing Keyshare nodes...");
            // call init on server
            // have nodes poll
	    	println!("Gathering Keyshare nodes for execution environment...");
	    }
	    if(selection_2 == 1){
	    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		    let input_crisp_id: String = Input::with_theme(&ColorfulTheme::default())
		        .with_prompt("Enter CRISP round ID.")
		        .interact_text()
		        .unwrap();
	    }

    }
    if(selection_1 == 1){
    	println!("Check back soon!");
    	std::process::exit(1);
    }

    // println!("Hello {}!", input);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your email")
    //     .validate_with({
    //         let mut force = None;
    //         move |input: &String| -> Result<(), &str> {
    //             if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
    //                 Ok(())
    //             } else {
    //                 force = Some(input.clone());
    //                 Err("This is not a mail address; type the same value again to force use")
    //             }
    //         }
    //     })
    //     .interact_text()
    //     .unwrap();

    // println!("Email: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your planet")
    //     .default("Earth".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Planet: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your galaxy")
    //     .with_initial_text("Milky Way".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Galaxy: {}", mail);
    Ok(())
}
