use std::io::{self, Write, Stdout};
use std::sync::{Arc, Mutex};
use colored::*;
use std::time::Instant;

pub struct ProgressBar {
    total: u64,
    current: u64,
    width: usize,
    message: String,
    start_time: Instant,
    stdout: Mutex<Stdout>,
    finished: bool,
}

impl ProgressBar {
    pub fn new(total: u64) -> Arc<Mutex<Self>> {
        let progress = ProgressBar {
            total,
            current: 0,
            width: 30,
            message: String::new(),
            start_time: Instant::now(),
            stdout: Mutex::new(io::stdout()),
            finished: false,
        };

        Arc::new(Mutex::new(progress))
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
        self.redraw();
    }

    pub fn update(&mut self, current: u64) {
        self.current = current;
        if current > self.total {
            self.current = self.total;
        }
        self.redraw();
    }

    pub fn increment(&mut self, amount: u64) {
        self.update(self.current + amount);
    }

    pub fn finish_with_message(&mut self, message: &str) {
        self.current = self.total;
        self.message = message.to_string();
        self.finished = true;
        self.redraw();
        println!();
    }

    fn redraw(&self) {
        let percent = if self.total == 0 { 100 } else { (self.current as f64 / self.total as f64 * 100.0) as u8 };
        let filled_width = (self.width as f64 * (percent as f64 / 100.0)) as usize;
        
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 { self.current as f64 / elapsed } else { 0.0 };
        let eta = if speed > 0.0 && self.current < self.total {
            (self.total - self.current) as f64 / speed
        } else {
            0.0
        };

        let bar = format!(
            "{} {}% [{}{}>{}] {}/{} [{:.2}/s, ETA: {:.1}s] {}",
            if self.finished { "✓".green() } else { "•".bright_blue() },
            percent,
            "=".repeat(filled_width),
            if filled_width < self.width { ">" } else { "" },
            " ".repeat(self.width - filled_width.min(self.width)),
            self.current,
            self.total,
            speed,
            eta,
            self.message
        );

        if let Ok(mut stdout) = self.stdout.lock() {
            let _ = write!(stdout, "\r{}", bar);
            let _ = stdout.flush();
        }
    }
}

type ProgressBarStorage = Vec<Arc<Mutex<ProgressBar>>>;
static PROGRESS_BARS: once_cell::sync::Lazy<Mutex<ProgressBarStorage>> = 
    once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));
static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn create_progress_bar(total: u64) -> usize {
    let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let progress = ProgressBar::new(total);
    
    if let Ok(mut bars) = PROGRESS_BARS.lock() {
        bars.push(progress);
    }

    id
}

pub fn update_progress(id: usize, current: u64, message: Option<&str>) -> bool {
    if let Some(progress) = get_progress_bar(id) {
        if let Ok(mut bar) = progress.lock() {
            bar.update(current);
            if let Some(msg) = message {
                bar.set_message(msg);
            }
            return true;
        }
    }
    false
}

pub fn finish_progress(id: usize, message: Option<&str>) -> bool {
    if let Some(progress) = get_progress_bar(id) {
        if let Ok(mut bar) = progress.lock() {
            bar.finish_with_message(message.unwrap_or("完了！"));
            return true;
        }
    }
    false
}

fn get_progress_bar(id: usize) -> Option<Arc<Mutex<ProgressBar>>> {
    if let Ok(bars) = PROGRESS_BARS.lock() {
        if id < bars.len() {
            return Some(bars[id].clone());
        }
    }
    None
}