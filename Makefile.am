## Copyright (c) 2016, Marc-Antoine Perennou <Marc-Antoine@Perennou.com>

ACLOCAL_AMFLAGS  = -I m4 ${ACLOCAL_FLAGS}
AM_MAKEFLAGS = --no-print-directory
AM_CPPFLAGS = -I$(srcdir)/embed_capi

EXTRA_DIST =                       \
	autogen.sh                 \
	README.md                  \
	embed/Cargo.toml           \
	embed/src/lib.rs           \
	embed_capi/Cargo.toml      \
	embed_capi/src/lib.rs      \
	embed_capi/include/embed.h \
	$(NULL)

CLEANFILES =

bin_PROGRAMS = main
noinst_LTLIBRARIES =

CARGO_MODE = release

main_SOURCES = src/main.c
main_DEPENDENCIES = embed_capi
main_CFLAGS = $(PTHREAD_CFLAGS)
main_LDFLAGS = -Lembed_capi/target/$(CARGO_MODE) -lembed_capi $(LIBADD_DL) $(PTHREAD_LIBS) $(LIBM)

CARGO_ARGS = --$(CARGO_MODE)
CARGO_CRATES = embed embed_capi

embed_capi_DEPS = embed

include Makefile.cargo

.PHONY: embed embed_capi
