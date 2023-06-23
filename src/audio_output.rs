use std::io;
use tts::*;


pub fn speak() -> Result<(), Error> {
    env_logger::init();
    let mut tts = Tts::default()?;
    if Tts::screen_reader_available() {
        println!("A screen reader is available on this platform.");
    } else {
        println!("No screen reader is available on this platform.");
    }
    let Features {
        utterance_callbacks,
        ..
    } = tts.supported_features();
    if utterance_callbacks {
        tts.on_utterance_begin(Some(Box::new(|utterance| {
            println!("Started speaking {:?}", utterance)
        })))?;
        tts.on_utterance_end(Some(Box::new(|utterance| {
            println!("Finished speaking {:?}", utterance)
        })))?;
        tts.on_utterance_stop(Some(Box::new(|utterance| {
            println!("Stopped speaking {:?}", utterance)
        })))?;
    }


    // Check if the bot is speaking
    let Features { is_speaking, .. } = tts.supported_features();
    if is_speaking {
        println!("Are we speaking? {}", tts.is_speaking()?);
    }

    // Check if it is rate
    let Features { rate, .. } = tts.supported_features();
    if rate {
        tts.set_rate(tts.normal_rate())?;
    }

    tts.speak("Hello, this is me, Oh you dont know my name, its PROTON, nice to meet you.", false)?;
    tts.speak("happy birthday too you, happy birthday to you, happy birthday DPS, happy birthday to you", false)?;
    let Features { pitch, .. } = tts.supported_features();

/* 

    Use this feature to change the pitch
    if pitch {
        let original_pitch = tts.get_pitch()?;
        tts.set_pitch(tts.max_pitch())?;
        tts.speak("This is high-pitch.", false)?;
        tts.set_pitch(tts.min_pitch())?;
        tts.speak("This is low pitch.", false)?;
        tts.set_pitch(tts.normal_pitch())?;
        tts.speak("This is normal pitch.", false)?;
        tts.set_pitch(original_pitch)?;
    }

*/

/* 
    Use this to change the volume
    let Features { volume, .. } = tts.supported_features();
    if volume {
        let original_volume = tts.get_volume()?;
        tts.set_volume(tts.max_volume())?;
        tts.speak("This is loud!", false)?;
        tts.set_volume(tts.min_volume())?;
        tts.speak("This is quiet.", false)?;
        tts.set_volume(tts.normal_volume())?;
        tts.speak("This is normal volume.", false)?;
        tts.set_volume(original_volume)?;
    }
*/


    // List the available voices
    let Features { voice, .. } = tts.supported_features();
    if voice {
        let voices = tts.voices()?;
        println!("Available voices:\n===");
        for v in &voices {
            println!("{:?}", v);
        }
        let Features { get_voice, .. } = tts.supported_features();
        let original_voice = if get_voice { tts.voice()? } else { None };
        for v in &voices {
            tts.set_voice(v)?;
            tts.speak(format!("This is {}.", v.name()), false)?;
        }
        if let Some(original_voice) = original_voice {
            tts.set_voice(&original_voice)?;
        }
    }

    tts.speak("Goodbye.", false)?;
    let mut _input = String::new();

    io::stdin().read_line(&mut _input)?;
    Ok(())
}