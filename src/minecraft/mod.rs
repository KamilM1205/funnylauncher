use crate::utils::constants::{MINECRAFT_FORGE, MINECRAFT_VERSION, WORKING_DIR};
use std::{
    error::Error,
    fs,
    process::{Child, Command},
};

use self::minecraft_json::MinecraftJson;

pub mod downloader;
pub mod minecraft_json;
mod tests;
pub mod validate;

pub const MINECRAFT: &str = "MINECRAFT";
// Arguments to launch java
pub struct JvmOptions {
    native_path: String,
    launcher_name: String,
    launcher_version: String,
    libs: String,
}

impl Default for JvmOptions {
    fn default() -> Self {
        Self {
            native_path: String::new(),
            launcher_name: "RuLauncher".to_string(),
            launcher_version: "OBT".to_string(),
            libs: String::new(),
        }
    }
}

impl JvmOptions {
    pub fn to_args(&self) -> Vec<String> {
        vec![
            "-Djava.library.path=".to_string() + &self.native_path,
            "-Dminecraft.launcher.brand=".to_string() + &self.launcher_name,
            "-Dminecraft.launcher.version=".to_string() + &self.launcher_version,
            "-cp".to_string(),
            self.libs.clone(),
        ]
    }
}

pub struct GameOptions {
    username: String,
    version: String,
    game_dir: String,
    assets_dir: String,
    assets_index: String,
    uuid: String,
    access_token: String,
    user_type: String,
    version_type: String,
    server: String,
    port: String,
}

// Launch arguments for game
impl Default for GameOptions {
    fn default() -> Self {
        Self {
            username: "MUTS04".to_string(),
            version: MINECRAFT_FORGE.to_string(),
            game_dir: String::new(),
            assets_dir: String::new(),
            assets_index: "".to_string(),
            uuid: "TmlsbA==".to_string(),
            access_token: "dummy_token".to_string(),
            user_type: "OFFLINE".to_string(),
            version_type: "release".to_string(),
            server: "localhost".to_string(),
            port: "25565".to_string(),
        }
    }
}

impl GameOptions {
    pub fn to_args(&self) -> Vec<String> {
        vec![
            "--username".to_string(),
            self.username.clone(),
            "--version".to_string(),
            self.version.clone(),
            "--gameDir".to_string(),
            self.game_dir.clone(),
            "--assetsDir".to_string(),
            self.assets_dir.clone(),
            "--assetIndex".to_string(),
            self.assets_index.clone(),
            "--uuid".to_string(),
            self.uuid.clone(),
            "--accessToken".to_string(),
            self.access_token.clone(),
            "--userType".to_string(),
            self.user_type.clone(),
            "--versionType".to_string(),
            self.version_type.clone(),
            "--server".to_string(),
            self.server.clone(),
            "--port".to_string(),
            self.port.clone(),
        ]
    }
}

// Struct for laucnhing game
pub struct Minecraft {
    //mc_data: MinecraftJson,
    forge_data: MinecraftJson,
    jvm_options: JvmOptions,
    game_options: GameOptions,
}

impl Minecraft {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let working_path = dirs::data_dir()
            .ok_or("OS data dir not found.")?
            .join(WORKING_DIR);

        let version_path = working_path.clone().join("versions");

        let forge_file = version_path
            .clone()
            .join(MINECRAFT_FORGE)
            .join(format!("{}.json", MINECRAFT_FORGE));

        let mc_file = version_path
            .clone()
            .join(MINECRAFT_VERSION)
            .join(format!("{}.json", MINECRAFT_VERSION));

        let forge_raw_data = match fs::read_to_string(&forge_file) {
            Ok(f) => f,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Couldn't open forge config file. Check your client.",
                )))
            }
        };
        let forge_data = MinecraftJson::new(&forge_raw_data)?;

        let mc_raw_data = match fs::read_to_string(&mc_file) {
            Ok(f) => f,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Couldn't open minecraft config file. Check your client.",
                )))
            }
        };
        let mc_data = MinecraftJson::new(&mc_raw_data)?;

        let mut jvm_options = JvmOptions::default();

        jvm_options.native_path = version_path
            .join(MINECRAFT_VERSION)
            .join("natives")
            .to_str()
            .ok_or("Path to native libraries not found. Check your client.")?
            .to_string();
        jvm_options.launcher_version = mc_data
            .minimum_launcher_version
            .unwrap_or_default()
            .to_string();

        let lib_path = working_path
            .clone()
            .join("libraries")
            .to_str()
            .ok_or("Libraries dir not found. Check your client.")?
            .to_string();

        let game_path = version_path
            .join(MINECRAFT_VERSION)
            .join(format!("{}.jar", MINECRAFT_VERSION))
            .to_str()
            .ok_or("Minecraft not found. Check your client.")?
            .to_string();

        jvm_options.libs = format!(
            "{}{}{}",
            forge_data.libs_to_args(&lib_path),
            mc_data.libs_to_args(&lib_path),
            game_path,
        );

        let mut game_options = GameOptions::default();

        game_options.game_dir = working_path
            .to_str()
            .ok_or("Work path not found. Check your client.")?
            .to_string();

        game_options.assets_dir = working_path
            .join("assets")
            .to_str()
            .ok_or("Asset path not found. Check your client.")?
            .to_string();
        game_options.assets_index = mc_data
            .asset_index
            .as_ref()
            .ok_or("Minecraft launch config file broken. Check your client.")?
            .id
            .clone();

        Ok(Self {
            //mc_data,
            forge_data,
            jvm_options,
            game_options,
        })
    }

    pub fn run(&self) -> Result<Child, Box<dyn Error>> {
        // javaw %jvm config% %jvm args% "%libs+minecraft%" %main_class% %forge args% %game args%
        let mut args: Vec<String> = Vec::new();

        args.append(&mut self.forge_data.jvm_args_to_arg());
        args.append(&mut self.jvm_options.to_args());
        args.push(self.forge_data.main_class.clone());
        args.append(&mut self.forge_data.game_args_to_arg());
        args.append(&mut self.game_options.to_args());

        let runtime_path = dirs::data_dir()
            .ok_or("OS data dir not found.")?
            .join(WORKING_DIR)
            .join("runtime")
            .join("bin")
            .join("java.exe");
        let runtime_path = runtime_path
            .to_str()
            .ok_or("Couldn't find runtime dir. Check your client.")?;

        let ret = match Command::new(runtime_path).args(args).spawn() {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Error while trying run minecraft: {e}")),
        }?;

        Ok(ret)
    }
}
