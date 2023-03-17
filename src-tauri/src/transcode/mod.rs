// 导入所需的库
extern crate rusqlite;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

// 定义任务结构体
struct Task {
    id: u32,
    command: String,
    status: String,
    handle: Option<JoinHandle<()>>,
}

// 定义 ffmepg 任务管理器结构体
pub struct FfmpegManager {
    db: Arc<Mutex<rusqlite::Connection>>, // 使用Arc 和Mutex 来保证线程安全
    tasks: Vec<Task>,
}

impl FfmpegManager {
    // 对外提供接口的方法，用于添加新任务到管理器中
    pub fn add_task(&mut self, command: &str) -> Result<(), String> {
        // 创建一个新的任务对象
        let task = Task {
            id: self.tasks.len() as u32 + 1, // 任务id从 1 开始
            command: command.to_string(),
            status: "created".to_string(), // 初始化任务状态为created
            handle: None,                  // 初始化任务句柄为空
        };
        // 将新任务添加到任务列表中
        self.tasks.push(task);
        // 记录任务信息到 sqlite 中
        let conn = self.db.lock().unwrap();
        let result = conn.execute(
            "INSERT INTO tasks (command, status) VALUES (?1, ?2)",
            &[&command, &"created"],
        );
        // 如果插入失败，则返回错误信息
        if result.is_err() {
            return Err("Failed to insert task into database".to_string());
        }
        Ok(())
    }

    // 对外提供接口的方法，用于获取所有任务信息
    pub fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    // 对外提供接口的方法，用于停止指定任务
    pub fn stop_task(&mut self, task_id: u32) -> Result<(), String> {
        // 查找指定id的任务
        if let Some(index) = self.tasks.iter().position(|t| t.id == task_id) {
            // 获取任务的句柄
            if let Some(handle) = self.tasks[index].handle.take() {
                // 杀死 ffmpeg 进程
                handle.abort();
                // 更新任务状态为 stopped
                self.tasks[index].status = "stopped".to_string();
                // 修改 sqlite 中对应任务的状态
                let conn = self.db.lock().unwrap();
                let result = conn.execute(
                    "UPDATE tasks SET status=?1 WHERE id=?2",
                    &[&"stopped", &task_id],
                );
                // 如果修改失败，则返回错误信息
                if result.is_err() {
                    return Err("Failed to update task status in database".to_string());
                }
                Ok(())
            } else {
                Err("Task is not running".to_string())
            }
        } else {
            Err("Task not found".to_string())
        }
    }

    // 内部方法，用于启动任务
    fn start_task(&mut self, index: usize) {
        let command = self.tasks[index].command.clone();
        let mut child = Command::new("sh")
            .args(&["-c", &command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute command");
        // 将任务的句柄保存到任务列表中
        self.tasks[index].handle = Some(tokio::spawn(async move {
            let _ = child.wait().await.unwrap();
            // 任务结束后更新任务状态为 completed 或者 failed
            let status = if child.status().unwrap().success() {
                "completed"
            } else {
                "failed"
            };
            // 更新任务状态并将句柄置为空
            self.tasks[index].status = status.to_string();
            self.tasks[index].handle = None;
            // 修改 sqlite 中对应任务的状态
            let conn = self.db.lock().unwrap();
            let result = conn.execute(
                "UPDATE tasks SET status=?1 WHERE id=?2",
                &[&status, &self.tasks[index].id],
            );
            // 如果修改失败，则返回错误信息
            if result.is_err() {
                println!("Failed to update task status in database");
            }
        }));
    }

    // 内部方法，用于检查任务列表中是否有已完成的任务，如果有则启动新的任务
    fn check_tasks(&mut self) {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.status == "created" {
                self.start_task(index);
            }
        }
    }

    // 内部方法，用于启动 ffmpeg 任务管理器的主循环
    async fn run_loop(mut self) -> Result<(), String> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            // 检查任务列表中是否有已完成的任务，如果有则启动新的任务
            self.check_tasks();
        }
    }

    // 对外提供接口的方法，用于启动 ffmpeg 任务管理器
    pub fn start() -> Result<(), String> {
        // 初始化 sqlite 数据库
        let conn = rusqlite::Connection::open("tasks.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            command TEXT,
            status TEXT
        )",
            [],
        )
        .unwrap();
        let db = Arc::new(Mutex::new(conn));
        // 创建 ffmepg 任务管理器对象
        let manager = FfmpegManager {
            db: db.clone(),
            tasks: Vec::new(),
        };
        // 启动 ffmepg 任务管理器的主循环
        tokio::spawn(async move {
            let _ = manager.run_loop().await;
        });
        Ok(())
    }
}
