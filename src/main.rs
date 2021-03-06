use enigo::{Enigo, KeyboardControllable, Key};
use std::{thread, time};
use clap::{Arg, App, SubCommand};

fn main() {
    // Process launch arguments with clap
    let matches = App::new("Honeycomb - Chunk Pregenerator")
        .version("1.0")
        .author("IronFist95 & codedcosmos (codedcosmos.mail@gmail.com)")
        .about("Automatically types teleport commands to pre-generate a minecraft world.")

        .arg(Arg::with_name("Input Delay")
            .value_name("Input Delay")
            .short("d")
            .long("input_delay")
            .help("Sets the delay between keypresses (milliseconds)")
            .takes_value(true)
            .default_value("40"))

        .arg(Arg::with_name("Delay Between Teleports")
            .value_name("Delay Between Teleports")
            .short("t")
            .long("teleport_delay")
            .help("Sets the delay between teleports (milliseconds)")
            .takes_value(true)
            .default_value("12000"))

        .arg(Arg::with_name("Start Delay time")
            .value_name("Start Delay time")
            .short("e")
            .long("start_delay")
            .help("Sets the delay before starting (seconds)")
            .takes_value(true)
            .default_value("5"))

        .arg(Arg::with_name("Automatic Creative Mode")
            .value_name("Automatic Creative Mode")
            .short("c")
            .long("creative_mode")
            .help("Starts the script by setting the players gamemode to creative")
            .takes_value(false))

        .arg(Arg::with_name("Automatic Spectator Mode")
            .value_name("Automatic Spectator Mode")
            .short("s")
            .long("spectator_mode")
            .help("Starts the script by setting the players gamemode to spectator")
            .takes_value(false))

        .arg(Arg::with_name("Auto Return")
            .value_name("Auto Return")
            .short("r")
            .long("auto_return")
            .help("Automatically returns to spawn")
            .takes_value(false))

        .arg(Arg::with_name("Start X")
            .value_name("Start X")
            .short("x")
            .long("start_x")
            .help("Sets the starting location on the x axis")
            .takes_value(true)
            .default_value("0"))

        .arg(Arg::with_name("Start Y")
            .value_name("Start Y")
            .short("y")
            .long("start_y")
            .help("Sets the starting location on the y axis")
            .takes_value(true)
            .default_value("0"))

        .arg(Arg::with_name("View Distance")
            .value_name("View Distance")
            .short("v")
            .long("view_distance")
            .help("Used to calculate how far to teleport the player")
            .takes_value(true)
            .default_value("12"))

        .arg(Arg::with_name("View Buffer")
            .value_name("View Buffer")
            .short("b")
            .long("view_buffer")
            .help("Safety Buffer distance, to help make sure all chunks are loaded")
            .takes_value(true)
            .default_value("0.9"))

        .arg(Arg::with_name("Pregenerate Distance")
            .value_name("Pregenerate Distance")
            .short("p")
            .long("pregenerate_distance")
            .help("Used to set how far to pregenerate (blocks). If set to 0, continues until manually stopped.")
            .takes_value(true)
            .default_value("1024"))

        .get_matches();

    // Load data from arguments
    let input_delay = matches.value_of("Input Delay").unwrap_or_default();
    let input_delay = input_delay.parse::<u64>().expect("input_delay must be a positive integer!");
    let input_delay_micro = input_delay * 1000;

    let teleport_delay = matches.value_of("Delay Between Teleports").unwrap_or_default();
    let teleport_delay = teleport_delay.parse::<u64>().expect("teleport_delay must be a positive integer!");

    let start_delay = matches.value_of("Start Delay time").unwrap_or_default();
    let start_delay = start_delay.parse::<u64>().expect("start_delay must be a positive integer!");

    let creative_mode = matches.is_present("Automatic Creative Mode");
    let spectator_mode = matches.is_present("Automatic Spectator Mode");
    if creative_mode && spectator_mode {
        println!("Player cannot be in creative mode and spectator mode at the same time");
        println!("Please use either -c or -s");
        return;
    }

    let auto_return = matches.is_present("Auto Return");

    let start_x = matches.value_of("Start X").unwrap_or_default();
    let start_x = start_x.parse::<i64>().expect("start_x must be an integer!");

    let start_y = matches.value_of("Start Y").unwrap_or_default();
    let start_y = start_y.parse::<i64>().expect("start_y must be an integer!");

    let view_distance = matches.value_of("View Distance").unwrap_or_default();
    let view_distance = view_distance.parse::<u64>().expect("view_distance must be a positive integer!");
    if view_distance <= 1 {
        println!("view_distance really should be higher than 1");
        return;
    }

    let view_buffer = matches.value_of("View Buffer").unwrap_or_default();
    let view_buffer = view_buffer.parse::<f64>().expect("view_buffer must be a decimal value!");
    if view_buffer < 0.1 || view_buffer > 1.0 {
        println!("view_buffer needs to be between 0.1 and 1.0");
        return;
    }

    let pregenerate_distance = matches.value_of("Pregenerate Distance").unwrap_or_default();
    let pregenerate_distance = pregenerate_distance.parse::<u64>().expect("pregenerate_distance must be a positive integer!");
    let pregenerate_distance = pregenerate_distance as i64;

    // Wait Block
    println!("Starting up honeycomb");
    for i in 0..start_delay {
        println!("Waiting {} seconds before starting", start_delay-i);
        thread::sleep(time::Duration::from_millis(1000));
    }

    // Setup enigo
    let mut enigo = Enigo::new();
    enigo.set_delay(input_delay_micro);

    // Set player gamemode
    if creative_mode {
        println!("Setting player to be in creative mode");
        execute_command(&mut enigo, input_delay, "/gamemode creative @s".to_string());
    } else if spectator_mode {
        println!("Setting player to be in survival mode");
        execute_command(&mut enigo, input_delay, "/gamemode creative @s".to_string());
    }

    // Configurable
    let chunk_size = 16.0;
    let look_dist = view_distance as f64 * 2.0;
    let hexagon_size = chunk_size * look_dist * view_buffer;

    let mut x = start_x as f64;
    let mut y = start_y as f64;
    let mut step_amount = 0;

    let long = 60.0_f64.to_radians().sin() * hexagon_size;
    let short = 60.0_f64.to_radians().cos() * hexagon_size;

    let up_left = (short, -long);
    let up = (hexagon_size, 0.0);
    let up_right = (short, long);
    let down_right = (-short, long);
    let down = (-hexagon_size, 0.0);
    let down_left = (-short, -long);

    // Iterating distance
    let mut current_distance = 0.0;

    while pregenerate_distance == 0 || current_distance < pregenerate_distance as f64 {
        x = start_x as f64;
        y = start_y as f64;

        // Move to Bottom
        for s in 0..step_amount {
            x += down.0;
            y += down.1;
        }

        // Calculate distance
        current_distance = ((start_x as f64 - x).powf(2.0) + (start_y as f64 - y).powf(2.0)).sqrt();

        // Start case
        println!("Teleporting to origin of current hexagon, current step_amount: {}", step_amount);
        teleport(&mut enigo, input_delay, teleport_delay, x as i64, y as i64);

        // Go around
        perform_dir(&mut enigo, input_delay, teleport_delay, &mut x, &mut y, step_amount, up_left, current_distance, pregenerate_distance, 0);
        perform_dir(&mut enigo, input_delay, teleport_delay, &mut x, &mut y, step_amount, up, current_distance, pregenerate_distance, 1);
        perform_dir(&mut enigo, input_delay, teleport_delay, &mut x, &mut y, step_amount, up_right, current_distance, pregenerate_distance, 2);
        perform_dir(&mut enigo, input_delay, teleport_delay, &mut x, &mut y, step_amount, down_right, current_distance, pregenerate_distance, 3);
        perform_dir(&mut enigo, input_delay, teleport_delay, &mut x, &mut y, step_amount, down, current_distance, pregenerate_distance, 4);
        perform_dir(&mut enigo, input_delay, teleport_delay, &mut x, &mut y, step_amount-1, down_left, current_distance, pregenerate_distance, 5);

        step_amount += 1;

        let delay = time::Duration::from_millis(1000);
        thread::sleep(delay);
    }

    if auto_return {
        println!("Returning player back to origin");
        teleport_with_rot(&mut enigo, input_delay, teleport_delay, start_x, start_y, 0.0);
    }

    println!("Completed");
}

fn perform_dir(mut enigo: &mut Enigo, input_delay: u64, teleport_delay: u64, x: &mut f64, y: &mut f64, step_amount: i64, dir: (f64, f64), current_distance: f64, pregenerate_distance: i64, step_index: i64) {
    let current_distance = current_distance as i64;
    let pregenerate_distance = pregenerate_distance as i64;

    let total_steps = step_amount*6;

    for s in 0..step_amount {
        *x += dir.0;
        *y += dir.1;

        let x = x.clone() as i64;
        let y = y.clone() as i64;

        let current_step = step_amount*step_index + s;

        println!("Current distance: {}/{} -=- step: {}/{} -=- x: {} -=- y: {}", current_distance, pregenerate_distance, current_step, total_steps, x, y);

        teleport(enigo, input_delay, teleport_delay, x, y);
    }
}

fn teleport(mut enigo: &mut Enigo, input_delay: u64, teleport_delay: u64, x: i64, y: i64) {
    teleport_with_rot(enigo, input_delay, teleport_delay, x, y, 0.0);
    teleport_with_rot(enigo, input_delay, teleport_delay, x, y, 90.0);
    teleport_with_rot(enigo, input_delay, teleport_delay, x, y, 180.0);
    teleport_with_rot(enigo, input_delay, teleport_delay, x, y, 270.0);
}

fn teleport_with_rot(enigo: &mut Enigo, input_delay: u64, teleport_delay: u64, x: i64, y: i64, rot: f64) {
    let command_text = format!("/tp @s {} {} {} {} 45.0", x, 160.0, y, rot);

    execute_command(enigo, input_delay, command_text);

    thread::sleep(time::Duration::from_millis(teleport_delay));
}

fn execute_command(enigo: &mut Enigo, input_delay: u64, command_text: String) {
    thread::sleep(time::Duration::from_millis(input_delay));

    enigo.key_click(Key::Return);

    thread::sleep(time::Duration::from_millis(input_delay * 2));

    let delay = (command_text.len()+1) as u64 * input_delay;
    enigo.key_sequence(command_text.as_str());

    thread::sleep(time::Duration::from_millis(delay));

    enigo.key_click(Key::Return);
}