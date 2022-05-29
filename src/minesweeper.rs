use crate::random::random_mine;

use std::{collections::HashSet, fmt::Display};

pub type Position = (usize, usize);

pub enum OpenResult {
  Mine,
  NoMine(usize),
}

#[derive(Debug)]
pub struct Minesweeper {
  width: usize,
  height: usize,
  open_fields: HashSet<Position>,
  mines: HashSet<Position>,
  flagged_fields: HashSet<Position>,
  lost: bool,
}

impl Display for Minesweeper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        let pos = (x, y);
        if !self.open_fields.contains(&pos) {
          if self.lost && self.mines.contains(&pos) {
            f.write_str("💣 ")?;
          } else if self.flagged_fields.contains(&pos) {
            f.write_str("🚩 ")?;
          } else {
            f.write_str("🟦 ")?;
          }
        } else if self.mines.contains(&pos) {
          f.write_str("💥 ")?;
        } else {
          let numbers: [&str; 8] = ["⬜️", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣"];
          write!(f, "{} ", numbers[self.neighboring_mines(pos)])?;
        }
      }
      f.write_str("\n")?;
    }

    Ok(())
  }
}

impl Minesweeper {
  pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
    Minesweeper {
      width,
      height,
      open_fields: HashSet::new(),
      mines: {
        let mut mines = HashSet::new();

        while mines.len() < mine_count {
          mines.insert(random_mine(width, height));
        }

        mines
      },
      flagged_fields: HashSet::new(),
      lost: false,
    }
  }

  pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
    let width = self.width;
    let height = self.height;
    (x.max(1) - 1..=(x + 1).min(width - 1))
      .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
      .filter(move |&pos| pos != (x, y))
  }

  pub fn neighboring_mines(&self, pos: Position) -> usize {
    self
      .iter_neighbors(pos)
      .filter(|pos| self.mines.contains(pos))
      .count() as usize
  }

  pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
    if self.lost || self.open_fields.contains(&pos) || self.flagged_fields.contains(&pos) {
      return None;
    }
    self.open_fields.insert(pos);
    let is_mine = self.mines.contains(&pos);
    if is_mine {
      self.lost = true;
      Some(OpenResult::Mine)
    } else {
      let mine_count = self.neighboring_mines(pos);
      if mine_count == 0 {
        for neighbor in self.iter_neighbors(pos) {
          if !self.open_fields.contains(&neighbor) {
            self.open(neighbor);
          }
        }
      }
      Some(OpenResult::NoMine(mine_count))
    }
  }

  pub fn toggle_flag(&mut self, pos: Position) {
    if self.lost || self.open_fields.contains(&pos) {
      return;
    }
    if self.flagged_fields.contains(&pos) {
      self.flagged_fields.remove(&pos);
    } else {
      self.flagged_fields.insert(pos);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Minesweeper;
  #[test]
  fn test() {
    let mut ms = Minesweeper::new(10, 10, 5);
    ms.open((2, 2));
    ms.toggle_flag((3, 8));
    println!("{}", ms);
  }
}
