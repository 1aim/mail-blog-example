use rand::random;

use mail::{
    // Note that that context is the concrete context type
    // returned by `simple_context::new`, not the context trait.
    default_impl::simple_context::{self, Context}
};

pub fn partial_random_context() -> Context {
    // This should be retrieved from a configuration e.g. passed in
    // as a command line argument
    let domain = "mail.crate.example.com".parse().unwrap();

    // Generate use a random 64bit hex string as unique part, while this is not
    // perfect it's good enough for the example. The parse is for converting it
    // into a soft-ascii-string)
    let unique_part = format!("{:x}", random::<u64>()).parse().unwrap();

    // This can only fail if:
    // - The domain name has some problems with `"Puny"` encoding (only matters for non us-ascii domain names).
    // - There is no current working directory (e.g. because the dir the program was started in was deleted since then).
    simple_context::new(domain, unique_part).unwrap()
}
