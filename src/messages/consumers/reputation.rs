struct Reputation {
    scores: HashMap<String, u32>,
}

fn reputation(){
    while let Ok(event) = rx.recv().await {

        match event {

            Event::AttackObserved(..)=>...

            Event::BruteForceDetected{..}=>...

            Event::PortScanDetected{..}=>...

            _=>{}
        }
    }
}
