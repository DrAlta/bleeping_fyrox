fn spread_rumor(source: &Person, target: &Person, rumor: &Rumor, game_state: &GameState) {
    // Check if the rumor has already been heard by the target
    if target.rumors.contains(rumor) {
        return;
    }

    // Calculate the probability of the rumor being spread
    let spread_probability = rumor.spread_chance * (1.0 - target.reputation) * source.reputation;

    // Generate a random number between 0 and 1
    let random_number = rand::random::<f32>();

    // Check if the rumor is spread to the target
    if random_number < spread_probability {
        // Add the rumor to the target's set of heard rumors
        target.rumors.insert(rumor.clone());

        // Update the target's reputation
        target.reputation += rumor.spread_chance * rumor.reputation_impact;

        // Update the target's opinion based on the rumor's impact on opinion
        target.opinion += rumor.opinion_impact;

        // Decay the rumor over time
        rumor.spread_chance *= rumor.decay_rate;

        // Spread the rumor to the target's friends
        for friend in target.friends.iter() {
            spread_rumor(target, friend, rumor, game_state);

            // Update the friend's opinion of the target based on opinion dynamics model
            let opinion_change = (target.opinion - friend.opinion) * game_state.opinion_spread_rate;
            friend.opinion += opinion_change * rumor.spread_chance * rumor.reputation_impact;
        }
    }
}
