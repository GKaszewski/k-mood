use tower_http::services::{ServeDir, ServeFile};

pub fn serve_spa(spa_dir: &str) -> ServeDir<ServeFile> {
    ServeDir::new(spa_dir).fallback(ServeFile::new(format!("{spa_dir}/index.html")))
}
