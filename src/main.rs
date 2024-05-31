pub mod wmctrl;

fn main() {
    let windows = wmctrl::list();
    for window in windows {
        println!("{}", window.name);
    }

    wmctrl::activate("chro");
    std::thread::sleep(std::time::Duration::from_secs(2));
    wmctrl::activate("term");
}
