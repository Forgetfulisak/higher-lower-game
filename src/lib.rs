// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod game;
mod read;

use std::convert::TryInto;

use game::{Active, Comparison, Done, Game, Inactive};
use read::Location;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
  let counts = include_str!("../counts");
  let locs_res: Result<Vec<Location>, String> = counts.lines().map(|l| l.try_into()).collect();
  Model {
    game: GameState::Active(Game::<Active>::new(locs_res.unwrap()).start()),
  }
}

// ------ ------
//     Model
// ------ ------

#[derive(Clone)]
pub enum GameState {
  Inactive(Game<Inactive>),
  Active(Game<Active>),
  Done(Game<Done>),
}

// `Model` describes our app state.
struct Model {
  game: GameState,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
  StartGame,
  Guess(Comparison),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::StartGame => {
      if let GameState::Inactive(game) = model.game.clone() {
        model.game = GameState::Active(game.start());
      } else if let GameState::Done(game) = model.game.clone() {
        model.game = GameState::Active(game.restart())
      }
    },
    Msg::Guess(guess) => {
      if let GameState::Active(game) = model.game.clone() {
        match game.guess(guess) {
          game::GuessResult::Correct(g) => model.game = GameState::Active(g),
          game::GuessResult::Incorrect(g) => model.game = GameState::Done(g),
        }
      }
    },
  }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
  match model.game.clone() {
    GameState::Inactive(_) => view_start_game(model),
    GameState::Active(game) => view_run_game(model, game),
    GameState::Done(game) => view_end_game(model, game),
  }
}

fn view_start_game(_model: &Model) -> Node<Msg> {
  div![button!["Start game!", ev(Ev::Click, |_| Msg::StartGame),],]
}

fn view_run_game(_model: &Model, game: Game<Active>) -> Node<Msg> {
  div![
    style! {
      St::Display => "flex",
      St::AlignItems => "center",
      St::JustifyContent => "center",
    },
    div![
      style! {
        St::Display => "flex",
        St::AlignItems => "center",
        St::JustifyContent => "center",
        St::FlexDirection => "column",
      },
      h2!["Over-Under: Hvilket stedsnavn er mest populært?"],
      p!["Maks score: ", game.max_score, " score: ", game.cur_score],
      div![
        style! {
          St::Display => "flex",
          St::FlexDirection => "row",
          // St::Padding => "20px",
          // St::Margin => "10px"
        },
        div![
          style! {
            St::PaddingRight => "20px",
            St::Margin => "10px"
          },
          p![format!(
            "{} steder heter {:?}",
            game.get_cur_loc().count,
            game.get_cur_loc().name,
          )],
        ],
        div![
          style! {
            // St::Padding => "20px",
            // St::Margin => "10px"
          },
          p![
            format!("{:?} er", game.get_next_loc().name),
            // style! {
            //   St::Display => "inline"
            // },
          ],
          button![
            "Mer",
            ev(Ev::Click, |_| Msg::Guess(Comparison::Higher)),
            // style! {
            //   St::Display => "inline"
            // },
          ],
          button![
            "Mindre",
            ev(Ev::Click, |_| Msg::Guess(Comparison::Lower)),
            style! {
              St::Display => "inline"
            },
          ],
          p![
            format!(" Populært"),
            // style! {
            //   St::Display => "inline"
            // },
          ],
        ]
      ]
    ]
  ]
}

fn view_end_game(_model: &Model, game: Game<Done>) -> Node<Msg> {
  div![
    h1!["Over-Under: Hvilket stedsnavn er mest populært?"],
    p!["Maks score: ", game.max_score, " score: ", game.cur_score],
    p![format!(
      "Navn: {}, Antall: {}",
      game.get_cur_loc().name,
      game.get_cur_loc().count
    )],
    p![format!(
      "Navn: {}, Antall: {}",
      game.get_next_loc().name,
      game.get_next_loc().count
    )],
    button!["Restart", ev(Ev::Click, |_| Msg::StartGame)],
  ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
  // Mount the `app` to the element with the `id` "app".
  App::start("app", init, update, view);
}
