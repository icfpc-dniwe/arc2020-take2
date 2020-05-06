use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use std::collections::HashMap;
use serde::Deserialize;
use ndarray::Array2;
use failure::err_msg;

use super::types::*;

pub type ImageJson = Vec<Vec<u8>>;

#[derive(Deserialize, Debug)]
pub struct TaskTrainJson {
    pub input: ImageJson,
    pub output: ImageJson,
}

#[derive(Deserialize, Debug)]
pub struct TaskTestJson {
    pub input: ImageJson,
    pub output: Option<ImageJson>,
}

#[derive(Deserialize, Debug)]
pub struct TaskJson {
    pub train: Vec<TaskTrainJson>,
    pub test: Vec<TaskTestJson>,
}

pub fn deserialize_image(img: &Vec<Vec<u8>>) -> MyResult<Image> {
    if img.len() == 0 {
        return Err(format_err!("Empty image"));
    }
    let cols_len = img[0].len();
    for row in img {
        if row.len() != cols_len {
            return Err(format_err!("Inconsistent image"));
        }
    }
    Ok(Array2::from_shape_fn((img.len(), cols_len), |(y, x)| img[y][x]))
}

pub fn deserialize_task_train(task: &TaskTrainJson) -> MyResult<TaskTrain> {
    Ok(TaskTrain {
        input: deserialize_image(&task.input)?,
        output: deserialize_image(&task.output)?,
    })
}

pub fn deserialize_task_test(task: &TaskTestJson) -> MyResult<TaskTest> {
    Ok(TaskTest {
        input: deserialize_image(&task.input)?,
        output: task.output.as_ref().map(|x| deserialize_image(&x)).transpose()?.into_iter().collect(),
    })
}

pub fn deserialize_task(task: &TaskJson) -> MyResult<Task> {
    Ok(Task {
        train: task.train.iter().map(|x| deserialize_task_train(&x)).collect::<MyResult<Vec<TaskTrain>>>()?,
        test: task.test.iter().map(|x| deserialize_task_test(&x)).collect::<MyResult<Vec<TaskTest>>>()?,
    })
}

pub fn serialize_image<S: Write>(stream: &mut S, img: &Image) -> MyResult<()> {
    if img.len() == 0 {
        stream.write_all(b"||")?;
    } else {
        stream.write_all(b"|")?;
        for row in img.outer_iter() {
            for x in row {
                write!(stream, "{}", x)?;
            }
            stream.write_all(b"|")?;
        }
    }
    Ok(())
}

pub fn serialize_images<'a, S: Write, T: Iterator<Item = &'a Image>>(stream: &'a mut S, mut imgs: T) -> MyResult<()> {
    if let Some(img) = imgs.next() {
        serialize_image(stream, img)?;
        for img in imgs {
            stream.write_all(b" ")?;
            serialize_image(stream, img)?;
        }
    }
    Ok(())
}

pub fn read_all_tasks(dir_path: &Path) -> MyResult<HashMap<String, Task>> {
    let mut tasks = HashMap::new();
    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();
        if path.is_file() {
            let name = path.file_stem().ok_or_else(|| err_msg("No file stem"))?.to_str().ok_or_else(|| err_msg("No string"))?.to_owned();
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let task_json = serde_json::from_reader(reader)?;
            let task = deserialize_task(&task_json)?;
            tasks.insert(name, task);
        }
    }
    Ok(tasks)
}

pub fn serialize_all_tasks<S: Write>(stream: &mut S, tasks: &HashMap<String, TaskSolution>) -> MyResult<()> {
    stream.write_all(b"output_id,output\n")?;
    for (name, task) in tasks {
        for (n, test) in task.test.iter().enumerate() {
            if test.output.len() > 0 {
                write!(stream, "{}_{},", name, n)?;
                serialize_images(stream, test.output.iter())?;
                stream.write_all(b"\n")?;
            }
        }
    }
    Ok(())
}
