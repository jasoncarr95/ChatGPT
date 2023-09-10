use crate::utils;
use log::error;
use std::{fs, path::PathBuf};
use tauri::{api, command, AppHandle, Manager};

#[command]
pub fn drag_window(app: AppHandle) {
  app.get_window("core").unwrap().start_dragging().unwrap();
}

#[command]
pub fn fullscreen(app: AppHandle) {
  let win = app.get_window("core").unwrap();
  if win.is_fullscreen().unwrap() {
    win.set_fullscreen(false).unwrap();
  } else {
    win.set_fullscreen(true).unwrap();
  }
}

/** Add function to check if file exists before saving */
use std::path::Path;

// #[tauri::command]
// pub fn file_exists(path: String) -> bool {
//   println!("Checking path: {}", path); // Debug: Log the path being checked
//   Path::new(&path).exists()
//   // Path::new(&path).is_file()
// }
#[tauri::command]
pub fn file_exists(name: String) -> bool {
  let path = utils::app_root().join(PathBuf::from(name));
  Path::new(&path).exists()
}

// #[command]
// pub fn download(app: AppHandle, name: String, blob: Vec<u8>) {
//   let win = app.app_handle().get_window("core");
//   let path = utils::app_root().join(PathBuf::from(name));
//   utils::create_file(&path).unwrap();
//   fs::write(&path, blob).unwrap();
//   tauri::api::dialog::message(
//     win.as_ref(),
//     "Save File",
//     format!("PATH: {}", path.display()),
//   );
// }

/**
 * Used for saving Markdown files
 */
#[command]
pub fn save_file(app: AppHandle, name: String, content: String) -> Result<(), String> {
  let win = app.app_handle().get_window("core");
  let path = utils::app_root().join(PathBuf::from(name));

  // If file exists, ask user if they want to overwrite
  // if Path::new(&path).exists() {
  //   // TODO
  // }

  utils::create_file(&path).unwrap();
  fs::write(&path, content).unwrap();
  // utils::open_file(path);
  tauri::api::dialog::message(
    win.as_ref(),
    "Save File",
    format!("PATH: {}", path.display()),
  );
  Ok(())
}

#[command]
pub fn open_link(app: AppHandle, url: String) {
  api::shell::open(&app.shell_scope(), url, None).unwrap();
}

#[command]
pub fn run_check_update(app: AppHandle, silent: bool, has_msg: Option<bool>) {
  utils::run_check_update(app, silent, has_msg);
}

#[command]
pub fn open_file(path: PathBuf) {
  utils::open_file(path);
}

/**
 * For Images and PDFs
 */
#[command]
pub fn download_file(name: String, blob: Vec<u8>) {
  let file = tauri::api::path::download_dir().unwrap().join(name);
  fs::write(&file, blob).unwrap();
  utils::open_file(file);
}

#[command]
pub async fn get_data(app: AppHandle, url: String, is_msg: Option<bool>) -> Option<String> {
  let is_msg = is_msg.unwrap_or(false);
  let res = if is_msg {
    utils::get_data(&url, Some(&app)).await
  } else {
    utils::get_data(&url, None).await
  };
  res.unwrap_or_else(|err| {
    error!("chatgpt_client_http: {}", err);
    None
  })
}

#[tauri::command]
pub async fn fetch_image(url: String) -> Vec<u8> {
  let response = reqwest::get(url).await.unwrap();
  let bytes = response.bytes().await.unwrap();
  bytes.to_vec()
}
