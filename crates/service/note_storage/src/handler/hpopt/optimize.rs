use crate::common::error::IdpGlobalError;

pub async fn get_optimize_objective_example_names() -> Result<Vec<String>, std::io::Error> {
    // get datasource dir path
    let optimize_objective_example_dir_path =
        business::path_tool::get_optimize_objective_example_path();
    // create file struct by path and get all file name.
    let mut example_name_list = Vec::new();
    if let Ok(dir) = std::fs::read_dir(optimize_objective_example_dir_path.clone()) {
        dir.for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            example_name_list.push(file_name.to_string());
                        }
                    }
                }
            }
        });
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no exists example file",
        ));
    }
    Ok(example_name_list)
}

pub async fn get_optimize_objective_code_content(
    objective_example_name: String,
) -> Result<String, IdpGlobalError> {
    let file_path = format!(
        "{}/{}",
        business::path_tool::get_optimize_objective_example_path(),
        objective_example_name
    );
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}
