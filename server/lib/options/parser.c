/*
** EPITECH PROJECT, 2024
** zappy
** File description:
** parser
*/

#include <stdlib.h>

#include "logger.h"
#include "options/converter.h"
#include "options/option.h"
#include "options/parser.h"

static fconverter_t find_converter(enum arg_type type)
{
    for (size_t i = 0; CONVERTERS[i].converter != NULL; i++) {
        if (CONVERTERS[i].type == type)
            return CONVERTERS[i].converter;
    }
    return NULL;
}

static option_t *get_option(char *const arg, struct options *const opts)
{
    for (size_t i = 0; i < opts->size; i++) {
        if (!strcmp(opts->data[i].identifier, arg)) {
            logs(DEBUG, "Found option: %s\n", arg);
            return &opts->data[i];
        }
    }
    logs(ERROR_LEVEL, "Unknown option: %s\n", arg);
    return NULL;
}

static bool fill_argument(
    fconverter_t *f,
    option_t *opts,
    parser_t *p,
    struct args *lst
)
{
    argument_t argument = {0};

    argument.option = opts;
    if ((*f)(&argument, p)) {
        logs(
            ERROR_LEVEL,
            "Invalid argument for option %s: %s\n",
            argument.option->identifier,
            p->args[p->idx]
        );
        return true;
    }
    vec_pushback_args(lst, argument);
    return false;
}

static bool parse_argument(struct args *lst, parser_t *p, option_t *opts)
{
    fconverter_t f = find_converter(opts->type);

    if (f == NULL) {
        logs(
            ERROR_LEVEL,
            "Couldn't find a converter for the argument %s\n",
            opts->identifier
        );
        return true;
    }
    return fill_argument(&f, opts, p, lst);
}

static bool parse_inner(parser_t *parser, struct args *lst)
{
    option_t *opt = NULL;

    opt = get_option(parser->args[parser->idx], parser->options);
    if (opt == NULL)
        return true;
    parser->idx += 1;
    if (opt->type != BOOL && parser->idx >= parser->args_size) {
        logs(ERROR_LEVEL, "Missing value for argument: %s\n", opt->identifier);
        return true;
    }
    return parse_argument(lst, parser, opt);
}

void free_args(struct args *ag)
{
    for (size_t i = 0; i < ag->size; i++) {
        if (ag->data[i].option->type == STRING_LIST)
            vec_free_vector_str_t(ag->data[i].value.string_list, str_free);
        if (ag->data[i].option->type == STRING)
            str_free(ag->data[i].value.string);
    }
    vec_destroy_args(ag);
}

struct args *parse(char **args, size_t count, struct options *opts)
{
    struct args *lst = vec_create_args(count);
    parser_t parser = {opts, args, count, 1};
    bool error = false;

    for (; parser.idx < parser.args_size; parser.idx++) {
        error |= parse_inner(&parser, lst);
    }
    set_defaults(opts, lst);
    if (error || !check_required(opts, lst)) {
        free_args(lst);
        return NULL;
    }
    return lst;
}
