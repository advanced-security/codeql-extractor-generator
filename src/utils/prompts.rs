use dialoguer::Input;

pub fn input_prompt(prompt: &str) -> String {
    Input::new()
        .with_prompt(prompt)
        .interact_text()
        .expect("Failed to read input")
}

pub fn select(prompt: &str, options: &[&str]) -> Result<Vec<String>, dialoguer::Error> {
    dialoguer::MultiSelect::new()
        .with_prompt(prompt)
        .items(options)
        .interact()
        .map(|selected_indices| {
            selected_indices
                .into_iter()
                .map(|i| options[i].to_string())
                .collect()
        })
        .map_err(|e| e.into())
}

pub fn input_optional_prompt(prompt: &str) -> Option<String> {
    let input: Result<String, dialoguer::Error> = Input::new()
        .allow_empty(true)
        .with_prompt(prompt)
        .interact_text();

    match input {
        Ok(s) => {
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
        Err(_) => None,
    }
}
