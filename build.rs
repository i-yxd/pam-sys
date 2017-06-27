extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn get_pam_functions() -> &'static [&'static str] {
    static PAM_FUNCTIONS: &'static [&str] = &[
        "pam_acct_mgmt",
        "pam_authenticate",
        "pam_chauthtok",
        "pam_close_session",
        "pam_end",
        "pam_fail_delay",
        "pam_get_authtok",
        "pam_get_authtok_noverify",
        "pam_get_authtok_verify",
        "pam_get_data",
        "pam_getenv",
        "pam_getenvlist",
        "pam_get_item",
        "pam_get_user",
        "pam_modutil_audit_write",
        "pam_modutil_drop_priv",
        "pam_modutil_getgrgid",
        "pam_modutil_getgrnam",
        "pam_modutil_getlogin",
        "pam_modutil_getpwnam",
        "pam_modutil_getpwuid",
        "pam_modutil_getspnam",
        "pam_modutil_read",
        "pam_modutil_regain_priv",
        "pam_modutil_sanitize_helper_fds",
        "pam_modutil_user_in_group_nam_gid",
        "pam_modutil_user_in_group_nam_nam",
        "pam_modutil_user_in_group_uid_gid",
        "pam_modutil_user_in_group_uid_nam",
        "pam_modutil_write",
        "pam_open_session",
        "pam_prompt",
        "pam_putenv",
        "pam_setcred",
        "pam_set_data",
        "pam_set_item",
        "pam_start",
        "pam_strerror",
        "pam_syslog",
        "pam_vprompt",
        "pam_vsyslog",
    ];
    PAM_FUNCTIONS
}

fn get_pamc_functions() -> &'static [&'static str] {
    static PAMC_FUNCTIONS: &'static [&str] = &[
        "pamc_converse",
        "pamc_disable",
        "pamc_end",
        "pamc_list_agents",
        "pamc_load",
        "pamc_start",
    ];
    PAMC_FUNCTIONS
}

fn get_pam_misc_functions() -> &'static [&'static str] {
    static PAM_MISC_FUNCTIONS: &'static [&str] = &[
        "misc_conv",
        "pam_binary_handler_fn",
        "pam_binary_handler_free",
        "pam_misc_conv_died",
        "pam_misc_conv_die_line",
        "pam_misc_conv_die_time",
        "pam_misc_conv_warn_line",
        "pam_misc_conv_warn_time",
        "pam_misc_drop_env",
        "pam_misc_paste_env",
        "pam_misc_setenv",
    ];
    PAM_MISC_FUNCTIONS
}

static GET_FUNCTIONS_LIST: &'static [fn () -> &'static [&'static str]] = &[
    get_pam_functions,
    get_pamc_functions,
    get_pam_misc_functions,
];

fn main() {
    println!("cargo:rustc-link-lib=dylib=pam");
    println!("cargo:rustc-link-lib=dylib=pam_misc");
    println!("cargo:rustc-link-lib=dylib=pamc");

    let mut builder = bindgen::Builder::default().header("wrapper.h");

    let mut get_functions_functions = GET_FUNCTIONS_LIST.iter();
    while let Some(get_functions) = get_functions_functions.next() {
        let mut functions = get_functions().iter();
        while let Some(func) = functions.next() {
            builder = builder.whitelisted_function(func);
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
