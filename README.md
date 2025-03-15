#### 两个简单的线程池

---

学习用尝试。

示例。

```rust
    fn test_cancel_while_task_in_queue() {
        let pool = ThreadPool::new()
            .num_threads(1)
            .build()
            .expect("Failed to create thread pool");

        let (sender, receiver) = std::sync::mpsc::channel();

        let task1 = move || {
            std::thread::sleep(std::time::Duration::from_secs(1)); // 模拟长时间运行的任务
            sender.send("Task 1 completed").unwrap();
            1
        };

        let task2 = || 2;

        let handle1 = pool.commit(task1);
        let handle2 = pool.commit(task2);

        // 取消第二个任务
        handle2.cancel();

        // 等待第一个任务完成
        assert_eq!(receiver.recv(), Ok("Task 1 completed"));

        // 检查第二个任务的状态
        assert_eq!(handle2.state(), TaskState::Cancelled); // 任务应被取消

        // 尝试等待第二个任务，应返回错误
        let result = handle2.wait();
        assert!(result.is_err());
    }
```