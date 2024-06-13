/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** main
*/

#include <signal.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/server.h"
#include "logger.h"
#include "macros.h"
#include "options/option.h"
#include "options/parser.h"
#include "args_info.h"
#include "options_def.h"

static void sig(int signum)
{
    (void)signum;
}

static void fill_infos(args_infos_t *infos, struct args *const ag)
{
    infos->clients_nb = get_arg(ag, "-c").unsigned_number;
    infos->port = get_arg(ag, "-p").unsigned_number;
    infos->width = get_arg(ag, "-x").unsigned_number;
    infos->height = get_arg(ag, "-y").unsigned_number;
    infos->freq = get_arg(ag, "-f").unsigned_number;
    infos->names = get_arg(ag, "-n").string_list;
    infos->level = set_log_level_from_str(get_arg(ag, "-l").string->data);
}

static struct options *create_opts(void)
{
    struct options *opts = vec_create_options(7);

    for (int i = 0; OPTIONS[i].identifier != NULL; i++) {
        vec_pushback_options(opts, OPTIONS[i]);
    }
    return opts;
}

int main(int ac, char **av)
{
    args_infos_t args = {0};
    struct args *ag = NULL;
    struct options *opts = create_opts();

    srand(time(NULL));
    ag = parse(av, ac, opts);
    if (ag == NULL) {
        dprintf(2, "Error parsing arguments\n");
        return EPI_ERROR;
    }
    fill_infos(&args, ag);
    signal(SIGINT, &sig);
    if (loop_server(&args) == ERROR)
        return EPI_ERROR;
    logs(INFO, "Server stopped\n");
    return SUCCESS;
}
