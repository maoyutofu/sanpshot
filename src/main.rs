use std::process::Stdio;
use std::{sync::Arc, thread, time::Duration};

use sanpshot::cmd;
use sanpshot::config;
use sanpshot::util;
use tracing_log::log::{error, info};

fn execute(src: config::Src, local_storage: Arc<String>, ffmpeg: Arc<String>) {
    loop {
        thread::sleep(Duration::from_secs(src.frequency));
        let url = src.url.clone();

        let label = format!("{}/{}", src.id, src.name);

        let file_name = util::generate_file_name();
        let local_storage = &(*local_storage);

        if let Some(dir) = util::file_path_join(&local_storage, &src.id) {
            if let Some(file_path) = util::file_path_join(&dir, &file_name) {
                if let Err(e) = util::create_dir(&file_path) {
                    error!("Create dir {}, {}", dir, e);
                    return;
                }

                let command_builder = cmd::CmdBuilder::new(&ffmpeg)
                    .stderr(Stdio::piped())
                    .option(cmd::Parameter::KeyValue(
                        String::from("-loglevel"),
                        String::from("quiet"),
                    ))
                    .option(cmd::Parameter::Single(String::from("-y")))
                    .option(cmd::Parameter::Single(String::from("-i")))
                    .option(cmd::Parameter::Single(url))
                    .option(cmd::Parameter::KeyValue(
                        String::from("-f"),
                        String::from("image2"),
                    ))
                    .option(cmd::Parameter::KeyValue(
                        String::from("-frames:v"),
                        String::from("1"),
                    ))
                    .option(cmd::Parameter::Single(file_path));

                info!("{} {}", label, command_builder.to_string());

                let command = match command_builder.run() {
                    Ok(process) => process,
                    Err(e) => panic!("{} {}", label, e),
                };

                let output = match command.process.wait_with_output() {
                    Ok(x) => x,
                    Err(e) => panic!("{} {}", label, e),
                };

                if !output.status.success() {
                    match std::str::from_utf8(&output.stderr) {
                        Ok(_) => {}
                        Err(e) => error!("{} {}", label, e),
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    info!("{}", "Load configuration file");
    let conf = config::Config::new()?;

    info!("{}", "Application started successfully");
    info!("{}", "Waiting for task execution");

    let ffmpeg_arc = Arc::new(conf.ffmpeg);
    let local_storage_arc = Arc::new(conf.storage.local);
    let handles: Vec<_> = conf
        .src
        .into_iter()
        .map(|src| {
            let ffmpeg_arc = Arc::clone(&ffmpeg_arc);
            let local_storage_arc = Arc::clone(&local_storage_arc);
            thread::spawn(move || execute(src, local_storage_arc, ffmpeg_arc))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
