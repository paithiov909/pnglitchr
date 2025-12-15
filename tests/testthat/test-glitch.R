skip_on_cran()
skip_on_ci()

test_that("glitch produces an image anyway", {
  fp <- system.file("images/barplot.png", package = "pnglitchr")
  nr <-
    glitch_remove(fp, 0, 1) |>
    glitch_replace(10) |>
    glitch_transpose(1, 10, 6) |>
    glitch_apply(0, count_scanlines(fp), "sub") |>
    fastpng::read_png(type = "nativeraster")

  expect_s3_class(nr, "nativeRaster")
})
