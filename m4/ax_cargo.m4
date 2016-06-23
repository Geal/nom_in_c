dnl Copyright (c) 2016 Marc-Antoine Perennou <Marc-Antoine@Perennou.com>

dnl AX_CARGO([value_if_not_found], [path])
dnl Detect the presence of cargo and fill the CARGO variable with its value
AC_DEFUN([AX_CARGO], [AC_CHECK_TOOL([CARGO], [cargo], [$1], [$2])])

dnl AX_REQUIRE_CARGO([path])
dnl Detect the presence of cargo and fill the CARGO variable with its value, fail if not found
AC_DEFUN([AX_REQUIRE_CARGO], [
    AX_CARGO([no], [$1])
    AS_IF([test x${CARGO} = xno], [AC_MSG_ERROR([cargo is required])])
])
