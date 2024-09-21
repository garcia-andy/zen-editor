use crate::editor::Editor;
use crate::fileinfo::FileInfo;
use std::path::PathBuf;

pub async fn scan_file(mut this: Editor, path: Option<PathBuf>) -> bool {
    let path = if let Some(p) = path {
        p
    } else {
        this.files[this.active_file].path.clone()
    };
    let mut idx = 0;
    for (i, f) in this.files.iter().enumerate() {
        if f.path.to_str().unwrap() == path.to_str().unwrap() {
            this.files[i].content =
                std::fs::read_to_string(f.path.clone())
                    .expect("Unable to read file");
            idx = i;
            break;
        }
    }
    idx == this.active_file
}

pub async fn open_file(mut this: Editor) -> Option<(PathBuf, String)> {
    let file = rfd::FileDialog::new().pick_file();
    if let Some(file) = file {
        load_file(&mut this, Some(file)).await;
        this.active_file = this.files.len() - 1;
        let info = this.files[this.active_file].clone();
        Some((info.path, info.content))
    } else {
        None
    }
}

pub async fn load_file(this: &mut Editor, path: Option<PathBuf>) {
    let file_path = if let Some(p) = path {
        p
    } else {
        rfd::FileDialog::new().pick_file().unwrap()
    };

    let content = std::fs::read_to_string(&file_path).unwrap();
    this.files.push(FileInfo::new(file_path, content));
}

pub async fn save_file(this: &mut Editor, path: Option<PathBuf>) {
    let file_path = if let Some(p) = path {
        p
    } else {
        rfd::FileDialog::new().save_file().unwrap()
    };

    let mut idx = this.files.len() + 1;
    for (i, f) in this.files.iter().enumerate() {
        if f.path.to_str().unwrap() == file_path.to_str().unwrap() {
            idx = i;
            break;
        }
    }

    if idx < this.files.len() {
        let content = this.files[idx].content.clone();
        std::fs::write(file_path.clone(), content)
            .expect("Unable to write file changes");
    } else {
        println!("File not found {:?} on {:?}", file_path, this.files);
    }
}

pub async fn quit_file(mut this: Editor, idx: usize) -> Option<PathBuf> {
    if this.files.len() >= 1 {
        let ret = this.files.remove(idx);

        if idx == this.active_file {
            if idx > (this.files.len() / 2) {
                this.active_file = this.files.len() - 1;
            } else {
                this.active_file = 0;
            }
        }

        Some(ret.path)
    } else {
        None
    }
}

pub async fn save_content(mut this: Editor) {
    if this.files.len() >= 1 {
        let buf = this.files[this.active_file].clone();
        save_file(&mut this, Some(buf.path)).await;
    }
}
