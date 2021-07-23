use std::fmt::Display;

#[derive(Debug)]
struct Satellite {
    name: String,
    velocity: f64,
}

#[derive(Debug)]
struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}

// trait 就是 java 的接口
trait Description {
    fn describe(&self) -> String {
        "description".to_string() // 有方法体的话可以不用实现
    }
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!(
            "{} flying at {} kilomiles per second.",
            &self.name, &self.velocity
        )
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "{} have {} crew at {} kilomiles hight.",
            &self.name, &self.crew_size, &self.altitude
        )
    }
}

impl Display for SpaceStation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.altitude)
    }
}

fn print_type<T: std::fmt::Debug>(item: T) {
    println!("{:?} is {}", item, std::any::type_name::<T>());
}

fn compare_and_print<T: Display + PartialEq + From<V>, V: Display + Copy>(a: T, b: V) {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b);
    }
}

fn get_displayable(space_station: SpaceStation) -> impl Display {
    space_station
}

fn main() {
    let satellite = Satellite {
        name: "神舟飞船".to_string(),
        velocity: 7.9_f64,
    };
    println!("{}", satellite.describe());
    println!("{:?}", satellite);

    let space_station = SpaceStation {
        name: "中国空间站".to_string(),
        crew_size: 3,
        altitude: 425,
    };
    println!("{}", space_station.describe());
    print_type(space_station);

    print_type(32_u64);

    compare_and_print(1.0, 1);

    println!(
        "{}",
        get_displayable(SpaceStation {
            name: "中国空间站".to_string(),
            crew_size: 3,
            altitude: 425,
        })
    );
}
