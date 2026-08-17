// Practice Exercise: 8-Bit Retro Space Fighter State System
// In Chapter 3, Rust introduces 4 explicit ways to handle integer overflow
// when working with bounded types:
//
// wrapping_*: Wraps around the minimum or maximum value of the type.
//
// saturating_*: Clamps at the boundary (0 or MAX).
//
// overflowing_*: Computes the wrapped value and returns a boolean flag
// indicating if overflow occurred.
//
// checked_*: Returns an Option<T> (Some(val) or None).
//
//
// In this exercise, you will build a status tracking module for an 8-bit
// retro arcade game where all stats are stored using 8-bit unsigned
// integers (u8).
//
// Task Description
// Create four helper functions using an 8-bit unsigned integer (u8) for
// calculations:
//
// apply_damage(health: u8, damage: u8) -> u8
//
// Health cannot drop below 0. Use saturating_sub so health safely clamps to 0
// without causing a runtime panic on massive damage.
//
// rotate_radar(current_degree: u8, rotation: u8) -> u8
//
// Radar angles wrap around a 256-degree wheel (0..=255). Use wrapping_add so
// rotating past 255 smoothly loops back to 0.
//
// charge_energy(current: u8, amount: u8) -> (u8, bool)
//
// Energy charges up to 255. Use overflowing_add to calculate the new energy
// level and detect if an energy discharge flag was triggered by overflowing.
//
// deposit_credits(balance: u8, deposit: u8) -> Option<u8>
//
// Bank capacity is strictly capped at 255. Use checked_add to return
// Some(new_balance) if the transaction fits, or None if it exceeds total vault
// capacity.


fn apply_damage(health: u8, damage: u8) -> u8 {
    health.saturating_sub(damage)
}

fn rotate_radar(current_degree: u8, rotation: u8) -> u8 {
    current_degree.wrapping_add(rotation)
}

fn charge_energy(current: u8, amount: u8) -> (u8, bool) {
    current.overflowing_add(amount)
}

fn deposit_credits(balance: u8, deposit: u8) -> Option<u8> {
    balance.checked_add(deposit)
}

fn main() {
    // Test 1: Saturating
    let health = 50;
    let heavy_hit = 80;
    println!("Health after hit: {}", apply_damage(health, heavy_hit));

    // Test 2: Wrapping
    let heading = 200;
    let turn = 100;
    println!("New radar position: {}", rotate_radar(heading, turn));

    // Test 3: Overflowing
    let core_power = 200;
    let boost = 70;
    let (new_power, discharged) = charge_energy(core_power, boost);
    println!("Power: {}, Supernova Triggered: {}", new_power, discharged);

    // Test 4: Checked
    let bank = 230;
    let income = 50;
    match deposit_credits(bank, income) {
        Some(new_balance) => println!("New bank balance: {}", new_balance),
        None => println!("Transaction failed: Vault capacity exceeded!"),
    }
}


// Expected Output
//
// Health after hit: 0
// New radar position: 44
// Power: 14, Supernova Triggered: true
// Transaction failed: Vault capacity exceeded!