// Implement a fsm via a state-transition-lookup-table implemented as a 2d array.
// In this example we'll model the states and transitions of Super Mario.

type StateTransitionTable = [[State; Event::COUNT]; State::COUNT];

const STATE_TRANSITION_TABLE: StateTransitionTable = [
    // Mario
    [
        State::SUPERMARIO, // Mushroom
        State::FIREMARIO,  // Flower
        State::CAPEMARIO,  // Feather
        State::DEAD,       // Damage
    ],
    // Super Mario
    [
        State::SUPERMARIO,
        State::FIREMARIO,
        State::CAPEMARIO,
        State::MARIO,
    ],
    // Fire Mario
    [
        State::FIREMARIO,
        State::FIREMARIO,
        State::CAPEMARIO,
        State::SUPERMARIO,
    ],
    // Cape Mario
    [
        State::CAPEMARIO,
        State::FIREMARIO,
        State::CAPEMARIO,
        State::SUPERMARIO,
    ],
    // 💀
    [State::DEAD, State::DEAD, State::DEAD, State::DEAD],
];

#[derive(Debug, Copy, Clone)]
enum State {
    MARIO,
    SUPERMARIO,
    FIREMARIO,
    CAPEMARIO,
    DEAD,
}

impl State {
    const COUNT: usize = 5;
}

enum Event {
    MUSHROOM,
    FLOWER,
    FEATHER,
    DAMAGE,
}

impl Event {
    const COUNT: usize = 4;
}

struct Fsm {
    state: State,
    state_transition_table: StateTransitionTable,
}

impl Default for Fsm {
    fn default() -> Fsm {
        return Fsm {
            state: State::MARIO,
            state_transition_table: STATE_TRANSITION_TABLE,
        };
    }
}

impl Fsm {
    fn transition(&mut self, event: Event) {
        self.state = self.state_transition_table[self.state as usize][event as usize];
    }

    fn dump(&self) {
        println!("{:?}", self.state);
    }

    fn transition_and_dump(&mut self, event: Event) {
        self.transition(event);
        self.dump();
    }
}

fn main() {
    let mut mario = Fsm::default();
    mario.dump();
    mario.transition_and_dump(Event::MUSHROOM);
    mario.transition_and_dump(Event::MUSHROOM);
    mario.transition_and_dump(Event::DAMAGE);
    mario.transition_and_dump(Event::FLOWER);
    mario.transition_and_dump(Event::FEATHER);
    mario.transition_and_dump(Event::MUSHROOM);
    mario.transition_and_dump(Event::DAMAGE);
    mario.transition_and_dump(Event::DAMAGE);
    mario.transition_and_dump(Event::DAMAGE);
    mario.transition_and_dump(Event::MUSHROOM);
}
