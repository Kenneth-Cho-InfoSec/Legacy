package com.jni.WifiCameraInfo;

import java.util.LinkedList;
import java.util.List;

public final class ThreadPool {
    private static ThreadPool threadPool = null;
    private static int worker_num = 10;
    private List<Runnable> taskQueue;
    private WorkThread[] workThreads;

    public List<Runnable> getTaskQueue() {
        return this.taskQueue;
    }

    private ThreadPool() {
        this(5);
    }

    private ThreadPool(int i) {
        this.taskQueue = new LinkedList();
        worker_num = i;
        this.workThreads = new WorkThread[i];
        for (int i2 = 0; i2 < i; i2++) {
            this.workThreads[i2] = new WorkThread();
            this.workThreads[i2].start();
        }
    }

    public static ThreadPool getThreadPool() {
        return getThreadPool(worker_num);
    }

    public static ThreadPool getThreadPool(int i) {
        if (threadPool == null) {
            threadPool = new ThreadPool(i);
        }
        return threadPool;
    }

    public void addTask(Runnable runnable) {
        synchronized (this.taskQueue) {
            this.taskQueue.add(runnable);
            this.taskQueue.notifyAll();
        }
    }

    public void destroy() {
        while (!this.taskQueue.isEmpty()) {
            try {
                Thread.sleep(10L);
            } catch (InterruptedException unused) {
            }
        }
        for (int i = 0; i < worker_num; i++) {
            this.workThreads[i].stopWorker();
            this.workThreads[i] = null;
        }
        threadPool = null;
        this.taskQueue.clear();
    }

    private class WorkThread extends Thread {
        private boolean isRunning;

        private WorkThread() {
            this.isRunning = true;
        }

        @Override
        public void run() {
            Runnable runnable;
            while (true) {
                if (!this.isRunning) {
                    return;
                }
                synchronized (ThreadPool.this.taskQueue) {
                    while (this.isRunning && ThreadPool.this.taskQueue.isEmpty()) {
                        try {
                            ThreadPool.this.taskQueue.wait(20L);
                        } catch (InterruptedException unused) {
                        }
                    }
                    runnable = ThreadPool.this.taskQueue.isEmpty() ? null : (Runnable) ThreadPool.this.taskQueue.remove(0);
                }
                if (runnable != null) {
                    runnable.run();
                }
            }
        }

        public void stopWorker() {
            this.isRunning = false;
        }
    }
}
