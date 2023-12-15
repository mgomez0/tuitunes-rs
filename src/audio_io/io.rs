use std::time::Duration;

use anyhow::Error;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;

fn get_stream() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    //select an input device
    let device = host
        .default_input_device()
        .ok_or_else(|| Error::msg("Failed to get host"))?;

    println!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("Failed to get input config");

    println!("Default input config: {:?}", config);
    let input_stream = device.build_input_stream(
        &config.into(),
        move |data, _: &_| process_audio_data(data),
        err_fn,
        Some(Duration::from_secs(5)),
    )?;

    input_stream.play()?;
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("Error with the audio stream: {:?}", err);
}

fn process_audio_data(data: &[f32]) {
    // Process the audio data here
    println!("Audio data: {:?}", data);
}
