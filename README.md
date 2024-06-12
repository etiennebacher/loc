
<!-- README.md is generated from README.Rmd. Please edit that file -->

# loc

<!-- badges: start -->
<!-- badges: end -->

**WIP**

`loc` provides a single function, `count_loc()`, whose purpose is to
count lines of code. You can provide specific paths to include or
exclude, consider hidden files or not, and you have access to a [long
list of
languages](https://github.com/XAMPPRocky/tokei?tab=readme-ov-file#supported-languages).

Under the hood, `loc` uses the
[`tokei`](https://github.com/XAMPPRocky/tokei) Rust crate.

## Installation

TODO

## Example

Here’s what was needed to make this package:

``` r
library(loc)

count_loc(languages = c("R", "Rust"))
#> 
#> Language: R 
#> ===================
#> Lines of code: 102 
#> Blank lines:   25 
#> Comments:      39 
#> Accurate:      Yes
#> 
#> Language: Rust 
#> ===================
#> Lines of code: 254 
#> Blank lines:   10 
#> Comments:      29 
#> Accurate:      Yes
```
