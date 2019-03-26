
# mail-blog-example

Repository containing examples for the `mail` crate used and
refered to in the blog post(s) around the `mail` crate.

A "crate" describes a library written in the `rust` programming
language managed by the `cargo` tool, so this are basically
examples for a library to automated create mails in `rust`.
(Not that the library is focused on the mail part not the
 hmtl you might want to display with the mail.).


Note that this are not the shortest examples possible, but
instead more usefull examples which e.g. use templates to
create the mail bodies (using `handlebars` as template engine).

There are both examples for sending the mail to a Mail
Submission Agent (`MSA`) and printing them to stdout.
The examples also demonstrate a way how you could make
you templates a bit more type-safe, if you want to.