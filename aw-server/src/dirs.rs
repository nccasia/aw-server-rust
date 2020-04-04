use std::path::PathBuf;

#[cfg(not(target_os = "android"))]
use std::fs;

#[cfg(target_os = "android")]
use std::sync::Mutex;

#[cfg(target_os = "android")]
lazy_static! {
    static ref ANDROID_DATA_DIR: Mutex<PathBuf> = Mutex::new(PathBuf::from(
        "/data/user/0/net.activitywatch.android/files"
    ));
}

pub fn get_config_dir() -> Result<PathBuf, ()> {
    #[cfg(not(target_os = "android"))]
    {
        let mut dir = appdirs::user_config_dir(Some("activitywatch"), None, false)?;
        dir.push("aw-server-rust");
        fs::create_dir_all(dir.clone()).expect("Unable to create config dir");
        Ok(dir)
    }

    #[cfg(target_os = "android")]
    {
        panic!("not implemented on Android");
    }
}

pub fn get_data_dir() -> Result<PathBuf, ()> {
    #[cfg(not(target_os = "android"))]
    {
        let mut dir = appdirs::user_data_dir(Some("activitywatch"), None, false)?;
        dir.push("aw-server-rust");
        fs::create_dir_all(dir.clone()).expect("Unable to create data dir");
        Ok(dir)
    }

    #[cfg(target_os = "android")]
    {
        return Ok(ANDROID_DATA_DIR.lock().unwrap().to_path_buf());
    }
}

pub fn get_cache_dir() -> Result<PathBuf, ()> {
    #[cfg(not(target_os = "android"))]
    {
        let mut dir = appdirs::user_cache_dir(Some("activitywatch"), None)?;
        dir.push("aw-server-rust");
        fs::create_dir_all(dir.clone()).expect("Unable to create cache dir");
        Ok(dir)
    }

    #[cfg(target_os = "android")]
    {
        panic!("not implemented on Android");
    }
}

pub fn get_log_dir() -> Result<PathBuf, ()> {
    #[cfg(not(target_os = "android"))]
    {
        let mut dir = appdirs::user_cache_dir(Some("activitywatch"), None)?;
        dir.push("log");
        dir.push("aw-server-rust");
        fs::create_dir_all(dir.clone()).expect("Unable to create cache dir");
        Ok(dir)
    }

    #[cfg(target_os = "android")]
    {
        panic!("not implemented on Android");
    }
}

pub fn db_path(testing: bool) -> PathBuf {
    let mut db_path = get_data_dir().unwrap();
    if testing {
        db_path.push("sqlite-testing.db");
    } else {
        db_path.push("sqlite.db");
    }
    db_path
}

#[cfg(target_os = "android")]
pub fn set_android_data_dir(path: &str) {
    let mut android_data_dir = ANDROID_DATA_DIR.lock().unwrap();
    *android_data_dir = PathBuf::from(path);
}
