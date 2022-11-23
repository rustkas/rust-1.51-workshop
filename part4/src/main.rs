fn main() {
    let mut city_names = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    //"👉 TODO Use .pop() to remove the last city from the list.";
    let last_city = match city_names.pop() {
        Some(inner_value) => inner_value,
        None => "",
    };

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    city_names.push(last_city);

    println!("Here is the full list of cities:");

    for city_name in city_names.iter() {
        print!("* {}", city_name)
    }
    // 👉 TODO print each of the city names.
    //
    // 💡 TIP: Here's an example of `for` loop syntax:
    //
    //     for my_element in my_vec.iter() { ... }
}
