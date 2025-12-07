use chrono::Timelike;
use tokio::time::{interval, Duration};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum Zaphkiel {
    LateNight,
    Morning,
}

pub async fn watch(tx: Sender<Zaphkiel>) {
    let mut ticker = interval(Duration::from_secs(15));

    loop {
        ticker.tick().await;
        let hr = chrono::Local::now().hour();

        if hr >= 0 && hr < 3 {
            tx.send(Zaphkiel::LateNight).await.ok();
        } else if hr >= 9 && hr < 12 {
            tx.send(Zaphkiel::Morning).await.ok();
        }
    }
}
