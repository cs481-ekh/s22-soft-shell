#include "st_interpret.h"
#include <string.h>
#include <stdio.h>

int main() {
    ProgHandlePointer handle = st_program_load("test_inputs/st_subset_1/01_mixed.st");
    st_program_run(&handle);
    VariableNameInfo * result = get_var(&handle, "a");
    printf("%s", result->name);
    printf("%s", result->value);
    printf("%s", result->kind);
    return 0;
}