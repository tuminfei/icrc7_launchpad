const IDENTITY: &str = "deploy_bts";
const NETWORK: &str = "local";
// const NETWORK: &str = "ic";
const ASSETS_DIR: &str = "../assets";
// const ASSETS_DIR: &str = "empty"; // 删除所有数据
// const ASSETS_DIR: &str = "assets-test"; // 测试数据
const IGNORE_FILES: [&str; 4] = [".DS_Store", ".gitkeep", ".gitignore", ".git"];
const CHUNK_SIZE: u64 = 1024 * 1024 * 2 - 1024 * 128;

#[derive(Debug, Clone)]
struct LocalFile {
    pub path: String,
    pub size: u64,
    pub headers: Vec<(String, String)>,
    pub modified: u64,
    pub hash: String,
    pub data: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct RemoteFile {
    pub path: String,
    pub size: u64,
    pub headers: Vec<(String, String)>,
    pub created: u64,
    pub modified: u64,
    pub hash: String,
}

#[derive(Debug)]
struct UploadFile {
    pub file: LocalFile,
    pub chunks: u64,
    pub chunk_size: u64,
    pub index: u64,
    pub offset: usize,
    pub offset_end: usize,
}

// Content-Type
const EXT_CONTENT_TYPES: [(&str, &str); 50] = [
    ("txt", "text/plain"), // 文本
    ("html", "text/html"),
    ("htm", "text/html"),
    ("htx", "text/html"),
    ("xhtml", "text/html"),
    ("css", "text/css"),
    ("js", "text/javascript"),
    ("md", "text/markdown"),
    ("ics", "text/calendar"),
    ("csv", "text/csv"),
    ("xml", "text/xml"),
    ("json", "application/json"),
    ("pdf", "application/pdf"),
    ("zip", "application/zip"),
    ("prefab", "application/zip"),
    ("7z", "application/x-7z-compressed"),
    ("eot", "application/vnd.ms-fontobject"),
    ("png", "image/png"),
    ("gif", "image/gif"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("svg", "image/svg+xml"),
    ("webp", "image/webp"),
    ("tif", "image/tiff"),
    ("tiff", "image/tiff"),
    ("ico", "image/x-icon"),
    ("mp4", "video/mp4"),
    ("avi", "video/x-msvideo"),
    ("mov", "video/quicktime"),
    ("mpeg", "video/mpeg"),
    ("ogv", "video/ogg"),
    ("webm", "video/webm"),
    ("mp3", "audio/mp3"),
    ("wav", "audio/wav"),
    ("flac", "audio/flac"),
    ("aac", "audio/aac"),
    ("webm", "audio/webm"),
    ("oga", "audio/ogg"),
    ("wma", "audio/x-ms-wma"),
    ("mid", "audio/midi"),
    ("midi", "audio/midi"),
    ("ra", "audio/x-realaudio"),
    ("ram", "audio/x-realaudio"),
    ("otf", "font/otf"),
    ("ttf", "font/ttf"),
    ("woff", "font/woff"),
    ("woff2", "font/woff2"),
    ("dat", ""),
    ("plot", ""),
    ("cache", ""),
];

#[test]
fn upload() {
    let mut local_files: Vec<LocalFile> = vec![];
    load_local_files(ASSETS_DIR, ASSETS_DIR, &mut local_files);
    let local_file_names: Vec<String> = local_files.iter().map(|f| f.path.clone()).collect();
    println!("local files: {:?}", local_file_names);

    let remote_files = load_remote_files();
    // println!("remote files: {:?}", remote_files);

    let deletes: Vec<String> = remote_files
        .iter()
        .map(|f| f.path.clone())
        .filter(|p| !local_file_names.contains(p))
        .collect();
    if !deletes.is_empty() {
        delete_files(deletes);
    }

    let local_files: Vec<LocalFile> = local_files
        .into_iter()
        .filter(|local_file| {
            let remote_file = remote_files.iter().find(|f| f.path == local_file.path);
            if remote_file.is_none() {
                return true;
            }
            let remote_file = remote_file.unwrap();
            let mut file_headers: Vec<String> = local_file
                .headers
                .iter()
                .map(|h| format!("{}:{}", h.0, h.1))
                .collect();
            file_headers.sort();
            let mut remote_file_headers: Vec<String> = remote_file
                .headers
                .iter()
                .map(|h| format!("{}:{}", h.0, h.1))
                .collect();
            remote_file_headers.sort();
            let changed = local_file.size != remote_file.size
                || file_headers.join(";") != remote_file_headers.join(";")
                || local_file.hash != remote_file.hash
                || remote_file.modified < local_file.modified * 1000000;
            if !changed {
                println!("file: {} has not changed. do nothing.", local_file.path)
            }
            changed
        })
        .collect();
    if local_files.is_empty() {
        println!("Nothing to do");
        return;
    }
    upload_files(local_files);
}

fn load_local_files(prefix: &str, dir_path: &str, files: &mut Vec<LocalFile>) {
    let entries = std::fs::read_dir(dir_path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_type = entry.file_type().unwrap();

        let path = format!("{}/{}", dir_path, file_name.to_str().unwrap().to_string());
        fn is_ignore(path: &str) -> bool {
            for ignore in IGNORE_FILES {
                if path.ends_with(ignore) {
                    return true;
                }
            }
            false
        }

        if is_ignore(&path) {
            continue;
        }

        if file_type.is_file() {
            let mut file = load_local_file(&path);
            file.path = (&file.path[prefix.len()..]).to_string();
            files.push(file);
        } else if file_type.is_dir() {
            load_local_files(prefix, &path, files);
        }
    }
}

fn load_local_file(path: &str) -> LocalFile {
    let metadata = std::fs::metadata(path).unwrap();
    let file_size = metadata.len();

    use std::time::UNIX_EPOCH;
    let modified_time = metadata
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    use std::io::Read;
    file.read_to_end(&mut buffer).unwrap();

    LocalFile {
        path: path.to_string(),
        size: file_size,
        headers: get_headers(&path),
        modified: modified_time as u64,
        hash: do_hash(&buffer),
        data: buffer,
    }
}

fn do_hash(data: &Vec<u8>) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&data[..]);
    let digest: [u8; 32] = hasher.finalize().into();
    hex::encode(&digest)
}

fn get_headers(file: &str) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = vec![];

    let mut content_type: String = String::from("");

    use std::path::Path;
    let file_path = Path::new(file);
    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let file_name = file.to_string();
            let ext = ext_str.to_lowercase();

            fn get_content_type(ext_str: &str) -> String {
                for (ext, content) in EXT_CONTENT_TYPES {
                    if ext == ext_str {
                        return content.to_string();
                    }
                }
                panic!("Unknown file type: {}", ext_str);
            }

            if &ext == "gz" {
                let mut ext = "";
                let mut s = (&file_name[0..(file_name.len() - 3)]).split(".");
                while let Some(e) = s.next() {
                    ext = e;
                }
                content_type = get_content_type(ext);
            } else {
                content_type = get_content_type(&ext);
            }
        } else {
            println!("Invalid extension");
        }
    } else {
        println!("No extension: {}", file);
    }

    if !content_type.is_empty() {
        headers.push(("Content-Type".to_string(), content_type.to_string()));
    }

    headers.push((
        "Cache-Control".to_string(),
        "public, max-age=31536000".to_string(),
    ));

    if file.ends_with(".gz") {
        headers.push(("Content-Encoding".to_string(), "gzip".to_string()));
    }

    headers
}

fn load_remote_files() -> Vec<RemoteFile> {
    use std::process::Command;

    let _start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    let output = Command::new("/Users/terry/Library/Application Support/org.dfinity.dfx/bin/dfx")
        .current_dir(".")
        .arg("--identity")
        .arg(IDENTITY)
        .arg("canister")
        .arg("--network")
        .arg(NETWORK)
        .arg("call")
        .arg("ic_canister_assets")
        .arg("files")
        .arg("()")
        .arg("--output")
        .arg("idl")
        .output()
        .expect("error");

    let _end = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    // println!("api: {} -> {:?}", "files", _end - _start);
    // println!("status: {}", output.status);

    if format!("{}", output.status).eq("exit status: 0") {
        let output = String::from_utf8(output.stdout.clone()).unwrap();
        // println!("output: {}", output);
        return parse_remote_files(output);
    }

    eprintln!(">>>>>>>>>> ERROR <<<<<<<<<<<");
    eprintln!("identity: {}", IDENTITY);
    eprintln!("api: {}", "files");
    eprintln!("arg: {}", "");
    eprintln!("status: {}", output.status);
    if format!("{}", output.status).eq("exit status: 0") {
        eprintln!(
            "output: {}",
            String::from_utf8(output.stdout).unwrap().trim_end()
        );
    } else {
        eprintln!(
            "error : {}",
            String::from_utf8(output.stderr).unwrap().trim_end()
        );
    }
    panic!("error");
}

fn parse_remote_files(output: String) -> Vec<RemoteFile> {
    let output = output.trim();
    let output = (&output[6..(output.len() - 2)]).to_string();
    let output = output.trim();

    if output.len() == 0 {
        return vec![];
    }

    let output = (&output[9..(output.len() - 4)]).to_string();
    let output = output.trim();

    let mut files = vec![];
    let mut splitted = output.split("};}; record { ");
    while let Some(content) = splitted.next() {
        let content = (&content[10..]).to_string();
        let created: u64 = content
            .split(r#" : nat64; modified = "#)
            .next()
            .unwrap()
            .to_string()
            .replace("_", "")
            .parse()
            .unwrap();
        let mut content = content.split(r#" : nat64; modified = "#);
        content.next();
        let content = content.next().unwrap();

        let modified: u64 = content
            .split(r#" : nat64; hash = ""#)
            .next()
            .unwrap()
            .to_string()
            .replace("_", "")
            .parse()
            .unwrap();
        let mut content = content.split(r#" : nat64; hash = ""#);
        content.next();
        let content = content.next().unwrap();

        let hash = (&content[0..64]).to_string();
        let mut content = content.split(r#""; path = ""#);
        content.next();
        let content = content.next().unwrap();

        let path = content.split(r#""; size = "#).next().unwrap().to_string();
        let mut content = content.split(r#""; size = "#);
        content.next();
        let content = content.next().unwrap();

        let size: u64 = content
            .split(r#" : nat64; headers = "#)
            .next()
            .unwrap()
            .to_string()
            .replace("_", "")
            .parse()
            .unwrap();
        let mut content = content.split(r#" : nat64; headers = "#);
        content.next();
        let content = content.next().unwrap();

        let headers: Vec<(String, String)> = if 5 < content.len() {
            let content = &content[16..(content.len() - 4)];
            let mut headers = vec![];
            let mut cs = content.split(r#"";}; record { ""#);
            while let Some(s) = cs.next() {
                let mut ss = s.split(r#""; ""#);
                let key = ss.next().unwrap().to_string();
                let value = ss.next().unwrap().to_string();
                headers.push((key, value));
            }
            headers
        } else {
            vec![]
        };

        files.push(RemoteFile {
            path,
            size,
            headers,
            created,
            modified,
            hash,
        });
    }
    files
}

fn delete_files(names: Vec<String>) {
    use std::process::Command;

    let _start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    let args = format!(
        "(vec {{{}}})",
        names
            .iter()
            .map(|name| format!("\"{}\"", name))
            .collect::<Vec<String>>()
            .join(";")
    );

    let output = Command::new("/usr/local/bin/dfx")
        .current_dir(".")
        .arg("--identity")
        .arg(IDENTITY)
        .arg("canister")
        .arg("--network")
        .arg(NETWORK)
        .arg("call")
        .arg("ic_canister_assets")
        .arg("delete")
        .arg(&args)
        .arg("--output")
        .arg("idl")
        .output()
        .expect("error");

    let _end = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    if format!("{}", output.status).eq("exit status: 0") {
        for name in names.iter() {
            println!("delete file: {}", name)
        }
        return;
    }

    eprintln!(">>>>>>>>>> ERROR <<<<<<<<<<<");
    eprintln!("identity: {}", IDENTITY);
    eprintln!("api: {}", "delete");
    eprintln!("arg: {}", args);
    eprintln!("status: {}", output.status);
    if format!("{}", output.status).eq("exit status: 0") {
        eprintln!(
            "output: {}",
            String::from_utf8(output.stdout).unwrap().trim_end()
        );
    } else {
        eprintln!(
            "error : {}",
            String::from_utf8(output.stderr).unwrap().trim_end()
        );
    }
    panic!("error");
}

fn upload_files(local_files: Vec<LocalFile>) {
    let mut upload_files: Vec<Vec<UploadFile>> = vec![];

    let mut all_count = 0;
    let mut count = 0;
    let mut upload_file: Vec<UploadFile> = vec![];
    for file in local_files.iter() {
        let size = file.size;
        let mut splitted = size / CHUNK_SIZE;
        if splitted * CHUNK_SIZE < size {
            splitted += 1;
        }
        for i in 0..splitted {
            let (current_size, offset, offset_end) = if i < splitted - 1 {
                (CHUNK_SIZE, CHUNK_SIZE * i, CHUNK_SIZE * (i + 1))
            } else {
                (size - (splitted - 1) * CHUNK_SIZE, CHUNK_SIZE * i, size)
            };
            if CHUNK_SIZE < count + current_size {
                upload_files.push(upload_file);
                count = 0;
                upload_file = vec![]
            }

            count += current_size;
            all_count += current_size;
            upload_file.push(UploadFile {
                file: file.clone(),
                chunks: splitted,
                chunk_size: CHUNK_SIZE,
                index: i,
                offset: offset as usize,
                offset_end: offset_end as usize,
            });
        }
    }
    if !upload_file.is_empty() {
        upload_files.push(upload_file);
    }

    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    use std::thread;
    let mut handles = vec![];
    for (i, upload_file) in upload_files.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            do_upload_file(&upload_file, i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!(
        "all done: total: {:.2}MB time: {}s average: {:.2}MB/s",
        all_count as f64 / 1024f64 / 1024f64,
        (end - start) / 1000,
        all_count as f64 / 1024f64 / 1024f64 / (((end - start) / 1000) as f64)
    );
}

fn do_upload_file(local_files: &Vec<UploadFile>, index: usize) {
    let mut arg = String::from("");
    arg.push_str("(vec{");
    arg.push_str(
        &local_files
            .iter()
            .map(|file| {
                format!(
                    "record{{ path=\"{}\"; headers=vec{{{}}}; size={}:nat64; chunk_size={}:nat64; index={}:nat32; chunk=vec{{{}}} }}",
                    file.file.path,
                    file.file
                        .headers
                        .iter()
                        .map(|header| { format!("record{{\"{}\";\"{}\"}}", header.0, header.1) })
                        .collect::<Vec<String>>()
                        .join(";"),
                    file.file.size,
                    file.chunk_size,
                    file.index,
                    (&file.file.data[file.offset..file.offset_end]).iter().map(|u|format!("{}:nat8", u)).collect::<Vec<String>>().join(";")
                )
            })
            .collect::<Vec<String>>()
            .join(";"),
    );
    arg.push_str("})");
    let arg_file = format!(".args.{}.temp", index);
    write_file(&arg_file, &arg);

    let r = do_upload_file_to_canister(&arg_file, local_files);
    if let Err(msg) = r {
        println!("{}. try again", msg);
        do_upload_file_to_canister(&arg_file, local_files).unwrap();
    }

    std::fs::remove_file(arg_file).unwrap();
}

fn write_file(path: &str, content: &str) {
    use std::io::Write;
    if let Ok(_) = std::fs::File::open(path) {
        std::fs::remove_file(path).unwrap();
    }
    std::fs::File::create(&path)
        .expect("create failed")
        .write_all(content.as_bytes())
        .expect("write candid failed");
}

fn do_upload_file_to_canister(arg: &str, local_files: &Vec<UploadFile>) -> Result<(), String> {
    use std::process::Command;

    let _start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    let output = Command::new("/usr/local/bin/dfx")
        .current_dir(".")
        .arg("--identity")
        .arg(IDENTITY)
        .arg("canister")
        .arg("--network")
        .arg(NETWORK)
        .arg("call")
        .arg("--argument-file")
        .arg(arg)
        .arg("ic_canister_assets")
        .arg("upload")
        .arg("--output")
        .arg("idl")
        .output();
    if let Err(e) = output {
        println!("error: {}", e);
        return Err("Upload failed".to_string());
    }
    let output = output.unwrap();

    let _end = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    if format!("{}", output.status).eq("exit status: 0") {
        for file in local_files.iter() {
            println!(
                "upload file: {} {}/{} ({} bytes) hash: {}",
                file.file.path,
                file.index + 1,
                file.chunks,
                file.offset_end - file.offset,
                file.file.hash
            )
        }
        return Ok({});
    }

    eprintln!(">>>>>>>>>> ERROR <<<<<<<<<<<");
    eprintln!("identity: {}", IDENTITY);
    eprintln!("api: {}", "upload");
    eprintln!("arg: {}", arg);
    eprintln!("status: {}", output.status);
    if format!("{}", output.status).eq("exit status: 0") {
        eprintln!(
            "output: {}",
            String::from_utf8(output.stdout).unwrap().trim_end()
        );
    } else {
        eprintln!(
            "error : {}",
            String::from_utf8(output.stderr).unwrap().trim_end()
        );
    }
    Err("Upload failed".to_string())
}
