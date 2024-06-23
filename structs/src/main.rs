struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    {
        // Similar to Tuple Data Type, but with a little twist. We can name the element, just like this
        // example below. In a given tuple "user" we would have (bool, String, String, u64), but we
        // would only get the value of these by index tuple_user.0 to tuple_user.3. A Structure is
        // simpler to read and to understand, because we now know that the index .0 on that tuple, is
        // "active", and so on, calling it (at the end of this section) from user1.active.
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }

        let user1 = User {
            active: true,
            username: String::from("UserNaame"),
            email: String::from("user@name.com"),
            sign_in_count: 1,
        };

        println!(
            "Active user?: {} / Username: {} / User email: {} / ID: {}",
            user1.active, user1.username, user1.email, user1.sign_in_count
        );
    }

    {
        let rect1 = Rect {
            width: 30,
            height: 50,
        };
        let rect2 = Rect {
            width: 50,
            height: 100,
        };
        println!(
            "The area of the rectangle is {} square pixels",
            rect1.area()
        );

        println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
        println!("Can rect2 hold rect1 ? {}", rect2.can_hold(&rect1));
    }
}
