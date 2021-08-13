use rand::prelude::*;

struct Toast {
    name: String,
    description: String,
    img_href: String,
}

fn main() {

    let toasts: [Toast; 3] = [
        Toast {
            name: "RAW TOAST".to_owned(),
            description: "Served straight or on the rocks.".to_owned(),
            img_href: "assets/images/raw_toast.jpg".to_owned()
        },
        Toast {
            name: "FRENCH TOAST".to_owned(),
            description: "pan-fried goodness.".to_owned(),
            img_href: "assets/images/french_toast.jpg".to_owned()
        },
        Toast {
            name: "PEANUT BUTTER POWER TOAST".to_owned(),
            description: "peanut butter, banana, and cha seeds. FEEL THE POWER!".to_owned(),
            img_href: "assets/images/peanut_butter_power_toast.jpg".to_owned()
        }
    ];

    println!("Content-Type: text/html\n");
    println!("<h1>Welcome to the Toast on Demand web site</h1>");
    println!("<p>Our mission is to provide you with delicious and nutritious toast - where you want it, when you want it, and as browned as you want it.</p>");

    let mut rng = thread_rng();
    let selected_toast = rng.gen_range(0..=2);
    println!("<h2>TOAST OF THE DAY</h2>");
    println!("<p>Today's special is {}</p>", toasts[selected_toast].name);
    println!("<p>{}</p>", toasts[selected_toast].description);
    println!("<p><img src='{}' /></p>", toasts[selected_toast].img_href);
}
