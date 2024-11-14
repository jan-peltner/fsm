// simple implementation of a turnstile finite state machine
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
    fn default() -> Fsm {
        return Fsm {
            state: State::LOCKED,
        };
    }
}

impl Fsm {
    fn _new(state: State) -> Fsm {
        return Fsm { state };
    }
    fn transition(&mut self, event: Event) {
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
