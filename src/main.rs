use chrono::{Duration, NaiveTime};

#[derive(Debug)]
struct ClassTime {
    start_time: NaiveTime,
    duration: Duration,
    end_time: NaiveTime,
}
impl ClassTime {
    fn new(start_time: NaiveTime, duration: Duration) -> Self {
        Self {
            start_time,
            duration,
            end_time: start_time + duration,
        }
    }
    fn after(class_time: &ClassTime, duration: Duration) -> Self {
        Self {
            start_time: class_time.end_time + duration,
            end_time: class_time.end_time + duration + class_time.duration,
            duration: class_time.duration,
        }
    }
}

sixtyfps::include_modules!();

fn main() {
    let mut classes = Vec::new();
    classes.push(ClassTime::new(
        NaiveTime::from_hms(7, 50, 0),
        Duration::minutes(45),
    ));
    classes.push(ClassTime::after(
        classes.last().unwrap(),
        Duration::minutes(10),
    ));
    classes.push(ClassTime::after(
        classes.last().unwrap(),
        Duration::minutes(10),
    ));
    classes.push(ClassTime::after(
        classes.last().unwrap(),
        Duration::minutes(10),
    ));
    classes.push(ClassTime::after(
        classes.last().unwrap(),
        Duration::minutes(70),
    ));
    classes.push(ClassTime::after(
        classes.last().unwrap(),
        Duration::minutes(10),
    ));
    classes.push(ClassTime::after(
        classes.last().unwrap(),
        Duration::minutes(10),
    ));

    println!("{:#?}", classes);

    let main_window = Main::new();

    main_window.set_class_no(2);
    main_window.set_is_class_time(false);

    main_window.run();
}
