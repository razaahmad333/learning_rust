pub fn play_it() {
    enum TrafficLight {
        RED,
        GREEN,
        // YELLOW,
    }

    impl TrafficLight {
        fn can_walk(&self) -> bool {
            match self {
                TrafficLight::GREEN => true,
                _ => false,
            }
        }
    }

    let light: TrafficLight = TrafficLight::GREEN;

    println!("can walk {}", light.can_walk())
}
