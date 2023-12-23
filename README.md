## What's wrong

Running `cargo build` will compile just fine, but running `cargo build -p macros` will fail with a legitimate error (missing items because of a missing feature on `syn`).

## What's happening

This is due to the `consumer` crate having a dependency (`debugify`) that happens to enable the `syn` features that `macros` would need. It seems that `cargo build` resolvs features for all the crates at once, while `cargo build -p` only resolves the features for that single package.

I'm not super familiar with the concept of lockfiles, but I've noticed that `debugify` commits the lockfile into source control. I have no idea if that's OK or not.