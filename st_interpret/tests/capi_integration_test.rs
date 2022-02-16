// C integration tests here

#[cfg(feature = "capi")]
mod capi_test {
    use inline_c::assert_c;

    #[test]
    fn capi_add_two() {
        (assert_c! {
            #include <st_interpret.h>
            #include <assert.h>

            int main() {
                assert(4 == lib_function_example_add_clib(2, 2));
                return 0;
            }
        }).success();
    }

    #[test]
    fn it_works_unit() {
        (assert_c! {
           #include <stdio.h>

        int main() {
            printf("Hello, World!");

            return 0;
        }
    }).success().stdout("Hello, World!");
    }
}