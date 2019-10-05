extern crate rand;

use std::collections::HashMap;
use std::hash::Hash;
use std::vec::Vec;

use self::rand::{Rng, SeedableRng};
use self::rand::prelude::StdRng;

static STATE_MISSING_ERR: &str = "state not in map";

enum FollowupState<T> {
    End,
    State(T),
}

fn add_to_markov_map<'a, T: Eq + Hash>(map: &mut HashMap<&'a T, Vec<FollowupState<&'a T>>>, states: &'a Vec<T>) {
    let start_state = &states[0];
    let _ = map.entry(start_state).or_insert_with(|| Vec::new());
    let mut prev_state = start_state;
    for state in states.iter().skip(1) {
        let _ = map.entry(state).or_insert_with(|| Vec::new());
        map.get_mut(prev_state).expect(STATE_MISSING_ERR).push(FollowupState::State(state));
        prev_state = state;
    }
    map.get_mut(prev_state).expect(STATE_MISSING_ERR).push(FollowupState::End);
}

fn gen_chain_from_start<'a, T: Eq + Hash, R: Rng + ?Sized>(map: &HashMap<&'a T, Vec<FollowupState<&'a T>>>, start_state: &'a T, rng: &mut R) -> Vec<&'a T> {
    let mut result = Vec::new();
    let mut current_state = start_state;
    loop {
        result.push(current_state);
        let f = map.get(current_state).unwrap();
        match f[rng.gen_range(0, f.len())] {
            FollowupState::End => break,
            FollowupState::State(state) => current_state = state,
        }
    }

    result
}

pub fn gen_chain_with_rng<'a, T: Eq + Hash, R: Rng + ?Sized>(states: &'a Vec<T>, rng: &mut R) -> Vec<&'a T> {
    if states.is_empty() {
        return Vec::new();
    }

    let mut map = HashMap::new();
    add_to_markov_map(&mut map, states);

    gen_chain_from_start(&map, &states[0], rng)
}

pub fn gen_chain<T: Eq + Hash>(states: &Vec<T>) -> Vec<&T> {
    gen_chain_with_rng(states, &mut StdRng::from_entropy())
}

pub fn gen_chain_from_many_with_rng<'a, T: Eq + Hash, R: Rng + ?Sized>(states_list: &'a Vec<Vec<T>>, rng: &mut R) -> Vec<&'a T> {
    if states_list.is_empty() {
        return Vec::new();
    }

    let mut map = HashMap::new();
    let mut starting_states = Vec::new();

    for states in states_list.iter() {
        if !states.is_empty() {
            add_to_markov_map(&mut map, states);
            starting_states.push(&states[0]);
        }
    }

    if starting_states.is_empty() {
        return Vec::new();
    }

    gen_chain_from_start(&map, starting_states[rng.gen_range(0, starting_states.len())], rng)
}

pub fn gen_chain_from_many<T: Eq + Hash>(states_list: &Vec<Vec<T>>) -> Vec<&T> {
    gen_chain_from_many_with_rng(states_list, &mut StdRng::from_entropy())
}