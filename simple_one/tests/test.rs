use simple_one::ThreadPool;

use std::sync::atomic::{self, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

#[test]
fn test_thread_pool_basic() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = counter.clone();
    pool.execute(move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    thread::sleep(Duration::from_millis(100)); // 等待任务执行完成
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_thread_pool_concurrent_tasks() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let counter_clone = counter.clone();
        let barrier_clone = barrier.clone();
        pool.execute(move || {
            barrier_clone.wait(); // 确保所有任务同时开始
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }
    pool.terminate();
    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

#[test]
fn test_thread_pool_terminate() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..10 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    pool.terminate(); // 终止线程池
    assert_eq!(counter.load(Ordering::SeqCst), 10); // 确保所有任务执行完成
}

#[test]
fn test_thread_pool_drop() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..10 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    drop(pool); // 显式 drop 线程池
    assert_eq!(counter.load(Ordering::SeqCst), 10); // 确保所有任务执行完成
}

#[test]
fn test_thread_pool_large_number_of_tasks() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..1000 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(500)); // 等待所有任务执行完成
    assert_eq!(counter.load(Ordering::SeqCst), 1000);
}

#[test]
fn test_high_concurrency() {
    let pool = ThreadPool::new(8); // 使用 8 个线程
    let counter = Arc::new(atomic::AtomicUsize::new(0));

    // 提交 1000 个任务
    for _ in 0..1000 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 等待任务完成
    drop(pool); // 确保所有任务被执行
    assert_eq!(counter.load(atomic::Ordering::SeqCst), 1000);
}

#[test]
fn test_long_running_tasks() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(atomic::AtomicUsize::new(0));

    // 提交 4 个长时间任务
    for _ in 0..4 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_millis(500)); // 模拟长时间任务
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 等待任务完成
    drop(pool);
    assert_eq!(counter.load(atomic::Ordering::SeqCst), 4);
}

#[test]
fn test_panic_handling() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(atomic::AtomicUsize::new(0));

    // 提交正常任务
    for _ in 0..4 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 提交会 panic 的任务
    pool.execute(move || {
        panic!("task panicked!");
    });

    // 等待任务完成
    drop(pool);
    assert_eq!(counter.load(atomic::Ordering::SeqCst), 4);
}

#[test]
fn test_dynamic_task_count() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(atomic::AtomicUsize::new(0));

    // 提交 100 个任务
    for _ in 0..100 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 再提交 100 个任务
    for _ in 0..100 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 等待任务完成
    drop(pool);
    assert_eq!(counter.load(atomic::Ordering::SeqCst), 200);
}

#[test]
fn test_mixed_tasks() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(atomic::AtomicUsize::new(0));

    // 提交短任务
    for _ in 0..50 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 提交长任务
    for _ in 0..4 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_millis(500)); // 模拟长时间任务
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 提交更多短任务
    for _ in 0..50 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 等待任务完成
    drop(pool);
    assert_eq!(counter.load(atomic::Ordering::SeqCst), 104);
}

#[test]
fn test_performance() {
    let pool = ThreadPool::new(8);
    let counter = Arc::new(atomic::AtomicUsize::new(0));

    let start = std::time::Instant::now();

    // 提交 10000 个任务
    for _ in 0..10000 {
        let counter_clone = counter.clone();
        pool.execute(move || {
            counter_clone.fetch_add(1, atomic::Ordering::SeqCst);
        });
    }

    // 等待任务完成
    drop(pool);
    let duration = start.elapsed();

    assert_eq!(counter.load(atomic::Ordering::SeqCst), 10000);
    println!("Time elapsed: {:?}", duration);
}