fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

}

fn while_let() {
    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop_matching() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
    // can pass in something like this
    // let point = (2, 3);
    // print_coordinates(&point);
}

fn match_literals() {
    let x = 2;
        match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_advanced() {
    let (x, y, z) = (Some(50), Some('a'), Some(20));

    // if you introduct a variable of the same name like we did here with y, 
    // the match will sorta override y as you can see
    match x {
        Some(50) => println!("Got 50"),
        // match num between 30 and 40

        Some(30..= 40) => println!("got a num between 30 and 40"),
        //   OR VVV

        // works if type is char
        Some(20) | Some(10) => println!("got 20 or ten"),
        // careful using letters that match variables. here y does not mean the
        // ten that we delcared above, rather, it matches any "Some()" value
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    match y {
        Some('a'..='z') => println!("got 20 or ten"),
        _ => println!("not from a-z!"),
    }
}

fn destrutured_assignment() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // EXAMPLE DESTRUCTURING
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }


}

fn nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
    // Complicated shit
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn discarding_matches () {
    struct Pokemon<'a> {
        health: i32,
        typing: &'a str,
        weakness: &'a str,
        level: i8,
        moves: [&'a str; 4],
        name: &'a str,
    }
    let pok1 = Pokemon {
        health: 22,
        name: "Charmander",
        typing: "fire",
        weakness: "water",
        level: 5,
        moves: ["ember", "scratch", "flamethrower", "protect"],
    };

    match pok1 {
        Pokemon {
            name,
            moves: [first, .., last],
            ..
        } => {
            println!("Name: {}", name);
            println!("First move: {}", first);
            println!("Last move: {}", last);
        }
    }
}

fn match_guards () { 
    let num = Some(4);

    match num {//   VVVV <---- match guard
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // BIND TO VAR VVV <--- AND ALSO TEST IT
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

}
// println!("at the end: x = {:?}, y = {y}", x);}
fn main() {
    discarding_matches()
}
