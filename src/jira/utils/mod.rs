use crate::fuzzy::fuzzy_select_multi;

use super::api::Issue;
use super::super::fuzzy::fuzzy_select_one;

pub fn select_issue(issues: &Vec<Issue>) -> &Issue {
  let titles = issues
    .into_iter()
    .map(|issue| format!("[{}] {}", issue.key, issue.fields.summary))
    .collect::<Vec<String>>();
  let index = fuzzy_select_one(titles).unwrap();
  return &issues[index];
}

pub fn select_issues(issues: &Vec<Issue>) -> Vec<&Issue> {
  let titles = issues
    .into_iter()
    .map(|issue| format!("[{}] {}", issue.key, issue.fields.summary))
    .collect::<Vec<String>>();
  let indexes = fuzzy_select_multi(titles);
  return indexes
    .into_iter()
    .filter_map(|i| Some(&issues[i]))
    .collect();
}