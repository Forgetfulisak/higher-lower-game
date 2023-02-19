use std::marker::PhantomData;

use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::read::Location;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
  Higher,
  Lower,
  Equal,
}

pub enum GuessResult {
  Correct(Game<Active>),
  Incorrect(Game<Done>),
}

#[derive(Clone)]
pub struct Active {}
#[derive(Clone)]
pub struct Inactive {}
#[derive(Clone)]
pub struct Done {}

#[derive(Clone)]
pub struct Game<Status = Inactive> {
  status: PhantomData<Status>,
  pub max_score: u64,
  pub cur_score: u64,
  cur: usize,
  next: usize,
  locations: Vec<Location>,
  rng: ThreadRng,
}

impl<Status> Game<Status> {
  pub fn new(locs: Vec<Location>) -> Game<Inactive> {
    let locs: Vec<Location> = locs.into_iter().filter(|l| l.count > 100).collect();

    Game {
      status: PhantomData::<Inactive>,
      max_score: 0,
      cur_score: 0,
      cur: 0,
      next: 0,
      locations: locs,
      rng: thread_rng(),
    }
  }

  fn gen_next_loc(&mut self) -> usize {
    assert!(!self.locations.is_empty());
    self.rng.gen_range(0..self.locations.len())
  }
}

impl Game<Inactive> {
  pub fn start(mut self) -> Game<Active> {
    Game::<Active> {
      status: PhantomData,
      max_score: 0,
      cur_score: 0,
      cur: self.gen_next_loc(),
      next: self.gen_next_loc(),
      locations: self.locations,
      rng: self.rng,
    }
  }
}

impl Game<Active> {
  pub fn guess(mut self, guess: Comparison) -> GuessResult {
    let is_correct = match (self.get_truth(), guess) {
      (Comparison::Higher, Comparison::Higher) => true,
      (Comparison::Lower, Comparison::Lower) => true,
      (Comparison::Equal, _) => true,
      (_, _) => false,
    };

    if is_correct {
      GuessResult::Correct(Game {
        status: PhantomData,
        max_score: self.max_score.max(self.cur_score + 1),
        cur_score: self.cur_score + 1,
        cur: self.next,
        next: self.gen_next_loc(),
        locations: self.locations,
        rng: self.rng,
      })
    } else {
      GuessResult::Incorrect(Game {
        status: PhantomData,
        max_score: self.max_score,
        cur_score: 0,
        cur: self.cur,
        next: self.next,
        locations: self.locations,
        rng: self.rng,
      })
    }
  }

  pub fn get_cur_loc(&self) -> &Location {
    self.locations.get(self.cur).unwrap()
  }
  pub fn get_next_loc(&self) -> &Location {
    self.locations.get(self.next).unwrap()
  }

  fn get_truth(&self) -> Comparison {
    match self.get_cur_loc().count.cmp(&self.get_next_loc().count) {
      std::cmp::Ordering::Less => Comparison::Lower,
      std::cmp::Ordering::Equal => Comparison::Equal,
      std::cmp::Ordering::Greater => Comparison::Higher,
    }
  }
}

impl Game<Done> {
  pub fn restart(mut self) -> Game<Active> {
    Game::<Active> {
      status: PhantomData::<Active> {},
      max_score: self.max_score,
      cur_score: 0,
      cur: self.next,
      next: self.gen_next_loc(),
      locations: self.locations,
      rng: self.rng,
    }
  }

  pub fn get_cur_loc(&self) -> &Location {
    self.locations.get(self.cur).unwrap()
  }
  pub fn get_next_loc(&self) -> &Location {
    self.locations.get(self.next).unwrap()
  }
}
