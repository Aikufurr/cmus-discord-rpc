use std::env;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

use env_logger;
use log::{debug, info};
use regex::Regex;

#[derive(PartialEq, Debug)]
enum Status {
    Playing,
    Paused,
    Stopped
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct ParseStatusError;

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "playing" => Ok(Status::Playing),
            "paused" => Ok(Status::Paused),
            "stopped" => Ok(Status::Stopped),
            _ => Err(ParseStatusError)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting cmus-discord-rpc...");

    let socket_path = get_socket_path();
    debug!("Using cmus socket {}", socket_path);
    let mut stream = get_unix_stream(&socket_path);
    let mut drpc = DiscordIpcClient::new("953758667369513040")?;
    drpc.connect()?;
    let mut connected = true;

    let mut output = String::new();

    let mut last_paused = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut logged_last_paused = true;

    loop {
        if stream.write_all(b"status\n").is_err() {
            drpc.close()?;
            connected = false;
            info!("Failed to clear presence");
            stream = get_unix_stream(&socket_path);
            continue;
        } else {
            if !connected {
                drpc.reconnect()?;
                connected = true
            }
        }

        let mut reader = BufReader::new(&stream);
        output.clear();

        // Read until an empty line
        while reader.read_line(&mut output).unwrap() != 1 {};
        debug!("Received\n{}", output);

        let status = get_value(&output, "status").unwrap().parse::<Status>().unwrap();
        let mut ac = activity::Activity::new();
        let mut url_string: String;
        let mut assets = activity::Assets::new();
        let album_url: String;
        let top_line: String;

        if status != Status::Stopped {
            let artist = get_value(&output, "tag artist");
            let title = get_value(&output, "tag title");

            if artist.is_none() || title.is_none() {
                // Capture filename
                let file_r = Regex::new(r"(?m)^file .+/(.+)\..+\n").unwrap();
                match file_r.captures(&output) {
                    Some(v) => ac = ac.state(v.get(1).unwrap().as_str()),
                    None => ac = ac.state("")
                }
            } else {
                ac = ac.details(title.unwrap());
                ac = ac.state(artist.unwrap());
            }

            // -- Album cover --
            // Capture folder of artist and folder
            let file_r = Regex::new(r"(?mU)^file *.+Music/(.+)/(.+)/(.+)/").unwrap();
            let base_folder: &str;
            let artist_folder: &str;
            let album_folder: &str;
            match file_r.captures(&output) {
                Some(v) => {base_folder = v.get(1).unwrap().as_str(); artist_folder = v.get(2).unwrap().as_str(); album_folder = v.get(3).unwrap().as_str()},
                None => {base_folder = ""; artist_folder = ""; album_folder = ""}
            }

            if !(artist_folder.is_empty() || album_folder.is_empty()) {
                url_string = format!("https://aikufurr.xyz/presence?{}cmus_artist={}&cmus_album={}", if base_folder.is_empty() {String::from("")} else {"cmus_base=".to_owned() + base_folder + "&"}, artist_folder, album_folder);
                url_string = url_string.replace(" ", "%20");
                let url: &str = &url_string;
                assets = assets.large_image(url);
                assets = assets.large_text(get_value(&output, "tag album").unwrap());
                ac = ac.assets(assets);
                if base_folder == "Lapfox" {
                    let mut buttons: Vec<activity::Button> = vec![];

                    let file_path: String = format!("/home/aikufurr/Music/Lapfox/{artist_folder}/{album_folder}/link.txt");
                    if std::path::Path::new(&file_path).exists() {
                        album_url = std::fs::read_to_string(file_path)?.parse()?;
                        let stripped_url: &str;
                        match album_url.lines().next() {
                            Some(v) => stripped_url = v,
                            None => stripped_url = ""
                        }
                        if !stripped_url.is_empty() {
                            buttons.push(activity::Button::new("View Album", stripped_url));
                        }
                    }

                    buttons.push(activity::Button::new("Halley Labs", "https://halleylabs.com/"));

                    ac = ac.buttons(buttons);
                }
            }

            if status == Status::Playing {
                logged_last_paused = false;
                let duration = get_value(&output, "duration").unwrap().parse::<u64>().unwrap();
                let position = get_value(&output, "position").unwrap().parse::<u64>().unwrap();
                let sce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let num:u64 = sce + duration - position;
                let ts = activity::Timestamps::new().end(num as i64);
                ac = ac.timestamps(ts);
            } else if status == Status::Paused {
                if !logged_last_paused {
                    last_paused = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    logged_last_paused = true;
                }

                top_line = title.unwrap().to_owned() + " - " + artist.unwrap();
                ac = ac.details(&top_line);
                ac = ac.state("Paused");
                let ts = activity::Timestamps::new().start(last_paused as i64);
                ac = ac.timestamps(ts);
            }
        } else {
            assets = assets.large_image("https://aikufurr.xyz/presence?cmus_artist=cmus&cmus_album=cmus");
            assets = assets.large_text("C* Music Player (cmus)");
            ac = ac.assets(assets);
        }

        drpc.set_activity(ac).expect("Failed to set presence");

        thread::sleep(Duration::from_secs(5));
    }
}

fn get_unix_stream(socket_path: &str) -> UnixStream {
    loop {
        if let Ok(s) = UnixStream::connect(socket_path) {
            return s;
        }

        // Try again in 15 seconds
        thread::sleep(Duration::from_secs(15));
    }
}

/// Get the path to the cmus socket the same way as cmus itself
fn get_socket_path() -> String
{
    if let Ok(v) = env::var("CMUS_SOCKET") {
        return v;
    }

    if let Ok(v) = env::var("XDG_RUNTIME_DIR") {
        return v + "/cmus-socket";
    }

    let cmus_config_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(v) => v,
        Err(_) => env::var("HOME").unwrap() + "/.config"
    } + "/cmus";

    cmus_config_dir + "/socket"
}

fn get_value<'t>(input: &'t str, key: &str) -> Option<&'t str> {
    let re = Regex::new(&format!("(?m)^{} (.+)$", key)).unwrap();

    Some(re.captures(input)?.get(1)?.as_str())
}
