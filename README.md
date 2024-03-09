# Amethyst Colorizer

A small utility website that automatically converts amethyst into their dyeable variants. Made for Astral SMP.

Amethyst Colorizer is a work-in-progress. Expect many issues and breakages.

### Usage

#### Through the Browser

The primary site is hosted at [https://amethyst-colorizer.shuttle.rs](https://amethyst-colorizer.shuttle.rs).

If you want to self-host, you can use the [shuttle command-line tool](https://docs.shuttle.rs/getting-started/installation).

#### Through the Terminal

There is also a command-line tool, which can be built locally by cloning this repository.

```sh
amethyst-colorizer [input] [..args]
```

#### As a Library

The main library of Amethyst Colorizer is located within `src/lib.rs`. To use this in your own projects, you may add
the following to your Cargo manifest.

```toml
[dependencies.amethyst-colorizer]
git = "https://github.com/Jaxydog/amethyst-colorizer.git"
default-features = false
```

Alternatively, you may run the following command:

```sh
cargo add amethyst-colorizer --git https://github.com/Jaxydog/amethyst-colorizer.git --no-default-features
```

### License

Amethyst Colorizer is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General 
Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any
later version.

Amethyst Colorizer is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied 
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License along with Amethyst Colorizer (located within
[LICENSE](./LICENSE)). If not, see <[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
