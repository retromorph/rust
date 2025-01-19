#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentOutcome {
    Cooperate,
    Cheat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

////////////////////////////////////////////////////////////////////////////////

pub struct Game {
    left_agent: Box<dyn Agent>,
    right_agent: Box<dyn Agent>,

    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left_agent: Box<dyn Agent>, right_agent: Box<dyn Agent>) -> Self {
        Self {
            left_agent,
            right_agent,
            left_score: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_outcome = self.left_agent.play();
        let right_outcome = self.right_agent.play();

        self.left_agent.update(right_outcome);
        self.right_agent.update(left_outcome);

        match (left_outcome, right_outcome) {
            (AgentOutcome::Cooperate, AgentOutcome::Cooperate) => {
                self.left_score += 2;
                self.right_score += 2;
                RoundOutcome::BothCooperated
            }
            (AgentOutcome::Cooperate, AgentOutcome::Cheat) => {
                self.left_score -= 1;
                self.right_score += 3;
                RoundOutcome::RightCheated
            }
            (AgentOutcome::Cheat, AgentOutcome::Cooperate) => {
                self.left_score += 3;
                self.right_score -= 1;
                RoundOutcome::LeftCheated
            }
            (AgentOutcome::Cheat, AgentOutcome::Cheat) => {
                self.left_score += 0;
                self.right_score += 0;
                RoundOutcome::BothCheated
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Agent {
    fn play(&self) -> AgentOutcome;

    fn update(&mut self, opponent_outcome: AgentOutcome);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn play(&self) -> AgentOutcome {
        AgentOutcome::Cheat
    }

    fn update(&mut self, _: AgentOutcome) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn play(&self) -> AgentOutcome {
        AgentOutcome::Cooperate
    }

    fn update(&mut self, _: AgentOutcome) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    is_enraged: bool,
}

impl Agent for GrudgerAgent {
    fn play(&self) -> AgentOutcome {
        if self.is_enraged {
            AgentOutcome::Cheat
        } else {
            AgentOutcome::Cooperate
        }
    }

    fn update(&mut self, opponent_outcome: AgentOutcome) {
        if opponent_outcome == AgentOutcome::Cheat {
            self.is_enraged = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {
    last_opponent_outcome: Option<AgentOutcome>,
}

impl Agent for CopycatAgent {
    fn play(&self) -> AgentOutcome {
        if self.last_opponent_outcome.is_none() {
            AgentOutcome::Cooperate
        } else {
            self.last_opponent_outcome.unwrap()
        }
    }

    fn update(&mut self, opponent_outcome: AgentOutcome) {
        self.last_opponent_outcome = Some(opponent_outcome);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    round_number: u32,
    is_opponent_cheated: bool,
    last_opponent_outcome: Option<AgentOutcome>,
}

impl Agent for DetectiveAgent {
    fn play(&self) -> AgentOutcome {
        if self.round_number == 0 {
            AgentOutcome::Cooperate
        } else if self.round_number == 1 {
            AgentOutcome::Cheat
        } else if self.round_number == 2 || self.round_number == 3 {
            AgentOutcome::Cooperate
        } else if self.is_opponent_cheated {
            self.last_opponent_outcome.unwrap()
        } else {
            AgentOutcome::Cheat
        }
    }

    fn update(&mut self, opponent_outcome: AgentOutcome) {
        if opponent_outcome == AgentOutcome::Cheat {
            self.is_opponent_cheated = true;
        }
        self.last_opponent_outcome = Some(opponent_outcome);
        self.round_number += 1;
    }
}
