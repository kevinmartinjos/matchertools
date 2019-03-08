[![Build Status](https://travis-ci.com/lonesword/matchertools.svg?branch=master)](https://travis-ci.com/lonesword/matchertools)

### matchertools

Hosted as a [crate](https://crates.io/crates/matchertools) on crates.io
Exposes an API for the following:

1. The Gale-Shapley algorithm ([stable marriages problem](https://en.wikipedia.org/wiki/Stable_marriage_problem))

    #### Usage

    ```rust
    extern crate matchertools;

    fn main() {
        let mut men_preferences= HashMap::new();
        let mut women_preferences = HashMap::new();

        men_preferences.insert(&"julius", vec![&"cleopatra", &"boudica", &"nefertiti"]);
        men_preferences.insert(&"antony", vec![&"cleopatra", &"nefertiti", &"boudica"]);
        men_preferences.insert(&"vercingetorix", vec![&"boudica", &"nefertiti", &"cleopatra"]);

        women_preferences.insert(&"cleopatra", vec![&"julius", &"antony", &"vercingetorix"]);
        women_preferences.insert(&"boudica", vec![&"vercingetorix", &"antony", &"julius"]);
        women_preferences.insert(&"nefertiti", vec![&"julius", &"vercingetorix", &"antony"]);

        let engaged_man_woman =
                matchertools::gale_shapley(&men_preferences, &women_preferences);
    }

    ```

2. <Yet to be implemented - stable roommates problem, stable residency problem)

### License

Copyright 2019 Kevin Martin Jose

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.