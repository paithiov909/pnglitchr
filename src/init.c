
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP savvy_pgltc_apply_filter__impl(SEXP c_arg__bytes, SEXP c_arg__filter_type, SEXP c_arg__from, SEXP c_arg__lines) {
    SEXP res = savvy_pgltc_apply_filter__ffi(c_arg__bytes, c_arg__filter_type, c_arg__from, c_arg__lines);
    return handle_result(res);
}

SEXP savvy_pgltc_count_scanlines__impl(SEXP c_arg__bytes) {
    SEXP res = savvy_pgltc_count_scanlines__ffi(c_arg__bytes);
    return handle_result(res);
}

SEXP savvy_pgltc_random_copy__impl(SEXP c_arg__bytes, SEXP c_arg__times) {
    SEXP res = savvy_pgltc_random_copy__ffi(c_arg__bytes, c_arg__times);
    return handle_result(res);
}

SEXP savvy_pgltc_remove_filter__impl(SEXP c_arg__bytes, SEXP c_arg__from, SEXP c_arg__lines) {
    SEXP res = savvy_pgltc_remove_filter__ffi(c_arg__bytes, c_arg__from, c_arg__lines);
    return handle_result(res);
}

SEXP savvy_pgltc_transpose__impl(SEXP c_arg__bytes, SEXP c_arg__src, SEXP c_arg__dst, SEXP c_arg__lines) {
    SEXP res = savvy_pgltc_transpose__ffi(c_arg__bytes, c_arg__src, c_arg__dst, c_arg__lines);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_pgltc_apply_filter__impl", (DL_FUNC) &savvy_pgltc_apply_filter__impl, 4},
    {"savvy_pgltc_count_scanlines__impl", (DL_FUNC) &savvy_pgltc_count_scanlines__impl, 1},
    {"savvy_pgltc_random_copy__impl", (DL_FUNC) &savvy_pgltc_random_copy__impl, 2},
    {"savvy_pgltc_remove_filter__impl", (DL_FUNC) &savvy_pgltc_remove_filter__impl, 3},
    {"savvy_pgltc_transpose__impl", (DL_FUNC) &savvy_pgltc_transpose__impl, 4},
    {NULL, NULL, 0}
};

void R_init_pnglitchr(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
