use std::path::Path;
// I have no idea about the name, stick with url for now
pub fn get_mime_type(url: &str) -> &str {
    match Path::new(url).extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}
