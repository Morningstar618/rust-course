struct Km {
    value: i32,
}

struct Kmh {
    value: i32,
}

// Associated Traits - The use of "Associated types" improves the overall readability
// of code by moving inner types locally into a trait as output types.
trait DistanceThreeHours {
    type Distance;
    fn distance_covered(&self, hours: i32) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_covered(&self, hours: i32) -> Self::Distance {
        Km {
            value: self.value * hours,
        }
    }
}

fn main() {
    let speed = Kmh { value: 50 };
    let distance_covered = speed.distance_covered(6);

    println!("Total distance covered:  {} kms", distance_covered.value);
}
