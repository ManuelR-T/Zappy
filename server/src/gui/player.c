/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** player_position
*/

#include <stdlib.h>
#include "client.h"
#include "middlewares.h"
#include "router/route.h"
#include "types/client.h"

static void send_infos(client_t *c, ai_t *ai, size_t nb)
{
    send_client(
        c,
        "ppo %lu %lu %lu %d %d %s\n",
        nb,
        ai->pos.x,
        ai->pos.y,
        ai->dir,
        ai->level,
        ai->team
    );
}

void player_position(client_t *c, command_state_t *com)
{
    long nb = -1;

    if (str_toint(&nb, com->args->data[1]) || (size_t)nb > com->game->ais->size)
        return send_invalid_args(c);
    return send_infos(c, &com->game->ais->data[nb], nb);
}

void player_level(client_t *c, command_state_t *com)
{
    long nb = -1;

    if (str_toint(&nb, com->args->data[1]) || (size_t)nb > com->game->ais->size)
        return send_invalid_args(c);
    return send_client(c, "plv %ld %d", nb, com->game->ais->data[nb].level);
}

static void send_inventory(client_t *c, ai_t *ai, size_t nb)
{
    send_client(
        c,
        "pin %lu %ld %ld %lu %lu %lu %lu %lu %lu %lu\n",
        nb,
        ai->pos.x,
        ai->pos.y,
        ai->inventory.food,
        ai->inventory.linemate,
        ai->inventory.deraumere,
        ai->inventory.sibur,
        ai->inventory.mendiane,
        ai->inventory.phiras,
        ai->inventory.thystame
    );
}

void player_inventory(client_t *c, command_state_t *com)
{
    long nb = -1;

    if (str_toint(&nb, com->args->data[1]) || (size_t)nb > com->game->ais->size)
        return send_invalid_args(c);
    return send_inventory(c, &com->game->ais->data[nb], nb);
}
