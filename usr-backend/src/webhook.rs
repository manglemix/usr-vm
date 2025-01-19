use std::{collections::HashMap, time::Instant};

use discord_webhook2::{message::Message, webhook::DiscordWebhook};
use parking_lot::Mutex;
use tracing::error;

struct Locked {
    queue: HashMap<u32, String>,
    deadline: Option<Instant>,
}

pub struct BatchedWebhook {
    locked: Mutex<Locked>,
    discord: DiscordWebhook,
}

impl BatchedWebhook {
    pub fn enqueue(&'static self, id: u32, message: String) {
        let mut guard = self.locked.lock();
        guard.queue.insert(id, message);
        let was_none = guard.deadline.is_none();
        guard.deadline = Some(Instant::now() + std::time::Duration::from_secs(60 * 5));

        if was_none {
            drop(guard);
            tokio::spawn(async move {
                loop {
                    let deadline = self.locked.lock().deadline.unwrap();
                    tokio::time::sleep_until(deadline.into()).await;
                    let queue;
                    {
                        let mut guard = self.locked.lock();
                        if guard.deadline.unwrap() != deadline {
                            continue;
                        }
                        let replacement = HashMap::with_capacity(guard.queue.capacity());
                        queue = std::mem::replace(&mut guard.queue, replacement);
                    }
                    let mut running = String::new();
                    for (_, msg) in queue {
                        if running.len() + msg.len() + 1 < 2000 {
                            running.push_str(&msg);
                            running.push_str("\n");
                        } else {
                            if let Err(e) = self.discord
                                .send(&Message::new(|message| message.content(running)))
                                .await
                            {
                                error!("Failed to trigger webhook: {e}");
                            }
                            running = msg;
                        }
                    }
                    if let Err(e) = self.discord
                        .send(&Message::new(|message| message.content(running)))
                        .await
                    {
                        error!("Failed to trigger webhook: {e}");
                    }
                    let mut guard = self.locked.lock();
                    if guard.queue.is_empty() {
                        guard.deadline = None;
                        break;
                    }
                }
            });
        }
    }
}

impl From<DiscordWebhook> for BatchedWebhook {
    fn from(discord: DiscordWebhook) -> Self {
        Self {
            locked: Mutex::new(Locked {
                queue: HashMap::new(),
                deadline: None,
            }),
            discord,
        }
    }
}