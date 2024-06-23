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
        let rect1 = (30, 50);
        println!("The area of the rectangle is {} square pixels", area(rect1));
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
