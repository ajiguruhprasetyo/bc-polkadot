fn main() {
    exercise_one();

    let statuses = [
        StatusNew::Start { x: 10, y: 30 },
        StatusNew::Pending(String::from("hello world")),
        StatusNew::OnGoing("lang".to_string(), "long".to_string()),
        StatusNew::Done,
        StatusNew::Finish,
    ];

    for status in &statuses {
        status.get_status();
    }

    let mut state = State {
        quit: false,
        position: Point { x: 3, y: 5 },
        color: (11, 255, 12),
        message: "hello state enum".to_string(),
    };
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Echo(String::from("Hello state enum!")));
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Quit);
}

enum Message {
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

#[derive(Debug)]
enum Status {
    Start,
    Pending,
    OnGoing,
    Done,
    Finish,
}

#[derive(Debug)]
enum StatusNew {
    Start { x: i32, y: i32 },
    Pending(String),
    OnGoing(String, String),
    Done,
    Finish,
}

fn exercise_one() {
    println!("{:?}", Status::Start);
    println!("{:?}", Status::Pending);
    println!("{:?}", Status::OnGoing);
    println!("{:?}", Status::Done);
    println!("{:?}", Status::Finish);
}

impl StatusNew {
    fn get_status(&self) {
        match self {
            StatusNew::Start { x, y } => println!("Start at ({}, {})", x, y),
            StatusNew::Pending(msg) => println!("Pending: {}", msg),
            StatusNew::OnGoing(a, b) => println!("OnGoing: {} {}", a, b),
            StatusNew::Done => println!("Done"),
            StatusNew::Finish => println!("Finish"),
        }
    }
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        println!("Moving to position: ({}, {})", p.x, p.y);
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(red, blue, green) => self.change_color((red, blue, green)),
            Message::Echo(message) => self.echo(message),
            Message::Move(position) => self.move_position(position),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
