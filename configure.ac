## Copyright (c) 2016, Marc-Antoine Perennou <Marc-Antoine@Perennou.com>

m4_define([nom_in_c_version],   [0.1])

m4_define([autoconf_required],  [2.69])
m4_define([automake_required],  [1.15])
m4_define([libtool_required],   [2.2.6])

AC_PREREQ(autoconf_required)
AC_INIT([nom_in_c], [nom_in_c_version], [https://github.com/Geal/nom_in_c/issues])

AC_SUBST([PACKAGE_NAME])
AC_SUBST([PACKAGE_VERSION])

AC_CONFIG_HEADERS([config.h])
AC_CONFIG_SRCDIR([src/])
AC_CONFIG_MACRO_DIR([m4])
AC_CONFIG_AUX_DIR([build-aux])
AC_PREFIX_DEFAULT([/usr])

AC_USE_SYSTEM_EXTENSIONS

AM_PROG_AR
LT_PREREQ(libtool_required)
LT_INIT([pic-only dlopen])
LT_LIB_DLLOAD
LT_LIB_M

AX_PTHREAD

AM_INIT_AUTOMAKE(automake_required [subdir-objects foreign no-dist-gzip dist-xz tar-ustar -Wall])
AM_MAINTAINER_MODE([enable])
AM_SILENT_RULES([yes])

AC_PROG_AWK
AC_PROG_SED
AC_PROG_MKDIR_P
AC_PROG_LN_S
AC_PROG_INSTALL

AC_PROG_MAKE_SET

AC_PROG_CPP
AC_PROG_CC
AC_PROG_CC_C99
AM_PROG_CC_C_O

AC_C_INLINE

AC_CONFIG_FILES([
    Makefile
])
AC_OUTPUT
AC_MSG_RESULT([
    ${PACKAGE_NAME} ${VERSION}
    ===========

    prefix:                 ${prefix}
    exec_prefix:            ${exec_prefix}
    sysconfdir:             ${sysconfdir}
    libdir:                 ${libdir}
    libexecdir:             ${libexecdir}
    includedir:             ${includedir}
    datarootdir:            ${datarootdir}
    datadir:                ${datadir}

    compiler:               ${CC}
    cflags:                 ${CFLAGS}
    ldflags:                ${LDFLAGS}
])
