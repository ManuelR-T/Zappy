/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** parse_unset
*/

#include <string.h>

#include "client.h"
#include "utils.h"

int unset_entrypoint(char const *line, client_t *c, game_t *game)
{
    if (strcmp(line, "GRAPHIC") == 0) {
        logger_info("Client %d is a GUI\n", c->fd);
        c->type = GUI;
        c->entrypoint = &gui_entrypoint;
        return 0;
    }
    if (game->teams == NULL) {
        logger_warn("No teams set\n");
        return 1;
    }
    for (int i = 0; game->teams[i]; i++) {
        if (strcmp(line, game->teams[i]) == 0) {
            logger_info("Client %d is an AI\n", c->fd);
            c->type = AI;
            c->entrypoint = &ai_entrypoint;
            return 0;
        }
    }
    return 1;
}
