use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Location {
  pub name: String,
  pub count: u64,
}

impl TryFrom<&str> for Location {
  type Error = String;
  fn try_from(s: &str) -> Result<Location, Self::Error> {
    let s = s.trim();
    let split: Vec<&str> = s.split(' ').collect();

    let count = split
      .first()
      .map_or(Err("hei".to_string()), |s| s.parse::<u64>().map_err(|_| "hei".to_string()))?;

    let name = split.get(1).map_or(Err("hei2".to_string()), |s| Ok(s.to_string()))?;

    Ok(Location { name, count })
  }
}

#[cfg(test)]
mod test {
  use std::convert::TryInto;

  use crate::read::Location;

  #[test]
  fn test_parse_counts() {
    let counts = include_str!("../counts");

    let locs: Result<Vec<Location>, String> = counts.lines().map(|l| l.try_into()).collect();

    assert!(locs.is_ok());
    dbg!(locs.unwrap().clone());
  }
}
