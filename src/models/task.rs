use std::{fs, path::Path};

pub struct Task {
    pub title: String,
    pub pomodoros_expected: u16,
    pub pomodoros_completed: u16,
    pub completed: bool,
}

impl Task {
    pub fn from_file(path: &Path) -> Vec<Self> {
        let raw_tasks = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return vec![],
        };

        let sections = raw_tasks
            .split("---")
            .map(|section| section.trim())
            .filter(|section| !section.is_empty())
            .collect::<Vec<&str>>();

        let tasks: Vec<Task> = sections
            .iter()
            .map(|section| {
                let section_items = section
                    .lines()
                    .flat_map(|line| line.split(":").skip(1).map(|val| val.trim()))
                    .collect::<Vec<&str>>();

                // Todo: is there a better way of doing this?
                Task {
                    title: section_items[0].to_string(),
                    pomodoros_expected: section_items[1].parse().unwrap_or_default(),
                    pomodoros_completed: section_items[2].parse().unwrap_or_default(),
                    completed: section_items[3].parse().unwrap_or_default(),
                }
            })
            .collect();

        tasks
    }

    pub fn save(path: &Path, tasks: &Vec<Self>) -> Result<(), std::io::Error> {
        let mut content_string = String::new();

        for task in tasks.iter() {
            content_string.push_str(
                format!(
                "---\ntitle: {}\npomodoros_expected: {}\npomodoros_completed: {}\ncompleted: {}\n",
                task.title, task.pomodoros_expected, task.pomodoros_completed, task.completed
            )
                .as_str(),
            );
        }

        content_string.push_str("---");

        fs::write(path, content_string)
    }
}

impl Task {
    pub fn list_print(&self) -> String {
        format!(
            "[{}] | {}/{} - {}",
            if self.completed { "x" } else { " " },
            self.pomodoros_completed,
            self.pomodoros_expected,
            self.title
        )
    }

    pub fn complete_task(&mut self) {
        self.completed = !self.completed;
    }

    pub fn complete_pomodoro(&mut self) {
        self.pomodoros_completed += 1;
    }

    pub fn negate_pomodoro(&mut self) {
        if self.pomodoros_completed > 0 {
            self.pomodoros_completed -= 1;
        }
    }
}
