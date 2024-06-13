#' Count lines of code
#'
#' @param paths Paths to include.
#' @param excluded Paths to exclude.
#' @param include_hidden Count hidden files and directories? Default is `FALSE`.
#' @param languages Languages to consider. See the list here: <https://github.com/XAMPPRocky/tokei?tab=readme-ov-file#supported-languages>.
#'
#' @return A data.frame with 4 columns: number of blank lines, comments, code,
#' and whether the count is accurate.
#' @export
#'
#' @examples
#' tmp <- tempfile(fileext = ".R")
#' cat("
#' library(loc)
#' # This is a wrong function
#' foo <- function(x) {
#'   x + 1
#' }
#'
#' ", file = tmp)
#' count_loc(tmp)
count_loc <- function(paths = ".", excluded = NULL, include_hidden = FALSE, languages = "R") {
  if (is.null(excluded)) {
    excluded <- paste(sample(letters, 100, TRUE), collapse = "")
  }
  count <- internal_count_loc(
    paths = paths,
    excluded = excluded,
    include_hidden = include_hidden,
    languages = languages
  )
  out <- lapply(seq_along(count), function(x) {
    res <- as.data.frame(count[[x]])
    res[["language"]] <- languages[x]
    res
  })
  out <- Reduce(rbind, out)
  out2 <- cbind(out[, "language", drop = FALSE], out[-ncol(out)])
  class(out2) <- c("loc", class(out2))
  out2
}

#' @export
print.loc <- function(x, ...) {
  lang <- unique(x$language)
  for (i in lang) {
    cat("\nLanguage:", i, "\n")
    cat("===================\n")
    cat("Lines of code:", x[x$language == i, "code"], "\n")
    cat("Blank lines:  ", x[x$language == i, "blanks"], "\n")
    cat("Comments:     ", x[x$language == i, "comments"], "\n")
    cat("Accurate: ", ifelse(
      is.na(x[x$language == i, "inaccurate"]),
      "    NA",
      ifelse(x[x$language == i, "inaccurate"], "    No", "    Yes"))
    )
    cat("\n")
  }
}
