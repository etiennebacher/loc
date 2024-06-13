
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

count_loc(paths = c("R", "src"), languages = c("R", "Rust"))
#> 
#> Language: R 
#> ===================
#> Lines of code: 37 
#> Blank lines:   7 
#> Comments:      32 
#> Accurate:      Yes
#> 
#> Language: Rust 
#> ===================
#> Lines of code: 249 
#> Blank lines:   7 
#> Comments:      28 
#> Accurate:      Yes
```
