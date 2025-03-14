#![allow(unused)]

use std::{
    sync::{atomic::{AtomicUsize, Ordering}, Arc, Barrier},
    thread, 
    time::Duration
};

use somewhat_complex_one::*;

#[cfg(test)]
mod complex_tests {
    use std::sync::atomic;

    use super::*;

    #[test]
    fn test_thread_pool_basic() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .scheduler(FifoScheduler::new())
        .build()
        .unwrap();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        pool.commit(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        thread::sleep(Duration::from_millis(100)); // 等待任务执行完成
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_thread_pool_concurrent_tasks() {
        let pool = ThreadPool::new()
        .num_threads(10)
        .scheduler(FifoScheduler::new())
        .build()
        .unwrap();
        let counter = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(10));

        for _ in 0..10 {
            let counter_clone = counter.clone();
            let barrier_clone = barrier.clone();
            pool.commit(move || {
                barrier_clone.wait(); // 确保所有任务同时开始
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_thread_pool_large_number_of_tasks() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .scheduler(FifoScheduler::new())
        .build()
        .unwrap();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..1000 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(500)); // 等待所有任务执行完成
        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }

    #[test]
    fn test_thread_pool_terminate() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .build()
        .unwrap();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(500)); // 等待所有任务执行完成
        pool.terminate(); // 终止线程池
        assert_eq!(counter.load(Ordering::SeqCst), 10); // 确保所有任务执行完成
    }

    #[test]
    fn test_thread_pool_drop() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .build()
        .unwrap();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(500)); // 等待所有任务执行完成
        drop(pool); // 显式 drop 线程池
        assert_eq!(counter.load(Ordering::SeqCst), 10); // 确保所有任务执行完成
    }

    #[test]
    fn test_high_concurrency() {
        let pool = ThreadPool::new()
        .num_threads(8)
        .build()
        .unwrap(); // 使用 8 个线程
        let counter = Arc::new(atomic::AtomicUsize::new(0));

        // 提交 1000 个任务
        for _ in 0..1000 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_micros(500));
        drop(pool); // 确保所有任务被执行
        assert_eq!(counter.load(atomic::Ordering::SeqCst), 1000);
    }

    #[test]
    fn test_long_running_tasks() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .build()
        .unwrap();
        let counter = Arc::new(atomic::AtomicUsize::new(0));

        // 提交 4 个长时间任务
        for _ in 0..4 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                thread::sleep(Duration::from_millis(500)); // 模拟长时间任务
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(100));
        drop(pool);
        assert_eq!(counter.load(atomic::Ordering::SeqCst), 4);
    }

    #[test]
    fn test_task_submission_and_state() {
        let pool = ThreadPool::new()
            .num_threads(2)
            .build()
            .expect("Failed to create thread pool");
    
        let task = (|| 42).to_task().unwrap();
        let handle = pool.commit(task);
    
        // 等待任务完成并检查结果
        let result = handle.wait().unwrap();
        assert_eq!(result, 42);
        assert_eq!(handle.state(), TaskState::Completed);
    }

    #[test]
    fn test_dynamic_task_count() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .build()
        .unwrap();
        let counter = Arc::new(atomic::AtomicUsize::new(0));

        // 提交 100 个任务
        for _ in 0..100 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        // 再提交 100 个任务
        for _ in 0..100 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_micros(500));
        drop(pool);
        assert_eq!(counter.load(atomic::Ordering::SeqCst), 200);
    }

    #[test]
    fn test_mixed_tasks() {
        let pool = ThreadPool::new()
        .num_threads(4)
        .build()
        .unwrap();
        let counter = Arc::new(atomic::AtomicUsize::new(0));

        // 提交短任务
        for _ in 0..50 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        // 提交长任务
        for _ in 0..4 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                thread::sleep(Duration::from_millis(500)); // 模拟长时间任务
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        // 提交更多短任务
        for _ in 0..50 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(1000));
        drop(pool);
        assert_eq!(counter.load(atomic::Ordering::SeqCst), 104);
    }

    #[test]
    fn test_performance() {
        let pool = ThreadPool::new()
        .num_threads(8)
        .build()
        .unwrap();
        let counter = Arc::new(atomic::AtomicUsize::new(0));

        let start = std::time::Instant::now();

        // 提交 10000 个任务
        for _ in 0..10000 {
            let counter_clone = counter.clone();
            pool.commit(move || {
                counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_micros(500));
        drop(pool);
        let duration = start.elapsed();

        assert_eq!(counter.load(atomic::Ordering::SeqCst), 10000);
        println!("Time elapsed: {:?}", duration);
    }

    #[test]
    fn test_task_cancellation() {
        let pool = ThreadPool::new()
            .num_threads(1)
            .build()
            .expect("Failed to create thread pool");

        let task = || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            42
        };
        pool.commit(task);
        let handle = pool.commit(|| 42);
        // 立即取消任务
        handle.cancel();

        // 检查任务状态是否为Cancelled
        assert_eq!(handle.state(), TaskState::Cancelled);

        // 尝试等待任务，应该返回错误
        let result = handle.wait();
        assert!(result.is_err());
    }

    #[test]
    fn test_concurrent_task_execution() {
        let pool = ThreadPool::new()
            .num_threads(4)
            .build()
            .expect("Failed to create thread pool");
    
        let task1 = || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            1
        };
        let task2 = || 2;
        let task3 = || 3;
        let task4 = || 4;
    
        let handle1 = pool.commit(task1);
        let handle2 = pool.commit(task2);
        let handle3 = pool.commit(task3);
        let handle4 = pool.commit(task4);
    
        let result1 = handle1.wait();
        let result2 = handle2.wait();
        let result3 = handle3.wait();
        let result4 = handle4.wait();
    
        assert_eq!(result1, Ok(1));
        assert_eq!(result2, Ok(2));
        assert_eq!(result3, Ok(3));
        assert_eq!(result4, Ok(4));
    }

    #[test]
    fn test_cancel_after_task_starts() {
        let pool = ThreadPool::new()
            .num_threads(1)
            .build()
            .expect("Failed to create thread pool");

        let (sender, receiver) = std::sync::mpsc::channel();

        let task = move || {
            sender.send("Task started").unwrap(); // 通知任务已开始
            std::thread::sleep(std::time::Duration::from_secs(1)); // 模拟长时间运行的任务
            42
        };

        let handle = pool.commit(task);

        // 等待任务开始
        assert_eq!(receiver.recv(), Ok("Task started"));

        // 尝试取消任务
        handle.cancel();

        // 检查任务状态
        assert_eq!(handle.state(), TaskState::Running); // 任务已经开始，无法取消

        // 等待任务完成
        let result = handle.wait();
        assert_eq!(result, Ok(42)); // 任务应正常完成
        assert_eq!(handle.state(), TaskState::Completed); // 任务状态应为已完成
    }

    #[test]
    fn test_wait_after_task_completes() {
        let pool = ThreadPool::new()
            .num_threads(1)
            .build()
            .expect("Failed to create thread pool");

        let task = || 42;

        let handle = pool.commit(task);

        // 等待任务完成
        let result = handle.wait();
        assert_eq!(result, Ok(42)); // 任务应正常完成

        // 再次调用 wait，应返回相同的结果
        let result = handle.wait();
        assert_eq!(result, Err(Error::MultipleWaits)); // 结果应保持不变
        assert_eq!(handle.state(), TaskState::Completed); // 任务状态应为已完成
    }

    #[test]
    fn test_concurrent_cancel_and_wait() {
        let pool = ThreadPool::new()
            .num_threads(1)
            .build()
            .expect("Failed to create thread pool");

        pool.commit(|| thread::sleep(Duration::from_secs(1)));
        let task = || {
            std::thread::sleep(std::time::Duration::from_secs(1)); // 模拟长时间运行的任务
            42
        };

        let handle = Arc::new(pool.commit(task));

        let handle_clone = Arc::clone(&handle);
        let cancel_thread = std::thread::spawn(move || {
            handle_clone.cancel();
        });

        let handle_clone = Arc::clone(&handle);
        let wait_thread = std::thread::spawn(move || {
            handle_clone.wait()
        });

        // 等待线程完成
        let cancel_result = cancel_thread.join().unwrap();
        let wait_result = wait_thread.join().unwrap();

        // 检查任务状态
        assert_eq!(handle.state(), TaskState::Cancelled); // 任务应被取消

        // 检查 wait 的结果
        assert!(wait_result.is_err()); // 任务被取消，应返回错误
    }

    #[test]
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

    #[test]
    fn test_wait_after_task_fails() {
        let pool = ThreadPool::new()
            .num_threads(1)
            .build()
            .expect("Failed to create thread pool");

        let task = || -> Result<i32, String> {
            Err("Task failed".to_string())
        };

        let handle = pool.commit(task);

        // 等待任务完成
        let result = handle.wait().unwrap();
        assert_eq!(result, Err("Task failed".to_string())); // 任务应返回错误
        assert_eq!(handle.state(), TaskState::Completed); // 任务状态应为已完成
    }
}