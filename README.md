![](/assets/menu/splash.png)
# brainstorm: bird invaders
best game ever*

<img width="1628" height="966" alt="image" src="https://github.com/user-attachments/assets/7bc29de6-57d9-468c-a8af-bea8c618897a" />

### Development
#### Setup
Linux requires Bevy's [Linux dependencies](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md) and in addition to [`mold` and `clang`](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md) to be installed.


#### Levels
Levels should be placed within `assets/levels`.

To load a specific level use the `--level` argument to provide the desired asset path.
To skip normal startup and start the level immediately, use `--initial-state game`. For example:
```sh
$ cargo run -- --level levels/test.ron --initial-state game
```
