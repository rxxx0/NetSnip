use anyhow::Result;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PacketQueue {
    pub max_rate: usize,
    pub current_tokens: usize,
    pub last_refill: Instant,
}

pub struct BandwidthController {
    packet_queues: Arc<Mutex<HashMap<IpAddr, PacketQueue>>>,
    statistics: Arc<Mutex<HashMap<IpAddr, BandwidthStats>>>,
}

#[derive(Debug, Clone)]
pub struct BandwidthStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_dropped: u64,
    pub last_update: Instant,
}

impl BandwidthController {
    pub fn new() -> Self {
        Self {
            packet_queues: Arc::new(Mutex::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn limit_bandwidth(&self, ip: IpAddr, limit_mbps: f64) -> Result<()> {
        let bytes_per_second = (limit_mbps * 1_000_000.0 / 8.0) as usize;
        let mut queues = self.packet_queues.lock().await;

        queues.insert(
            ip,
            PacketQueue {
                max_rate: bytes_per_second,
                current_tokens: bytes_per_second,
                last_refill: Instant::now(),
            },
        );

        Ok(())
    }

    pub async fn remove_limit(&self, ip: IpAddr) -> Result<()> {
        let mut queues = self.packet_queues.lock().await;
        queues.remove(&ip);
        Ok(())
    }

    pub async fn should_forward_packet(&self, from_ip: IpAddr, packet_size: usize) -> bool {
        let mut queues = self.packet_queues.lock().await;

        if let Some(queue) = queues.get_mut(&from_ip) {
            // Token bucket algorithm
            let now = Instant::now();
            let elapsed = now.duration_since(queue.last_refill);
            let tokens_to_add = (elapsed.as_secs_f64() * queue.max_rate as f64) as usize;

            queue.current_tokens = (queue.current_tokens + tokens_to_add).min(queue.max_rate);
            queue.last_refill = now;

            if queue.current_tokens >= packet_size {
                queue.current_tokens -= packet_size;

                // Update statistics
                self.update_stats(from_ip, packet_size, false).await;
                return true;
            }

            // Packet dropped
            self.update_stats(from_ip, 0, true).await;
            return false;
        }

        // No limit set, forward packet
        self.update_stats(from_ip, packet_size, false).await;
        true
    }

    async fn update_stats(&self, ip: IpAddr, bytes: usize, dropped: bool) {
        let mut stats = self.statistics.lock().await;
        let entry = stats.entry(ip).or_insert_with(|| BandwidthStats {
            bytes_sent: 0,
            bytes_received: 0,
            packets_dropped: 0,
            last_update: Instant::now(),
        });

        if dropped {
            entry.packets_dropped += 1;
        } else {
            entry.bytes_sent += bytes as u64;
        }
        entry.last_update = Instant::now();
    }

    pub async fn get_statistics(&self, ip: IpAddr) -> Option<BandwidthStats> {
        let stats = self.statistics.lock().await;
        stats.get(&ip).cloned()
    }

    pub async fn get_all_statistics(&self) -> HashMap<IpAddr, BandwidthStats> {
        let stats = self.statistics.lock().await;
        stats.clone()
    }

    pub async fn reset_statistics(&self) {
        let mut stats = self.statistics.lock().await;
        stats.clear();
    }
}