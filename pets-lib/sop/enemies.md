# Enemies

## Rust Types

// TODO

## Adding Enemies

- Add a new entry to the `EnemyID` enum in the GDExtension.
- Open the "Walking Enemy" scene and select the animated sprite.
- Add an idle animation and name it "xxx-Idle" where xxx is the ID of the enemy.
- Add another one named "xxx-Run" for the running animation.

### If adding the enemy into a room:

- Open the room scene and add a new Walking Enemy instance.
- Set its `enemy_id` field accordingly.
