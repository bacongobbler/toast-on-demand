struct Toast {
    name: String,
    description: String,
    img_href: String,
}

fn main() {
  println!("Content-Type: text/html\n");
  println!("<h1>Welcome to the Toast on Demand web site</h1>");
  println!("<p>Our mission is to provide you with delicious and nutritious toast - where you want it, when you want it, and as browned as you want it.</p>");

  let raw_toast = Toast {
      name: "RAW TOAST".to_owned(),
      description: "Served straight or on the rocks.".to_owned(),
      img_href: "assets/images/raw_toast.jpg".to_owned()
  };

  println!("<h2>TOAST OF THE DAY</h2>");
  println!("<p>Today's special is {}</p>", raw_toast.name);
  println!("<p>{}</p>", raw_toast.description);
  println!("<p><img src='{}' /></p>", raw_toast.img_href);
}
