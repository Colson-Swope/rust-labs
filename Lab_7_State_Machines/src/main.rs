enum State {
    Standing, 
    Laying, 
    Sitting, 
    Walking, 
    Running, 
    Jumping, 
    Falling, 
    Dead, 
    
}   

#[methods_enum::gen(Act: run)]
impl State {
    pub fn set(&mut self); 
    pub fn input_char(&mut self, ch: char); 

    fn run(&mut self, act: Act) {
        match self {
            State::Standing => match act {
                Act::set() => println!("Standing"), 

                Act::input_char('k') => self.set_state(State::Walking), 
                Act::input_char('l') => self.set_state(State::Running), 
                Act::input_char('j') => self.set_state(State::Jumping), 
                Act::input_char('h') => self.set_state(State::Sitting), 
                _ => self.set(), 
            },
            State::Laying => match act {
                Act::set() => println!("Laying"), 

                Act::input_char('j') => self.set_state(State::Sitting), 
                _ => self.set(), 
            },
            State::Sitting => match act {
                Act::set() => println!("Sitting"),

                Act::input_char('h') => self.set_state(State::Laying), 
                Act::input_char('j') => self.set_state(State::Standing), 
                _ => self.set(), 
            },
            State::Walking => match act {
                Act::set() => println!("Walking"), 

                Act::input_char('l') => self.set_state(State::Running), 
                Act::input_char('h') => self.set_state(State::Standing), 
                Act::input_char('j') => self.set_state(State::Jumping), 
                _ => self.set(), 
            }, 
            State::Running => match act {
                Act::set() => println!("Running"), 

                Act::input_char('k') => self.set_state(State::Walking), 
                Act::input_char('j') => self.set_state(State::Jumping), 
                Act::input_char('h') => self.set_state(State::Standing), 
                _ => self.set(), 
            },
            State::Jumping => match act {
                Act::set() => println!("Jumping"), 

                Act::input_char('h') => self.set_state(State::Falling), 
                Act::input_char('j') => self.set_state(State::Falling), 
                Act::input_char('k') => self.set_state(State::Falling), 
                Act::input_char('l') => self.set_state(State::Falling), 
                _ => self.set(), 
            },
            State::Falling => match act {
                Act::set() => println!("Falling"),

                Act::input_char('h') => self.set_state(State::Dead), 
                Act::input_char('j') => self.set_state(State::Standing), 
                Act::input_char('k') => self.set_state(State::Standing), 
                Act::input_char('l') => self.set_state(State::Standing), 
                _ => self.set(), 
            },
            State::Dead => match act {
                Act::set() => println!("Dead"), 
                
                _ => panic!("Error"), 
            }, 
            
        }
    }

    fn set_state(&mut self, new_state: State) {
        *self = new_state; 
        self.set(); 
    }
}

fn main() {
    let mut machine = State::Standing;

    // Changed type to String and initialized with "left"
    let mut walking = String::from("left");
    machine.set();

    while !matches!(&machine, State::Dead) {
        machine.input_char(char_entered());

        // Changed values from true/false to "left"/"right"
        match machine {
            State::Walking if walking == "left" => walking = String::from("right"),
            State::Standing if walking == "right" => walking = String::from("left"),
            _ => (),
        }

        println!("{}", walking);
    }
}

fn char_entered() -> char {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).unwrap_or(0);
    text.chars().next().unwrap_or('\x0d')
}