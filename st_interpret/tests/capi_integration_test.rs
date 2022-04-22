/// C API integration tests

#[cfg(feature = "capi")]
mod capi_test {
    use inline_c::assert_c;
    use std::process::Command;
    use std::process::Output;

    #[test]
    /// Basic addition test
    fn capi_add_two() {
        (assert_c! {
            #include <st_interpret.h>
            #include <assert.h>

            int main() {
                assert(4 == lib_function_example_add_clib(2, 2));
                return 0;
            }
        })
        .success();
    }

    #[test]
    /// Example running basic C code
    fn it_works_unit() {
        (assert_c! {
            #include <stdio.h>

            int main() {
                printf("Hello, World!");

                return 0;
            }
        })
        .success()
        .stdout("Hello, World!");
    }

    #[test]
    /// Test the st_program_load function
    fn test_load() {
        let mut result = assert_c! {
            #include <st_interpret.h>
            #include <assert.h>
            #include <stdio.h>

            int main() {
                ProgHandlePointer handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st");
                printf("load was called\n");
                assert(handle.prog_handle != NULL);
                return 0;
            }
        };
        println!(
            "{:?}",
            std::str::from_utf8(&(result.assert().get_output().stdout)).unwrap()
        );
        result.success();
    }

    #[test]
    /// Test the st_program_run function
    fn test_run() {
        (assert_c! {
            #include <st_interpret.h>
            #include <assert.h>

            int main() {
                ProgHandlePointer handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st");
                st_program_run(&handle);
                return 0;
            }
        })
            .success();
    }

    #[test]
    /// Test the get_var function
    fn test_get_var() {
        (assert_c! {
            #include <st_interpret.h>
            #include <assert.h>
            #include <string.h>

            int main() {
                ProgHandlePointer handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st");
                st_program_run(&handle);
                VariableNameInfo * result = get_var(&handle, "a");
                assert(!strcmp(result->name, "a"));
                assert(!strcmp(result->value, "real: 1.2"));
                assert(!strcmp(result->kind, "NORMAL"));
                return 0;
            }
        })
            .success();
    }

    #[test]
    /// Test the get_all_vars function
    fn test_get_all_vars() {
        (assert_c! {
            #include <st_interpret.h>
            #include <assert.h>
            #include <string.h>

            int main() {
                ProgHandlePointer handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st");
                st_program_run(&handle);
                VariableNameInfo * results = get_all_vars(&handle);
                VariableNameInfo * current = results;
                while (current != NULL) {
                    if (!strcmp(current->name, "a")) {
                        assert(!strcmp(current->value, "real: 1.2"));
                    }
                    else if (!strcmp(current->name, "b")) {
                        assert(!strcmp(current->value, "int: 5"));
                    }
                    else if (!strcmp(current->name, "c")) {
                        assert(!strcmp(current->value, "bool: true"));
                    }
                    else {
                        assert(false);
                    }
                    current = current->next;
                }
                return 0;
            }
        })
            .success();
    }

    #[test]
    /// Test the update_var function
    fn test_update_var() {
        (assert_c! {
            #include <st_interpret.h>
            #include <assert.h>
            #include <string.h>

            int main() {
                ProgHandlePointer handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st");
                st_program_run(&handle);
                update_var(&handle, "a", "real", "2.1");
                VariableNameInfo * result = get_var(&handle, "a");
                assert(!strcmp(result->name, "a"));
                assert(!strcmp(result->value, "real: 2.1"));
                return 0;
            }
        })
            .success();
    }

    #[test]
    /// Test the add_var function
    fn test_add_var() {
        let mut result = assert_c! {
            #include <st_interpret.h>
            #include <assert.h>
            #include <string.h>
            #include <stdio.h>

            int main() {
                printf("starting test\n");
                ProgHandlePointer handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st");
                printf("load was called\n");
                st_program_run(&handle);
                printf("run was called\n");
                add_var(&handle, "d", "INPUT", "int", "1");
                printf("add variable was called\n");
                VariableNameInfo * result = get_var(&handle, "d");
                printf("get variable was called\n");
                assert(!strcmp(result->name, "d"));
                assert(!strcmp(result->value, "int: 1"));
                assert(!strcmp(result->kind, "INPUT"));
                return 0;
            }
        };
        println!(
            "{:?}",
            std::str::from_utf8(&(result.assert().get_output().stdout)).unwrap()
        );
        result.success();
    }
}
