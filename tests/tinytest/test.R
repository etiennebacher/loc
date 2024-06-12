source("helpers.R")

is_interactive_call <- is.null(sys.call())
if (!is_interactive_call) {
  base <- getwd()
} else {
  base <- "./tests/tinytest"
}

# basic example -----------------------------------------------

out <- count_loc(paths = file.path(base, "test_examples"))

expect_equal(
  out,
  data.frame(
    language = "R",
    blanks = 5L,
    comments = 4L,
    code = 8L,
    inaccurate = FALSE
  ) |>
    structure(class = c("loc", "data.frame"))
)

# languages -----------------------------------------------

out <- count_loc(
  paths = file.path(base, "test_examples"),
  languages = c("R", "Rust")
)

expect_equal(
  out,
  data.frame(
    language = c("R", "Rust"),
    blanks = c(5L, 3L),
    comments = c(4L, 1L),
    code = c(8L, 5L),
    inaccurate = c(FALSE, FALSE)
  ) |>
    structure(class = c("loc", "data.frame"))
)


# paths -----------------------------------------------

out <- count_loc(
  paths = c(
    file.path(base, "test_examples/file1.R"),
    file.path(base, "test_examples/file1.rs")
  ),
  languages = c("R", "Rust")
)

expect_equal(
  out,
  data.frame(
    language = c("R", "Rust"),
    blanks = c(3L, 3L),
    comments = 2:1,
    code = 4:5,
    inaccurate = c(FALSE, FALSE)
  ) |>
    structure(class = c("loc", "data.frame"))
)
