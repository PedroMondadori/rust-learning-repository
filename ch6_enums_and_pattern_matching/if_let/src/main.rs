fn verbose_control_flow(config_max: Option<u8>) {
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

fn concise_control_flow(config_max: Option<u8>) {
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    else {
        ();
    }
}

fn main() {
    verbose_control_flow(Some(3u8));
    concise_control_flow(Some(3u8));
    concise_control_flow(None);
}
