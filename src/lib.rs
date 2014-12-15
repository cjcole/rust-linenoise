#![crate_type="lib"]

extern crate libc;

use std::c_str;
pub mod ffi;


pub type Completions = ffi::Struct_linenoiseCompletions;
type Callback = ffi::linenoiseCompletionCallback;

pub type CompletionCallback = fn(&str) -> Vec<&str>;
static mut USER_COMPLETION: Option<CompletionCallback> = None;



/// Sets the callback when tab is pressed
pub fn set_callback(rust_cb: CompletionCallback ) {
    unsafe {
        USER_COMPLETION = Some(rust_cb);
        let ca = internal_callback as *mut _;
        ffi::linenoiseSetCompletionCallback(ca);
    }
}

/// Shows the prompt with your prompt as prefix
/// Retuns the typed string or None is nothing or EOF
pub fn input(prompt: &str) -> Option<String> {
    let cprompt = prompt.to_c_str();
    let mut retval:Option<String>;

    unsafe {
        let cs = cprompt.as_ptr();
        let rret = ffi::linenoise(cs);

        if rret != 0 as *mut i8 {
            let ptr = rret as *const i8;
            let ret = c_str::CString::new(ptr, true);
            let cast = ret.as_str();

            cast.map(|x| x.to_string())
        } else {
            None
        }

    }
}

/// Add this string to the history
pub fn history_add(line: &str) -> i32 {
    let cs = line.to_c_str().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryAdd(cs);
    }
    ret
}

/// Set max length history
pub fn history_set_max_len(len: i32) -> i32 {
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySetMaxLen(len);
    }
    ret
}

/// Save the history on disk
pub fn history_save(file: &str) -> i32 {
    let fname = file.to_c_str().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySave(fname);
    }
    ret
}

/// Load the history on disk
pub fn history_load(file: &str) -> i32 {
    let fname = file.to_c_str().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryLoad(fname);
    }
    ret
}

///Clears the screen
pub fn clear_screen() {
    unsafe {
        ffi::linenoiseClearScreen();
    }
}

pub fn set_multiline(ml: i32) {
    unsafe {
        ffi::linenoiseSetMultiLine(ml);
    }
}

pub fn print_key_codes() {
    unsafe {
        ffi::linenoisePrintKeyCodes();
    }
}


/// Add a completion to the current list of completions.
pub fn add_completion(c: *mut Completions, s: &str) {
    unsafe {
        ffi::linenoiseAddCompletion(c, s.to_c_str().as_ptr());
    }
}

fn internal_callback(cs: *mut libc::c_char, lc:*mut Completions ) {
    unsafe {
        (*lc).len = 0;
    }
    let input: Option<&str>;
    let ccurrent_input: std::c_str::CString;

    unsafe {
        ccurrent_input = c_str::CString::new(cs as *const _, false);
        input = ccurrent_input.as_str();
    }
    for completable in input.iter() {
        unsafe {
            for external_callback in USER_COMPLETION.iter() {
                let ret = (*external_callback)(*completable);
                for x in ret.iter() {
                    add_completion(lc, *x);
                }
            }
        }
    }
}
