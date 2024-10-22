/*
** EPITECH PROJECT, 2024
** zappy
** File description:
** loader.spec
*/

#include <criterion/criterion.h>

#include "loader.h"

Test(loader, success_case)
{
    lib_t lib = open_dhl("tests/server/base.so");

    cr_assert_not_null(lib.handle);
    cr_assert_not_null(lib.loop);
    cr_assert_not_null(lib.init);
    cr_assert_not_null(lib.destroy);
    close_dhl(&lib);
}

Test(loader, fail_case)
{
    lib_t lib = open_dhl("lou.so");

    cr_assert_null(lib.handle);
    cr_assert_null(lib.loop);
    cr_assert_null(lib.init);
    cr_assert_null(lib.destroy);
}

Test(loader, missing_function)
{
    lib_t lib = open_dhl("tests/server/missing_function.so");

    cr_assert_null(lib.handle);
    cr_assert_null(lib.loop);
    cr_assert_null(lib.init);
    cr_assert_null(lib.destroy);
}
