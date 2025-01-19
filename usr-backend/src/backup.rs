use std::{process::Command, sync::atomic::Ordering, time::Duration};

use crate::UsrState;

pub fn backup_db(state: &'static UsrState) {
    if state.backup_task_running.swap(true, Ordering::Relaxed) {
        return;
    }
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60 * 10)).await;
        state.backup_task_running.store(false, Ordering::Relaxed);
        if let Err(e) = std::fs::copy("usr-db.sqlite", "../usr-db-backup/usr-db.sqlite") {
            tracing::error!("Failed to copy database: {}", e);
            return;
        }
        if let Err(e) = Command::new("git")
            .arg("add")
            .arg("usr-db.sqlite")
            .current_dir("../usr-db-backup")
            .output()
        {
            tracing::error!("Failed to add files to git: {}", e);
        }
        if let Err(e) = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Automated backup")
            .current_dir("../usr-db-backup")
            .output()
        {
            tracing::error!("Failed to commit files to git: {}", e);
        }
        if let Err(e) = Command::new("git")
            .arg("push")
            .current_dir("../usr-db-backup")
            .output()
        {
            tracing::error!("Failed to push files to git: {}", e);
        }
    });
    
}