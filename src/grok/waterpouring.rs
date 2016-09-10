use std::cmp;

#[derive(Clone, Debug)]
enum Action {
  Fill { glass: usize },
  Empty { glass: usize },
  Transfer { from: usize, to: usize },
}

#[derive(Clone, Debug)]
pub struct State {
  glasses: Vec<usize>,
  actions: Vec<Action>,
}

trait HasChangeState {
  fn change(&self, action: &Action, glass_volumes: &Vec<usize>) -> State;
}

impl HasChangeState for State {
  fn change(&self, action: &Action, glass_volumes: &Vec<usize>) -> State {
    let mut new_glasses = self.glasses.to_owned();
    let mut new_actions = self.actions.to_owned();
    new_actions.push(action.to_owned());
    
    match *action {
      Action::Fill { glass } => new_glasses[glass] = glass_volumes[glass],
      Action::Empty { glass } => new_glasses[glass] = glass_volumes[glass],
      Action::Transfer { from, to } => {
        let change = cmp::min(self.glasses[from], glass_volumes[to] - self.glasses[to]);
        new_glasses[from] = new_glasses[from] - change;
        new_glasses[to] = new_glasses[to] + change;
      }
    }
    State { glasses: new_glasses, actions: new_actions }
  }
}

pub fn solve(max_volumes: &Vec<usize>, required_volume: usize) -> State {
  let volumes: Vec<usize> = vec![0, glass_volumes.len()];
  
  
  State { glasses: Vec::new(), actions: Vec::new() }
}