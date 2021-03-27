#![windows_subsystem = "windows"]

use std::convert::TryInto;

use chrono::{Duration, Local, NaiveTime};

struct ClassTime {
    start_time: NaiveTime,
    duration: Duration,
    end_time: NaiveTime,
    no: i32,
}
impl ClassTime {
    fn new(start_time: NaiveTime, duration: Duration, no: i32) -> Self {
        Self {
            start_time,
            duration,
            end_time: start_time + duration,
            no,
        }
    }
    fn after(class_time: &ClassTime, duration: Duration) -> Self {
        Self {
            start_time: class_time.end_time + duration,
            end_time: class_time.end_time + duration + class_time.duration,
            duration: class_time.duration,
            no: class_time.no + 1,
        }
    }
}

struct CurrentClassTime<'a> {
    current: &'a ClassTime,
    next: &'a ClassTime,
    is_class_time: bool,
}

impl<'a> CurrentClassTime<'a> {
    fn get(current: NaiveTime, class_times: &'a [ClassTime]) -> Option<Self> {
        for (current_class, next_class) in class_times.iter().zip(class_times.iter().skip(1)) {
            if current_class.start_time < current {
                if current_class.end_time > current {
                    return Some(Self {
                        current: current_class,
                        next: next_class,
                        is_class_time: true,
                    });
                }
                if next_class.start_time > current {
                    return Some(Self {
                        current: current_class,
                        next: next_class,
                        is_class_time: false,
                    });
                }
            }
        }
        None
    }
}

sixtyfps::include_modules!();

fn main() {
    let mut classes = vec![ClassTime::new(
        NaiveTime::from_hms(7, 50, 0),
        Duration::minutes(45),
        1,
    )];
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

    let main_window = Main::new();

    main_window.set_class_no(2);
    main_window.set_is_class_time(false);

    // let current = NaiveTime::from_hms(10, 31, 0);
    let current = Local::now().time();
    let class = CurrentClassTime::get(current, &classes);
    if let Some(class) = class {
        if class.is_class_time {
            main_window.set_start_time(class.current.start_time.format("%H:%M").to_string().into());
            main_window.set_end_time(class.current.end_time.format("%H:%M").to_string().into());
        } else {
            main_window.set_start_time(class.current.end_time.format("%H:%M").to_string().into());
            main_window.set_end_time(class.next.start_time.format("%H:%M").to_string().into());
        }
        main_window.set_is_class_time(class.is_class_time);
        main_window.set_class_no(class.current.no);
        main_window.set_left_time(
            (class.next.start_time - current)
                .num_minutes()
                .try_into()
                .unwrap(),
        )
    }

    main_window.run();
}
