#' @keywords internal
"_PACKAGE"

#' @noRd
.onUnload <- function(libpath) {
  library.dynam.unload("pnglitchr", libpath)
}

#' Create a glitched PNG image
#'
#' Creates glitched PNG image data.
#'
#' @details
#' The following functions are available:
#' * `count_scanlines()`: Returns the total number of scan lines in the PNG image.
#' * `glitch_replace()`: Replaces the scan lines randomly.
#' * `glitch_remove()`: Removes filter from the scan lines.
#' * `glitch_transpose()`: Transposes some scan lines from `src` to `dst`.
#' * `glitch_apply()`: Applies a specified filter to the scan lines.
#'
#' @param x A character string specifying the path to a PNG file
#' or a raw vector containing PNG image data.
#' @param times An integer specifying the number of times to copy.
#' @param from,src,dst Scan line index.
#' @param lines Number of scan lines to be updated.
#' @param filter_type Filter type.
#' One of "none", "sub", "up", "average", "paeth".
#' @returns A raw vector containing glitched PNG image data.
#' @rdname pnglitch
#' @name pnglitch
NULL

#' @rdname pnglitch
#' @export
count_scanlines <- function(x) {
  if (!is.raw(x)) {
    x <- readBin(x, "raw", n = file.info(x)$size)
  }
  pgltc_count_scanlines(x)
}

#' @rdname pnglitch
#' @export
glitch_replace <- function(x, times) {
  if (!is.raw(x)) {
    x <- readBin(x, "raw", n = file.info(x)$size)
  }
  pgltc_random_copy(x, times)
}

#' @rdname pnglitch
#' @export
glitch_remove <- function(x, from, lines) {
  if (!is.raw(x)) {
    x <- readBin(x, "raw", n = file.info(x)$size)
  }
  pgltc_remove_filter(x, from, lines)
}

#' @rdname pnglitch
#' @export
glitch_transpose <- function(x, src, dst, lines) {
  if (!is.raw(x)) {
    x <- readBin(x, "raw", n = file.info(x)$size)
  }
  pgltc_transpose(x, src, dst, lines)
}

#' @rdname pnglitch
#' @export
glitch_apply <- function(
  x,
  from,
  lines,
  filter_type = c("none", "sub", "up", "average", "paeth")
) {
  if (!is.raw(x)) {
    x <- readBin(x, "raw", n = file.info(x)$size)
  }
  filter <- match.arg(filter_type)
  value <- switch(filter, none = 0, sub = 1, up = 2, average = 3, paeth = 4)
  pgltc_apply_filter(x, from, lines, value)
}
