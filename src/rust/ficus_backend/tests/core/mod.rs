use std::{env, fs};
use std::path::PathBuf;

pub fn execute_test_with_gold<T>(gold_file_path: PathBuf, test_func: T)
    where T : FnOnce() -> String
{
    let gold_file_dir = gold_file_path.parent().unwrap();
    if !gold_file_dir.exists() {
        fs::create_dir_all(gold_file_dir).ok();
    }
    
    let test_value = test_func();

    let write_tmp = || {
        let file_name = gold_file_path.file_name().unwrap().to_str().unwrap();
        let tmp_file_path = gold_file_dir.join(file_name.to_owned() + ".tmp");
        fs::write(&tmp_file_path, &test_value).ok();
    };

    if gold_file_path.exists() {
        let gold_content = String::from_utf8(fs::read(&gold_file_path).ok().unwrap()).ok().unwrap();
        if gold_content != test_value {
            write_tmp();
            panic!("Gold and test values are not equal for {}", gold_file_path.display());
        }

        return;
    }

    write_tmp();
    panic!("There was no gold for {}", gold_file_path.display());
}

pub fn get_test_data_path() -> PathBuf {
    let current_dir = env::current_dir().ok().unwrap();
    let root_dir = current_dir.parent().unwrap().parent().unwrap().parent().unwrap();
    root_dir.join("test_data")
}

pub fn get_test_data_sources_path() -> PathBuf {
    get_test_data_path().join("source")
}

pub fn get_test_data_rust_gold_path() -> PathBuf {
    get_test_data_path().join("gold").join("rust")
}

pub fn get_example_logs_gold_path() -> PathBuf {
    get_test_data_rust_gold_path().join("example_logs")
}

pub fn get_paths_to_example_logs() -> Vec<PathBuf> {
    let example_logs_dir = get_test_data_sources_path().join("example_logs");
    
    let mut logs = Vec::new();
    for path in fs::read_dir(example_logs_dir).unwrap() {
        let candidate_path = path.unwrap().path();
        if candidate_path.extension().unwrap().to_str().unwrap() == "xes" {
            logs.push(candidate_path);
        }
    }
    
    logs
}

pub fn create_example_log_gold_file_path(log_name: &str) -> PathBuf {
    get_example_logs_gold_path().join(log_name.to_owned() + ".gold")
}
