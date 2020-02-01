use gpio::{GpioOut};
use std::{thread, time};

/*
On the board,
    Pin 87 -> Motor Rev
    Pin 89 -> Motor Fwd
*/

/*
Here's how you'd do it in BASH

ON:
    # echo 89 > /sys/class/gpio/export
    # echo out > /sys/class/gpio/gpio49/direction
    # echo 1 > /sys/class/gpio/gpio49/value
*/
// fn door_open() {

// }

fn yeet_door() {
    // Let's try to open GPIO 87 and 89.
    let mut gpio87 = gpio::sysfs::SysFsGpioInput::open(87).unwrap();
    let mut gpio89 = gpio::sysfs::SysFsGpioOutput::open(89).unwrap();

    // GPIO87 and 89 will be toggled every 0.5 sec in the background by a different thread
    // This should cause the door to lock and unlock.
    let mut value = false;
    thread::spawn(move || loop {
        // Motor Forward for 0.5 seconds.
        gpio89.set_value(value).expect("could not set gpio89");
        thread::sleep(time::Duration::from_millis(500));
        value = !value;
        gpio89.set_value(value).expect("could not set gpio89");
        thread::sleep(time::Duration::from_millis(3000)); // Leave it in that state for 3 secs
        // Motor backwards for 0.5 seconds.
        gpio87.set_value(value).expect("could not set gpio87");
        thread::sleep(time::Duration::from_millis(500));
        value = !value;
        gpio87.set_value(value).expect("could not set gpio87");
        thread::sleep(time::Duration::from_millis(3000)); // Leave it in that state for 3 secs
    });

    // The main thread will simply display the current value of GPIO23 every 100ms.
    loop {
        println!("GPIO87: {:?}", gpio87.read_value().unwrap());
        println!("GPIO89: {:?}", gpio89.read_value().unwrap());
        thread::sleep(time::Duration::from_millis(100));
    }
}