![](/assets/menu/splash.png)
# brainstorm: bird invaders
best game ever*

### Development
#### Setup
Linux requires Bevy's [Linux dependencies](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md) and in addition to [`mold` and `clang`](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md) to be installed.


#### Levels
Levels should be placed within `assets/levels`.

To load a specific level use the `--level` argument to provide the desired asset path.
To skip normal startup and start the level immediately, use `--initial-state loading`. For example:
```sh
$ cargo run -- --level levels/test.ron --initial-state loading
```
