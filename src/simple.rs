// simple implementation of a turnstile finite state machine.
// https://en.wikipedia.org/wiki/Finite-state_machine
enum State {
    UNLOCKED,
    LOCKED,
}

enum Event {
    PUSH,
    COIN,
}

struct Fsm {
    state: State,
}

impl Default for Fsm {
    fn default() -> Self {
        return Self {
            state: State::LOCKED,
        };
    }
}

impl Fsm {
    fn _new(state: State) -> Self {
        return Self { state };
    }
    fn transition(&mut self, event: Event) {
        // this outer match statement isn't strictly necessary, since the inner matches are
        // identical for each state. However, to illustrate the idea of a fsm properly,
        // we will match on the state regardless.
        match self.state {
            State::LOCKED => match event {
                Event::PUSH => {
                    println!("Turnstile locked. Please insert a coin!");
                }
                Event::COIN => {
                    self.state = State::UNLOCKED;
                    println!("Turnstile now unlocked. Please push!");
                }
            },
            State::UNLOCKED => match event {
                Event::PUSH => {
                    self.state = State::LOCKED;
                    println!("Turnstile now locked again. Have a nice day!");
                }
                Event::COIN => {
                    println!("Turnstile has already been unlocked. Please push!");
                }
            },
        }
    }
}
fn main() {
    let mut fsm = Fsm::default();
    fsm.transition(Event::PUSH);
    fsm.transition(Event::PUSH);
    fsm.transition(Event::COIN);
    fsm.transition(Event::COIN);
    fsm.transition(Event::PUSH);
    fsm.transition(Event::PUSH);
}
