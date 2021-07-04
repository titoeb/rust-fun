use std::io::stdin;

enum LockState {
    Locked,
    Failed,
    Unlocked,
}

fn main() {
    let code = String::from("1234");
    let mut state = LockState::Locked;
    let mut entry = String::new();

    loop {
        match state {
            LockState::Locked => {
                // The Lock is locked. Now the user can enter a code
                // in order to unlock it.
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => {
                        continue;
                    }
                }
                if entry == code {
                    state = LockState::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    // "1234".startswith("125")
                    state = LockState::Failed;
                }
            }
            LockState::Failed => {
                println!("Failed");
                entry.clear();
                state = LockState::Locked;
                continue;
            }
            LockState::Unlocked => {
                println!("Unlocked");
                return;
            }
        }
    }
}
