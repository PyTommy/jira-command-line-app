
use std::io::Cursor;
use skim::prelude::*;

pub fn fuzzy_select_one(options: Vec<String>) -> Option<usize>  {
  // https://github.com/lotabout/skim#use-as-a-library
  let skim_options = SkimOptionsBuilder::default()
      .build()
      .unwrap();
  let input = options.join("\n").to_string();
  let item_reader = SkimItemReader::default();
  let items = item_reader.of_bufread(Cursor::new(input));
  let selected_items = Skim::run_with(&skim_options, Some(items))
      .map(|out| out.selected_items)
      .unwrap_or_else(|| Vec::new());
  let index = match selected_items.first() {
      Some(item) => options.iter().position(|el| el == &item.output()),
      None =>  None,
  };
  return index;
}

pub fn fuzzy_select_multi(options: Vec<String>) -> Vec<usize> {
    // https://github.com/lotabout/skim#use-as-a-library
    let skim_options = SkimOptionsBuilder::default()
        .multi(true)
        .build()
        .unwrap();
    let input = options.join("\n").to_string();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let selected_items = Skim::run_with(&skim_options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let indexes: Vec<usize> = selected_items
      .iter()
      .filter_map(|item| options.iter().position(|el| el == &item.output()))
      .collect();
    
    return indexes;
  }